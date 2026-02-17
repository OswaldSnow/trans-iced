use iced::{
    Alignment, Element,
    Length::{self},
    Task,
    widget::{button, column, operation, text_editor},
    window,
};

const ORIGIN_TEXT_ID: &str = "origin_text";

#[derive(Debug)]
pub struct AppState {
    origin_content: text_editor::Content,
    result_content: text_editor::Content,
    appid: String,
    appkey: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    _Exit,
    EditOrigin(text_editor::Action),
    EditResult(text_editor::Action),
    Translate,
}

impl AppState {
    pub fn new(origin_text: &str, appid: String, appkey: String) -> (Self, Task<Message>) {
        (
            AppState {
                origin_content: text_editor::Content::with_text(origin_text),
                result_content: text_editor::Content::with_text(""),
                appid,
                appkey,
            },
            operation::focus(ORIGIN_TEXT_ID),
        )
    }
}

impl Default for AppState {
    fn default() -> Self {
        AppState {
            origin_content: text_editor::Content::with_text(""),
            result_content: text_editor::Content::with_text(""),
            appid: String::from(""),
            appkey: String::from(""),
        }
    }
}

impl AppState {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::_Exit => window::latest().and_then(window::close),
            Message::EditOrigin(action) => {
                self.origin_content.perform(action);
                Task::none()
            }
            Message::EditResult(_action) => {
                self.result_content.perform(_action);
                Task::none()
            }
            Message::Translate => {
                let res: String = crate::translate_api::translate(
                    &self.origin_content.text(),
                    "en",
                    "zh",
                    self.appid.clone(),
                    self.appkey.clone(),
                )
                .expect("error");
                self.result_content = text_editor::Content::with_text(&res);
                Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        column![
            // column![button("Exit").on_press(Message::Exit)].align_x(Alignment::End).width(Length::Fill),
            text_editor(&self.origin_content)
                .id(ORIGIN_TEXT_ID)
                .on_action(Message::EditOrigin)
                .height(300),
            column![button("翻译").on_press(Message::Translate)]
                .align_x(Alignment::Center)
                .width(Length::Fill),
            text_editor(&self.result_content)
                .on_action(Message::EditResult)
                .height(300),
        ]
        .spacing(10)
        .padding(8)
        .into()
    }
}
