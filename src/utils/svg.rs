use femtovg::{Canvas, Color, Paint, Path, Renderer};
use usvg::{Options, Tree};

pub struct CustomSize {
    pub scale_x: f32,
    pub scale_y: f32,
}

pub struct Position {
    pub x: f32,
    pub y: f32,
}

pub fn draw_svg<T: Renderer>(
    canvas: &mut Canvas<T>,
    icon: &[u8],
    pos: Position,
    color: Option<Color>,
    size: Option<CustomSize>,
) -> () {
    let mut paths = render_svg(
        Tree::from_data(icon, &Options::default()).unwrap(),
        size,
    );

    canvas.save();
    canvas.translate(pos.x, pos.y);

    for (path, fill, stroke) in paths.iter_mut() {
        if let Some(fill) = fill {
            if let Some(col) = color {
                fill.set_color(col);
            }

            canvas.fill_path(path, fill);
        }

        if let Some(stroke) = stroke {
            if let Some(col) = color {
                stroke.set_color(col);
            }

            canvas.stroke_path(path, stroke);
        }
    }

    canvas.restore();
}

pub fn render_svg(
    svg: usvg::Tree,
    size: Option<CustomSize>,
) -> Vec<(Path, Option<Paint>, Option<Paint>)> {
    let mut paths = Vec::new();

    let dimensions = size.unwrap_or(CustomSize {
        scale_x: svg.size().width(),
        scale_y: svg.size().height(),
    });

    let scale_x = dimensions.scale_x / svg.size().width();
    let scale_y = dimensions.scale_y / svg.size().height();

    fn collect_paths(
        children: &[usvg::Node],
        paths: &mut Vec<(Path, Option<Paint>, Option<Paint>)>,
        (scale_x, scale_y): (f32, f32),
    ) {
        use usvg::Node;
        use usvg::tiny_skia_path::PathSegment;

        for node in children {
            match node {
                Node::Group(group) => {
                    collect_paths(group.children(), paths, (scale_x, scale_y));
                }
                Node::Path(svg_path) => {
                    let mut path = Path::new();

                    for command in svg_path.data().segments() {
                        match command {
                            PathSegment::MoveTo(pt) => path.move_to(pt.x * scale_x, pt.y * scale_y),
                            PathSegment::LineTo(pt) => path.line_to(pt.x * scale_x, pt.y * scale_y),
                            PathSegment::CubicTo(pt1, pt2, pt) => path.bezier_to(
                                pt1.x * scale_x,
                                pt1.y * scale_y,
                                pt2.x * scale_x,
                                pt2.y * scale_y,
                                pt.x * scale_x,
                                pt.y * scale_y,
                            ),
                            PathSegment::QuadTo(pt1, pt) => path.quad_to(
                                pt1.x * scale_x,
                                pt1.y * scale_y,
                                pt.x * scale_x,
                                pt.y * scale_y,
                            ),
                            PathSegment::Close => path.close(),
                        }
                    }

                    let to_femto_color = |usvg_paint: &usvg::Paint| match usvg_paint {
                        usvg::Paint::Color(usvg::Color { red, green, blue }) => {
                            Some(Color::rgb(*red, *green, *blue))
                        }
                        _ => None,
                    };

                    let fill = svg_path
                        .fill()
                        .and_then(|fill| to_femto_color(&fill.paint()))
                        .map(|col| Paint::color(col).with_anti_alias(true));

                    let stroke = svg_path.stroke().and_then(|stroke| {
                        to_femto_color(&stroke.paint()).map(|paint| {
                            let mut paint = Paint::color(paint)
                                .with_line_width(stroke.width().get() * scale_x)
                                .with_anti_alias(true);

                            if let Some(dasharray) = stroke.dasharray() {
                                let scaled_dasharray: Vec<f32> =
                                    dasharray.iter().map(|v| v * scale_x).collect();

                                paint.set_line_dash(&scaled_dasharray);
                                paint.set_line_dash_offset(stroke.dashoffset() * scale_x);
                            }

                            paint
                        })
                    });

                    paths.push((path, fill, stroke))
                }
                _ => {}
            }
        }
    }

    collect_paths(svg.root().children(), &mut paths, (scale_x, scale_y));

    paths
}
