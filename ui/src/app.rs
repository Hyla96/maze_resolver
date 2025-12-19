use iced::mouse::Cursor;
use iced::widget::canvas::{self, Canvas, Cache};
use iced::widget::{column, container, row};
use iced::{Element, Event, Rectangle, Renderer, Subscription, Theme};
use iced::keyboard::{Event as KeyboardEvent, Key, key};
use logic::{Action, GameState};

use crate::controls_view::{ControlsView, Message};
use crate::map_view::MapView;
use crate::player_view::PlayerView;

const SQUARE_SIZE: u32 = 20;
const DEFAULT_WIDTH: usize = 50;
const DEFAULT_HEIGHT: usize = 30;

#[derive(Debug)]
pub struct App {
    game_state: GameState,
    tiles_cache: Cache,
}

impl App {
    pub fn new(width: usize, height: usize) -> Self {
        App {
            game_state: GameState::new(width, height),
            tiles_cache: Cache::default(),
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let canvas_width = (self.game_state.width as u32 * SQUARE_SIZE) as f32;
        let canvas_height = (self.game_state.height as u32 * SQUARE_SIZE) as f32;

        container(
            column![
                Canvas::new(self).width(canvas_width).height(canvas_height),
                row![ControlsView::view()],
            ]
        )
        .into()
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Reset => {
                self.game_state.reset();
                self.tiles_cache.clear();
            }
            Message::Up => self.game_state.apply_action(Action::Up),
            Message::Down => self.game_state.apply_action(Action::Down),
            Message::Right => self.game_state.apply_action(Action::Right),
            Message::Left => self.game_state.apply_action(Action::Left),
        }
    }
}

impl Default for App {
    fn default() -> Self {
        App::new(DEFAULT_WIDTH, DEFAULT_HEIGHT)
    }
}

impl canvas::Program<Message> for App {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: Cursor,
    ) -> Vec<canvas::Geometry> {
        let tiles_geometry = MapView::draw(
            &self.game_state,
            &self.tiles_cache,
            renderer,
            bounds.size(),
        );

        let player_geometry = PlayerView::draw(
            &self.game_state,
            renderer,
            bounds.size(),
        );

        vec![tiles_geometry, player_geometry]
    }
}

pub fn subscription(_state: &App) -> Subscription<Message> {
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
