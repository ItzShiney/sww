use crate::sheet::*;
use event::*;
use sww::app::EventInfo;
use sww::app::HandleEvent;
use sww::vec2;
use sww::window::*;
use sww::Ratio;
use sww::Vec2;

mod drawer;
mod objects;
mod pieces;

pub use drawer::*;
pub use objects::*;
pub use pieces::*;

pub fn translation(x: i32, y: i32) -> Vec2 {
    vec2(x as _, y as _)
}

pub struct MyApp<'i, 'w> {
    rw: &'i RenderWindow<'w>,
    objects: Objects<'i, 'w>,
    drawer: Drawer,
}

impl<'i, 'w> MyApp<'i, 'w> {
    pub fn new(rw: &'i RenderWindow<'w>) -> Self {
        let drawer = Drawer::new(rw);
        let mut objects = Objects::new(rw);

        objects.pieces.transforms.push(make_piece_transform(
            0,
            0,
            PieceType::Boat,
            PieceColor::White,
        ));
        objects.pieces.transforms.push(make_piece_transform(
            -1,
            -1,
            PieceType::Boat,
            PieceColor::Black,
        ));

        Self {
            rw,
            drawer,
            objects,
        }
    }
}

impl HandleEvent for MyApp<'_, '_> {
    fn on_resized(&mut self, _info: EventInfo, new_size: PhysicalSize) {
        self.rw.resize_surface(new_size);
        self.rw.window().request_redraw();
    }

    fn on_redraw_requested(&mut self, _info: EventInfo) {
        let Ok(mut frame) = self.rw.start_drawing() else {
            return;
        };

        self.objects.scale(self.rw.window().ratio());
        self.draw(&mut frame);
    }
}

impl MyApp<'_, '_> {
    fn draw(&mut self, frame: &mut Frame) {
        let mut render_pass =
            frame
                .commands
                .encoder()
                .begin_render_pass(&wgpu::RenderPassDescriptor {
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: frame.surface.view(),
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    ..Default::default()
                });

        self.objects.draw(&self.drawer, &mut render_pass);
    }
}
