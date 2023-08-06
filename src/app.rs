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
                gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_margin_all: 16,
                    set_spacing: 16,

                    gtk::Label {
                        set_label: "Title 1",
                        add_css_class: "title-1",
                        set_halign: gtk::Align::Start,
                    },
                    gtk::Label {
                        set_label: "Title 2",
                        add_css_class: "title-2",
                        set_halign: gtk::Align::Start,
                    },
                    gtk::Label {
                        set_label: "Title 3",
                        add_css_class: "title-3",
                        set_halign: gtk::Align::Start,
                    },
                    gtk::Label {
                        set_label: "Title 4",
                        add_css_class: "title-4",
                        set_halign: gtk::Align::Start,
                    },
                    gtk::Label {
                        set_label: "Heading",
                        add_css_class: "heading",
                        set_halign: gtk::Align::Start,
                    },
                    gtk::Label {
                        set_label: "Body (default)",
                        add_css_class: "body",
                        set_halign: gtk::Align::Start,
                    },
                    gtk::Label {
                        set_label: "Caption Heading",
                        add_css_class: "caption-heading",
                        set_halign: gtk::Align::Start,
                    },
                    gtk::Label {
                        set_label: "Caption",
                        add_css_class: "caption",
                        set_halign: gtk::Align::Start,
                    },
                    gtk::Label {
                        set_label: "Monospace (0123456789)",
                        add_css_class: "monospace",
                        set_halign: gtk::Align::Start,
                    },
                    gtk::Label {
                        set_label: "Numeric (0123465789)",
                        add_css_class: "numeric",
                        set_halign: gtk::Align::Start,
                    },
                    gtk::Label {
                        set_label: "Dim Label",
                        add_css_class: "dim-label",
                        set_halign: gtk::Align::Start,
                    },
                }
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
