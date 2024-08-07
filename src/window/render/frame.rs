use crate::window::RenderWindow;

mod commands;
mod surface;

pub use commands::*;
pub use surface::*;

pub struct Frame<'w> {
    pub commands: FrameCommands<'w>,
    pub surface: FrameSurface,
}

impl<'w> Frame<'w> {
    pub fn new(
        info: &'w RenderWindow<'w>,
        command_encoder: wgpu::CommandEncoder,
        surface_texture: wgpu::SurfaceTexture,
    ) -> Self {
        Self {
            commands: FrameCommands::new(info, command_encoder),
            surface: FrameSurface::new(surface_texture),
        }
    }
}
