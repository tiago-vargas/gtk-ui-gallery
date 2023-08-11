use gtk::prelude::*;
use relm4::*;

pub(crate) struct ContentModel;

#[relm4::component(pub(crate))]
impl SimpleComponent for ContentModel {
    type Init = ();
    type Input = ();
    type Output = ();

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            set_margin_all: 16,
            set_spacing: 16,

            gtk::Label {
                set_label: "Grid",
                add_css_class: "title-1",
                set_halign: gtk::Align::Start,
            },

            gtk::Grid {
                // set_row_spacing: 16,
                // set_column_spacing: 16,

                attach[1, 1, 1, 1] = &gtk::Frame {
                    gtk::Label {
                        set_label: "Cell 1",
                        // add_css_class: "large-title",
                        set_halign: gtk::Align::Start,
                    }
                },
                attach[2, 1, 1, 1] = &gtk::Frame {
                    gtk::Label {
                        set_label: "Cell 2",
                        // add_css_class: "large-title",
                        set_halign: gtk::Align::Start,
                    }
                },
                attach[3, 1, 1, 1] = &gtk::Frame {
                    gtk::Label {
                        set_label: "Cell 3",
                        // add_css_class: "large-title",
                        set_halign: gtk::Align::Start,
                    }
                },

                attach[1, 2, 1, 1] = &gtk::Label {
                    set_label: "Cell 4",
                    // add_css_class: "large-title",
                    set_halign: gtk::Align::Start,
                },
                attach[2, 2, 1, 1] = &gtk::Label {
                    set_label: "Cell 5",
                    // add_css_class: "large-title",
                    set_halign: gtk::Align::Start,
                },
                attach[3, 2, 1, 1] = &gtk::Label {
                    set_label: "Cell 6",
                    // add_css_class: "large-title",
                    set_halign: gtk::Align::Start,
                },
            }
        }
    }

    fn init(
        _init: Self::Init,
        root: &Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = ContentModel;
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }
}
