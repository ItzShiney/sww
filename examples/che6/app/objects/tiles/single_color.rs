use crate::Drawer;
use crate::Scalable;
use crate::Scalables;
use sww::buffers::Binding;
use sww::buffers::MutBuffer;
use sww::buffers::MutVecBuffer;
use sww::shaders;
use sww::shaders::mesh::Transform;
use sww::utility::PushLast;
use sww::window::RenderWindow;
use sww::Color;
use sww::Vec2;

pub struct SingleColorTiles<'w> {
    pub transforms: MutVecBuffer<'w, Transform>,
    bind_group0: shaders::mesh::BindGroup0,
}

impl<'w> SingleColorTiles<'w> {
    pub fn new(
        rw: &'w RenderWindow,
        scalables: &mut Scalables,
        color: Color,
        transforms: MutVecBuffer<'w, Transform>,
    ) -> Self {
        let scalable = scalables.push_last(Scalable::new(
            MutBuffer::new(
                rw.device(),
                Transform {
                    color: color.into(),
                    ..Default::default()
                },
            ),
            Vec2::splat(2. / 8.),
        ));

        let bind_group0 = {
            let global_transform = scalable.transform_buffer.buffer().binding();
            shaders::mesh::BindGroup0::from_bindings(rw.device(), global_transform.into())
        };

        Self {
            transforms,
            bind_group0,
        }
    }
}

impl<'c> SingleColorTiles<'_> {
    pub fn draw(
        &'c self,
        drawer: &'c Drawer,
        render_pass: &mut wgpu::RenderPass<'c>,
        bind_group1: &'c shaders::mesh::BindGroup1,
    ) {
        drawer.draw_squares(
            render_pass,
            self.transforms.slice(..),
            shaders::mesh::BindGroups {
                bind_group0: &self.bind_group0,
                bind_group1,
            },
        );
    }
}
