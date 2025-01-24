use crate::{
    studio::{Drawing, Studio},
    window,
};
use skia_safe::{Canvas, Color4f, ColorSpace, Paint, PaintStyle};
use taffy::prelude::*;

pub fn draw_tree(drawing: &mut Drawing, layout: &Layout) {
    let canvas = drawing.canvas();

    let mut paint = Paint::new(
        Color4f::new(0.0, 0.0, 1.0, 0.5),
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
}
