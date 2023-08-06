use gtk::prelude::*;
use relm4::*;

pub(crate) struct HeaderModel;

#[relm4::component(pub(crate))]
impl SimpleComponent for HeaderModel {
    type Init = ();
    type Input = ();
    type Output = ();

    view! {
        gtk::HeaderBar {
            #[wrap(Some)]
            set_title_widget = &gtk::Box {
                gtk::Label {
                    set_label: "GTK UI Gallery",
                    add_css_class: "heading",
                }
            }
        }
    }

    fn init(
        _init: Self::Init,
        root: &Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = HeaderModel;
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }
}
