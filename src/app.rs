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
                #[template]
                LabelPage {}
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

#[relm4::widget_template(pub(crate))]
impl WidgetTemplate for LabelPage {
    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            set_margin_all: 16,
            set_spacing: 16,

            #[template]
            StyleExamplesSection {},

            #[template]
            LoremIpsumSection {},
        }
    }
}

#[relm4::widget_template(pub(crate))]
impl WidgetTemplate for StyleExamplesSection {
    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
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
    }
}

#[relm4::widget_template(pub(crate))]
impl WidgetTemplate for LoremIpsumSection {
    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            set_spacing: 16,

            gtk::Label {
                set_label: "Lorem Ipsum",
                add_css_class: "title-1",
                set_halign: gtk::Align::Start,
            },
            gtk::Label {
                set_label: "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed suscipit laoreet velit in vestibulum. In tempus dui finibus lectus porttitor porttitor. Suspendisse tincidunt, sapien eget varius fringilla, leo tellus laoreet lectus, at laoreet ante odio feugiat turpis. Proin sed augue at diam laoreet aliquet id vel ipsum. In non accumsan orci. Praesent id sollicitudin elit. Donec rutrum ante eget fringilla fringilla.",
                set_halign: gtk::Align::Start,
                set_wrap: true,
                set_justify: gtk::Justification::Fill,
            },
            gtk::Label {
                set_label: "Nam scelerisque, urna quis convallis scelerisque, nunc libero sagittis magna, eget varius augue arcu nec dui. Vestibulum aliquam non dui at varius. Integer mattis metus non vestibulum interdum. Suspendisse sed rhoncus felis. Duis ultricies venenatis molestie. Phasellus vel quam tristique eros tempor bibendum at nec velit. Quisque at augue eu nibh pharetra laoreet. Pellentesque at ornare dui.",
                set_halign: gtk::Align::Start,
                set_wrap: true,
                set_justify: gtk::Justification::Fill,
            },
            gtk::Label {
                set_label: "Phasellus blandit cursus gravida. Suspendisse varius consequat elit nec convallis. Integer dictum gravida massa pretium luctus. Mauris id hendrerit dui, in euismod dolor. Pellentesque vitae erat ac justo ultricies commodo at ac mi. Integer vitae faucibus nisl. Nulla in libero faucibus, dignissim dolor ut, interdum tortor. Sed vitae suscipit mauris. Suspendisse tincidunt, neque in rutrum facilisis, arcu sem dapibus quam, hendrerit suscipit erat nisi ac nibh. Suspendisse id maximus odio. Pellentesque interdum cursus dui, eget auctor justo. Aliquam et leo sodales metus vehicula lobortis.",
                set_halign: gtk::Align::Start,
                set_wrap: true,
                set_justify: gtk::Justification::Fill,
            },
            gtk::Label {
                set_label: "Sed varius mi non lectus volutpat, in tempor dolor posuere. Curabitur venenatis dolor eu ante iaculis, nec sollicitudin neque gravida. In quis ultrices justo. Pellentesque molestie dolor arcu, vel mattis augue congue id. Maecenas convallis molestie blandit. Proin nisi arcu, efficitur in hendrerit nec, tincidunt eget orci. Integer mollis orci massa, id malesuada nunc malesuada at. Pellentesque dignissim neque vitae est sagittis, sit amet fringilla quam consequat. Curabitur quis tincidunt ex, eget faucibus dui. Quisque scelerisque, ex non hendrerit tincidunt, leo ante lobortis lorem, eget hendrerit nulla leo eget urna. Duis vitae finibus magna. Nam dui nisl, tristique a tristique vitae, porta vitae urna.",
                set_halign: gtk::Align::Start,
                set_wrap: true,
                set_justify: gtk::Justification::Fill,
            },
            gtk::Label {
                set_label: "Integer ullamcorper lacinia augue, ac consequat massa eleifend eu. Nunc interdum scelerisque arcu eget porttitor. Nunc suscipit dui vitae tellus aliquam mollis. Suspendisse sit amet dictum nunc. In non dui vehicula, egestas ex eget, tempus turpis. Nulla eu rutrum purus, vel viverra neque. Pellentesque nisi ipsum, rutrum vel orci congue, scelerisque iaculis ipsum. Donec iaculis fermentum ligula, at bibendum urna ornare sed. Duis dictum mauris non libero pulvinar blandit. Pellentesque imperdiet commodo orci et tempus. Nullam arcu odio, lobortis vitae faucibus vitae, blandit eget magna. Nunc accumsan mi et tortor elementum imperdiet. Praesent aliquet, sapien at mollis viverra, magna libero venenatis lectus, non commodo mi ipsum sit amet ligula.",
                set_halign: gtk::Align::Start,
                set_wrap: true,
                set_justify: gtk::Justification::Fill,
            },
        }
    }
}
