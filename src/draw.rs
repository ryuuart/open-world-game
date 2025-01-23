use std::fs;

use objc2::{rc::Retained, runtime::ProtocolObject};
use objc2_metal::{MTLCommandQueue, MTLDevice};
use skia_safe::{
    gpu::{
        direct_contexts,
        mtl::{self, BackendContext},
        surfaces, BackendTexture, Budgeted, SurfaceOrigin,
    },
    surface::BackendHandleAccess,
    Canvas, Color4f, ColorSpace, ISize, ImageInfo, Paint, PaintStyle, Surface, SurfaceProps,
};
use taffy::prelude::*;

struct MetalContext {
    metal_device: Retained<ProtocolObject<dyn MTLDevice>>,
    command_queue: Retained<ProtocolObject<dyn MTLCommandQueue>>,
    sk_metal_backend_context: BackendContext,
}

impl MetalContext {
    pub fn new() -> Self {
        let metal_device = objc2_metal::MTLCreateSystemDefaultDevice()
            .expect("Failed to get default system device.");
        let command_queue = metal_device
            .newCommandQueue()
            .expect("Failed to create a command queue.");
        let sk_metal_backend_context = unsafe {
            mtl::BackendContext::new(
                Retained::as_ptr(&metal_device) as mtl::Handle,
                Retained::as_ptr(&command_queue) as mtl::Handle,
            )
        };

        Self {
            metal_device,
            command_queue,
            sk_metal_backend_context,
        }
    }
}

pub struct Studio {
    metal_context: MetalContext,
}

pub struct Drawing {
    surface: Surface,
}

impl Drawing {
    fn new(surface: Surface) -> Self {
        Self { surface }
    }

    pub fn get_texture(&mut self) -> Option<BackendTexture> {
        surfaces::get_backend_texture(&mut self.surface, BackendHandleAccess::FlushWrite)
    }

    pub fn canvas(&mut self) -> &Canvas {
        &self.surface.canvas()
    }

    pub fn export_img(&mut self, path: &str) {
        let image = self.surface.image_snapshot();
        let data = image
            .encode(
                &mut self
                    .surface
                    .canvas()
                    .direct_context()
                    .expect("Couldn't use gpu from canvas"),
                skia_safe::EncodedImageFormat::PNG,
                100,
            )
            .expect("Couldn't write canvas to image.");
        fs::write(path, data.as_bytes()).expect("Couldn't write image to disk.");
    }
}

impl Studio {
    pub fn new() -> Self {
        Self {
            metal_context: MetalContext::new(),
        }
    }

    pub fn create_drawing(&self, width: i32, height: i32) -> Drawing {
        let image_info = ImageInfo::new_n32_premul(ISize { width, height }, ColorSpace::new_srgb());
        let surface_props = SurfaceProps::default();
        let mut sk_metal_context =
            direct_contexts::make_metal(&self.metal_context.sk_metal_backend_context, None)
                .expect("Failed to create context on Metal gpu directly.");

        let surface = surfaces::render_target(
            &mut sk_metal_context,
            Budgeted::No,
            &image_info,
            1,
            SurfaceOrigin::TopLeft,
            Some(&surface_props),
            false,
            false,
        )
        .expect("Failed to create surface from Metal gpu directly.");

        Drawing::new(surface)
    }
}

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
}
