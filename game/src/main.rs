mod map;

use iced::keyboard::{Event as KeyboardEvent, Key, key};
use iced::widget::container;
use iced::{Element, Event, Result, Subscription, Task};
use map::{Map, Message};

fn main() -> Result {
    iced::application(Map::default, update, view)
        .subscription(subscription)
        .run()
}

fn update(state: &mut Map, message: Message) -> Task<Message> {
    state.update(message);
    Task::none()
}

fn view(state: &Map) -> Element<'_, Message> {
    container(state.view()).into()
}

fn subscription(_state: &Map) -> Subscription<Message> {
    iced::event::listen().filter_map(|event| match event {
        Event::Keyboard(KeyboardEvent::KeyPressed {
            key: Key::Named(key::Named::ArrowUp),
            ..
        }) => Some(Message::Up),
        Event::Keyboard(KeyboardEvent::KeyPressed {
            key: Key::Named(key::Named::ArrowDown),
            ..
        }) => Some(Message::Down),
        Event::Keyboard(KeyboardEvent::KeyPressed {
            key: Key::Named(key::Named::ArrowLeft),
            ..
        }) => Some(Message::Left),
        Event::Keyboard(KeyboardEvent::KeyPressed {
            key: Key::Named(key::Named::ArrowRight),
            ..
        }) => Some(Message::Right),
        _ => None,
    })
}
