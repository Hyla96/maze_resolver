mod map_view;
mod player_view;
mod controls_view;
mod app;

use iced::{Result, Task, Element};
use app::App;
use controls_view::Message;

fn main() -> Result {
    iced::application(App::default, update, view)
        .subscription(app::subscription)
        .run()
}

fn update(state: &mut App, message: Message) -> Task<Message> {
    state.update(message);
    Task::none()
}

fn view(state: &App) -> Element<'_, Message> {
    state.view()
}
