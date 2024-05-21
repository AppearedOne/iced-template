use iced::event::{self, Event};
use iced::widget::{
    button, checkbox, column, combo_box, container, horizontal_space, row, scrollable, stack, text,
    text_input, tooltip, vertical_space,
};
use iced::window;
use iced::{alignment, Command, Element, Font, Length, Padding, Subscription, Theme};
use iced_aw::graphics::icons::{bootstrap::icon_to_string, BootstrapIcon, BOOTSTRAP_FONT_BYTES};

#[tokio::main]
async fn main() -> iced::Result {
    iced::program(App::title, App::update, App::view)
        .theme(App::theme)
        .font(BOOTSTRAP_FONT_BYTES)
        .subscription(App::subscription)
        .load(|| Command::none())
        .exit_on_close_request(false)
        .run()
}

const ICON_FONT: Font = Font::with_name("bootstrap-icons");

pub struct App {}

#[derive(Debug, Clone)]
pub enum Message {
    Exit(()),
    EventOccurred(Event),
}

impl App {
    fn new() -> (Self, Command<Message>) {
        (Self {}, Command::none())
    }
    fn title(&self) -> String {
        String::from("Application")
    }
    fn subscription(&self) -> Subscription<Message> {
        event::listen().map(Message::EventOccurred)
    }
    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Exit(_) => {
                return window::close(window::Id::MAIN);
            }
            Message::EventOccurred(event) => {
                if let Event::Window(_id, window::Event::CloseRequested) = event {
                    return Command::none();
                }
                return Command::none();
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let text = text("Hello World").size(50);
        column![text]
            .align_items(alignment::Alignment::Center)
            .into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new().0
    }
}
