use crate::studio::Studio;
use skia_safe::{Color4f, ColorSpace, Paint, PaintStyle};
use taffy::prelude::*;

pub fn draw_tree(layout: &Layout) {
    let studio = Studio::new();
    let mut drawing = studio.create_drawing(layout.size.width as i32, layout.size.height as i32);

    let canvas = &drawing.canvas();
    let mut paint = Paint::new(
        Color4f::new(0.0, 0.0, 1.0, 1.0),
        ColorSpace::new_srgb().as_ref(),
    );
    paint.set_style(PaintStyle::Fill);

    canvas.draw_rect(
        skia_safe::Rect {
            top: layout.padding.top,
            right: layout.size.width - layout.padding.right,
            bottom: layout.size.height - layout.padding.bottom,
            left: layout.padding.left,
        },
        &paint,
    );

    drawing.export_img("../image.png");
    studio.publish_drawing(&mut drawing);
}
