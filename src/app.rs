use gtk::prelude::*;
use relm4::*;

pub(crate) mod header;

pub(crate) struct AppModel {
    pub(crate) header: Controller<header::HeaderModel>,
}

#[relm4::component(pub(crate))]
impl SimpleComponent for AppModel {
    type Init = ();
    type Input = ();
    type Output = ();

    view! {
        main_window = gtk::Window {
            set_default_width: 500,
            set_default_height: 250,
            set_title: Some("GTK UI Gallery"),
            set_titlebar: Some(model.header.widget()),

            gtk::ScrolledWindow {
                gtk::Label {
                    set_label: "Hello, World!",
                    add_css_class: "title-1",
                },
            },
        }
    }

    fn init(
        _init: Self::Init,
        root: &Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let header: Controller<header::HeaderModel> = header::HeaderModel::builder()
            .launch(())
            .detach();

        let model = AppModel { header };

        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, _message: Self::Input, _sender: ComponentSender<Self>) {}
}
