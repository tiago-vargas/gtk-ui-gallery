use gtk::prelude::*;
use relm4::prelude::*;

pub(crate) struct ContentModel;

#[relm4::component(pub(crate))]
impl SimpleComponent for ContentModel {
    type Init = ();
    type Input = ();
    type Output = ();

    view! {
        gtk::Label {
            set_label: "Hello, World!",
            add_css_class: "title-1",
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

    fn update(&mut self, _message: Self::Input, _sender: ComponentSender<Self>) {}
}
