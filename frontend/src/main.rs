use rustenv_app::App;

fn main() {
    console_error_panic_hook::set_once();
    _ = console_log::init_with_level(log::Level::Debug);
    yew::Renderer::<App>::new().render();
}
