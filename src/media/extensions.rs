pub trait DefaultView {
    fn default_view(&self) -> wgpu::TextureView;
}

impl DefaultView for wgpu::Texture {
    fn default_view(&self) -> wgpu::TextureView {
        self.create_view(&Default::default())
    }
}
