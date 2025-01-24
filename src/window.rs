use crate::{drawing, studio::Studio};
use taffy::prelude::*;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowId},
};

fn build_tree(tree: &mut TaffyTree<()>) -> Result<NodeId, taffy::TaffyError> {
    let container = tree.new_leaf(Style {
        size: Size {
            width: length(800.0),
            height: length(600.0),
        },
        padding: Rect {
            left: LengthPercentage::Length(16.0),
            right: LengthPercentage::Length(16.0),
            top: LengthPercentage::Length(32.0),
            bottom: LengthPercentage::Length(8.0),
        },
        ..Default::default()
    })?;

    Ok(container)
}

struct App {
    window: Option<Window>,
    studio: Studio,
    tree: TaffyTree<()>,
    root: NodeId,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.window = Some(
            event_loop
                .create_window(Window::default_attributes())
                .unwrap(),
        );
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                // Redraw the application.
                //
                // It's preferable for applications that do not render continuously to render in
                // this event rather than in AboutToWait, since rendering in here allows
                // the program to gracefully handle redraws requested by the OS.

                // Draw.

                self.tree
                    .compute_layout(self.root, Size::MAX_CONTENT)
                    .expect("Couldn't compute layout");
                // self.tree.print_tree(self.root);
                let layout = self
                    .tree
                    .layout(self.root)
                    .expect("Couldn't get the layout");

                let mut drawing = self
                    .studio
                    .create_drawing(layout.size.width as i32, layout.size.height as i32);
                drawing::draw_tree(&mut drawing, &layout);
                self.studio.publish_drawing(&mut drawing);

                // Queue a RedrawRequested event.
                //
                // You only need to call this if you've determined that you need to redraw in
                // applications which do not always need to. Applications that redraw continuously
                // can render here instead.
                self.window.as_ref().unwrap().request_redraw();
            }
            _ => (),
        }
    }
}
pub fn start_app() {
    let event_loop = EventLoop::new().unwrap();

    event_loop.set_control_flow(ControlFlow::Poll);

    let mut tree = TaffyTree::new();
    let root = build_tree(&mut tree).expect("Couldn't build tree");
    let mut app = App {
        window: None,
        studio: Studio::new(),
        tree,
        root,
    };
    event_loop.run_app(&mut app).expect("Couldn't run the app.");
}
