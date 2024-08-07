pub mod pieces;

use app::*;
use sww::app::App;
use sww::app::AppPack;
use sww::event_handler_builder;
use sww::window::DefaultRenderWindowSettings;
use sww::window::*;

mod app;

pub fn main() {
    env_logger::init();

    let mut app = App::new(|event_loop| {
        let window = event_loop
            .create_window(window_attributes("che6", 400, 200))
            .unwrap();

        AppPack::new(
            window,
            rw_builder(&DefaultRenderWindowSettings),
            event_handler_builder!(MyApp::new),
        )
    });

    event_loop().run_app(&mut app).unwrap();
}
