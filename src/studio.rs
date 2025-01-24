use std::fs;

use objc2::{rc::Retained, runtime::ProtocolObject};
use objc2_core_foundation::{CGFloat, CGPoint, CGSize};
use objc2_foundation::NSRect;
use objc2_metal::{MTLCommandBuffer, MTLCommandQueue, MTLDevice, MTLTexture};
use skia_safe::{
    gpu::{
        direct_contexts,
        mtl::{self, BackendContext},
        surfaces, BackendTexture, Budgeted, SurfaceOrigin,
    },
    surface::BackendHandleAccess,
    Canvas, ColorSpace, ISize, ImageInfo, Surface, SurfaceProps,
};
use syphon::metal_server::SyphonMetalServer;

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

pub struct Drawing {
    surface: Surface,
}

impl Drawing {
    fn new(surface: Surface) -> Self {
        Self { surface }
    }

    pub fn get_texture(&mut self) -> Option<BackendTexture> {
        self.surface.direct_context().unwrap().flush_and_submit();
        surfaces::get_backend_texture(&mut self.surface, BackendHandleAccess::FlushRead)
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

pub struct Studio {
    metal_context: MetalContext,
    syphon_server: Retained<SyphonMetalServer>,
}

impl Studio {
    pub fn new() -> Self {
        let metal_context = MetalContext::new();
        let syphon_server =
            SyphonMetalServer::from_device("Open World Game", &metal_context.metal_device);

        Self {
            metal_context,
            syphon_server,
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

    pub fn publish_drawing(&self, drawing: &mut Drawing) {
        let mtl_command_buffer = self
            .metal_context
            .command_queue
            .commandBuffer()
            .expect("Couldn't use command buffer on Metal GPU.");
        let texture = drawing
            .get_texture()
            .expect("Couldn't retrieve internal texture from drawing.");
        let size = CGSize::new(
            CGFloat::from(texture.width()),
            CGFloat::from(texture.height()),
        );
        let texture = texture
            .metal_texture_info()
            .expect("Couldn't use internal texture from GPU.");
        let texture = texture.texture();
        let texture = texture as *const ProtocolObject<dyn MTLTexture>;

        self.syphon_server.publish_frame_texture(
            texture,
            Retained::as_ptr(&mtl_command_buffer),
            NSRect::new(CGPoint { x: 0.0, y: 0.0 }, size),
            true,
        );
        mtl_command_buffer.commit();
    }
}

impl Drop for Studio {
    fn drop(&mut self) {
        self.syphon_server.stop();
    }
}
