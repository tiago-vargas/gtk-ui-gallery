use relm4::RelmApp;

mod app;

fn main() {
    let relm = RelmApp::new("com.github.tiago-vargas.gtk-ui-gallery");
    relm.run::<app::AppModel>(app::AppMode::Edit);
}
