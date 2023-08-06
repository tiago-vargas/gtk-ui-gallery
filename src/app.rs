use gtk::prelude::{ApplicationExt, GtkWindowExt};
use relm4::*;

pub(crate) mod dialog;
pub(crate) mod header;

#[derive(Debug)]
pub(crate) enum AppMode {
    View,
    Edit,
    Export,
}

#[derive(Debug)]
pub(crate) enum AppMsg {
    SetMode(AppMode),
    CloseRequest,
    Close,
}

pub(crate) struct AppModel {
    pub(crate) mode: AppMode,
    pub(crate) header: Controller<header::HeaderModel>,
    pub(crate) dialog: Controller<dialog::DialogModel>,
}

#[relm4::component(pub(crate))]
impl SimpleComponent for AppModel {
    type Init = AppMode;
    type Input = AppMsg;
    type Output = ();

    view! {
        main_window = gtk::Window {
            set_default_width: 500,
            set_default_height: 250,
            set_titlebar: Some(model.header.widget()),

            gtk::Label {
                #[watch]
                set_label: &format!("Placeholder for {:?}", model.mode),
            },
            connect_close_request[sender] => move |_| {
                sender.input(AppMsg::CloseRequest);
                gtk::Inhibit(true)
            }
        }
    }

    fn init(
        params: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let header: Controller<header::HeaderModel> = header::HeaderModel::builder()
            .launch(())
            .forward(sender.input_sender(), |msg| match msg {
                header::HeaderOutput::View => AppMsg::SetMode(AppMode::View),
                header::HeaderOutput::Edit => AppMsg::SetMode(AppMode::Edit),
                header::HeaderOutput::Export => AppMsg::SetMode(AppMode::Export),
            });

        let dialog = dialog::DialogModel::builder()
            .transient_for(root)
            .launch(true)
            .forward(sender.input_sender(), |msg| match msg {
                dialog::DialogOutput::Close => AppMsg::Close,
            });

        let model = AppModel {
            mode: params,
            header,
            dialog,
        };

        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            AppMsg::SetMode(mode) => {
                self.mode = mode;
            }
            AppMsg::CloseRequest => {
                self.dialog
                    .sender()
                    .send(dialog::DialogInput::Show)
                    .unwrap();
            }
            AppMsg::Close => {
                relm4::main_application().quit();
            }
        }
    }
}
