use crate::window::RenderWindow;

mod commands;
mod surface;

pub use commands::*;
pub use surface::*;

pub struct Frame<'i, 'w> {
    pub commands: FrameCommands<'i, 'w>,
    pub surface: FrameSurface,
}

impl<'i, 'w> Frame<'i, 'w> {
    pub fn new(
        info: &'i RenderWindow<'w>,
        command_encoder: wgpu::CommandEncoder,
        surface_texture: wgpu::SurfaceTexture,
    ) -> Self {
        Self {
            commands: FrameCommands::new(info, command_encoder),
            surface: FrameSurface::new(surface_texture),
        }
    }
}
