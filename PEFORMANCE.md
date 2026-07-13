# Performance — Orbis Font Engine

## Diagnóstico atual

O loop de renderização redesenha **todos os glifos a cada frame**. Para cada caractere (36 no editor), são criadas **12 partes** (`OrbParts`), cada uma gerando uma chamada individual a `stroke_path` ou `fill_path` do femtovg. Isso resulta em **~432 draw calls por frame**, além de recalcular geometria que não muda.

As chamadas de draw dentro de `OrbFont::draw()` são **duplicadas** — o `FontMask::initialize()` redesenha os mesmos paths logo em seguida para aplicar highlight.

---

## Estratégias de otimização

### 1. Remover draws duplicados (impacto alto, esforço baixo)

Cada método `draw_*` (ex: `draw_circle_base`, `draw_left_lag`) chama `stroke_path`/`fill_path` internamente. Porém, o `FontMask` já itera sobre os paths retornados e desenha novamente. Remover as chamadas de draw dentro dos métodos `draw_*` corta as draw calls pela metade.

**Antes:**
```rust
pub fn draw_circle_base(&mut self) -> (Path, Paint, FontFillKind) {
    let mut base_circle = Path::new();
    base_circle.arc(cx, cy, self.base_circle_r, 0.0, PI * 2.0, Solidity::Solid);
    self.canvas.stroke_path(&base_circle, &self.default_paint); // redundante
    (base_circle, self.default_paint.clone(), FontFillKind::Stroke)
}
```

**Depois:**
```rust
pub fn build_circle_base(&self) -> (Path, Paint, FontFillKind) {
    let mut base_circle = Path::new();
    base_circle.arc(cx, cy, self.base_circle_r, 0.0, PI * 2.0, Solidity::Solid);
    (base_circle, self.default_paint.clone(), FontFillKind::Stroke)
}
```

### 2. Batch de paths por tipo e cor (impacto alto, esforço médio)

Agrupar todos os paths que compartilham o mesmo `Paint` e `FontFillKind` em um único `Path` composto, reduzindo centenas de draw calls para poucas.

```rust
let mut all_strokes = Path::new();
let mut all_fills = Path::new();

for (path, _paint, kind) in &path_list {
    match kind {
        FontFillKind::Stroke => { /* append arcs/segments ao all_strokes */ }
        FontFillKind::Path   => { /* append ao all_fills */ }
        _ => {}
    }
}

canvas.stroke_path(&all_strokes, &stroke_paint);
canvas.fill_path(&all_fills, &fill_paint);
```

### 3. Cache de geometria (impacto médio, esforço baixo)

Os `Path` dos glifos dependem apenas de tamanho e posição — não mudam entre frames. Calcular uma vez e reutilizar:

```rust
struct CachedGlyph {
    paths: Vec<(Path, Paint, FontFillKind)>,
}

// Calcular no init ou no primeiro frame
let cache: HashMap<char, CachedGlyph> = build_glyph_cache(font_size, padding);

// A cada frame, só desenhar
for (path, paint, kind) in &cache[&ch].paths {
    canvas.fill_path(path, paint);
}
```

### 4. Renderizar para textura (impacto alto, esforço alto)

Renderizar os glifos estáticos para uma textura off-screen (via `canvas.screenshot()` ou render target) e exibir como imagem. Redesenhar apenas o glifo sob o mouse.

Útil quando a quantidade de glifos visíveis é grande e a maioria não muda entre frames.

---

## Resumo de prioridades

| #  | Estratégia                  | Impacto | Esforço | Draw calls estimadas |
|----|-----------------------------|---------|---------|----------------------|
| 1  | Remover draws duplicados    | Alto    | Baixo   | ~432 → ~216          |
| 2  | Batch de paths              | Alto    | Médio   | ~216 → ~6-12         |
| 3  | Cache de geometria          | Médio   | Baixo   | — (reduz CPU)        |
| 4  | Renderizar para textura     | Alto    | Alto    | — (reduz GPU)        |

## Nota sobre paralelismo (Rayon)

`par_iter()` **não é aplicável** ao loop de draw atual porque `Canvas<T>` exige `&mut self` (acesso mutável exclusivo), que não pode ser compartilhado entre threads. Paralelismo só seria possível separando construção de geometria (pura) da renderização (mutável), mas com ~12 partes por glifo o overhead do thread pool supera o ganho.
