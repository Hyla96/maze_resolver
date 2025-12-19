use iced::mouse::Cursor;
use iced::widget::canvas::{self, Canvas};
use iced::widget::{Column, button, column, container, row};
use iced::{Color, Point, Rectangle, Renderer, Size, Theme};
use std::time::Instant;

use crate::game_state::GameState;
use crate::types::{Action, Direction};

#[derive(Debug)]
pub struct Map {
    game_state: GameState,
    tiles_cache: canvas::Cache,
}

impl Map {
    fn clear_tiles_cache(&mut self) {
        self.tiles_cache.clear();
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Reset,
    Up,
    Down,
    Right,
    Left,
}

const SQUARE_SIZE: u32 = 20;
const DIRECTION_INDICATOR_SIZE: u32 = 4;
const BUTTON_SIZE: u32 = 30;
const DEFAULT_WIDTH: usize = 50;
const DEFAULT_HIGHT: usize = 30;

impl Map {
    pub fn new(width: usize, height: usize) -> Self {
        Map {
            game_state: GameState::new(width, height),
            tiles_cache: canvas::Cache::default(),
        }
    }

    pub fn view(&'_ self) -> Column<'_, Message> {
        let canvas_width = (self.game_state.width as u32 * SQUARE_SIZE) as f32;
        let canvas_height = (self.game_state.height as u32 * SQUARE_SIZE) as f32;

        column![
            Canvas::new(self).width(canvas_width).height(canvas_height),
            row![
                button("Reset").on_press(Message::Reset),
                column![
                    row![
                        container("").height(BUTTON_SIZE).width(BUTTON_SIZE),
                        button("↑")
                            .height(BUTTON_SIZE)
                            .width(BUTTON_SIZE)
                            .on_press(Message::Up),
                        container("").height(BUTTON_SIZE).width(BUTTON_SIZE),
                    ],
                    row![
                        button("←")
                            .height(BUTTON_SIZE)
                            .width(BUTTON_SIZE)
                            .on_press(Message::Left),
                        container("").height(BUTTON_SIZE).width(BUTTON_SIZE),
                        button("→")
                            .height(BUTTON_SIZE)
                            .width(BUTTON_SIZE)
                            .on_press(Message::Right),
                    ],
                    row![
                        container("").height(BUTTON_SIZE).width(BUTTON_SIZE),
                        button("↓")
                            .height(BUTTON_SIZE)
                            .width(BUTTON_SIZE)
                            .on_press(Message::Down),
                        container("").height(BUTTON_SIZE).width(BUTTON_SIZE),
                    ],
                ]
            ],
        ]
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Reset => {
                self.game_state.reset();
                self.clear_tiles_cache();
            }
            Message::Up => self.game_state.apply_action(Action::Up),
            Message::Down => self.game_state.apply_action(Action::Down),
            Message::Right => self.game_state.apply_action(Action::Right),
            Message::Left => self.game_state.apply_action(Action::Left),
        }
    }
}

impl Default for Map {
    fn default() -> Self {
        Map::new(DEFAULT_WIDTH, DEFAULT_HIGHT)
    }
}

impl canvas::Program<Message> for Map {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: Cursor,
    ) -> Vec<canvas::Geometry> {
        let draw_start = Instant::now();
        let square_size = SQUARE_SIZE as f32;
        let direction_size = DIRECTION_INDICATOR_SIZE as f32;

        // Draw static tiles (cached)
        let tiles_geometry = self.tiles_cache.draw(renderer, bounds.size(), |frame| {
            let compute_start = Instant::now();
            for y in 0..self.game_state.height {
                for x in 0..self.game_state.width {
                    let tile = &self.game_state.tiles[x][y];
                    let pos_x = x as f32 * square_size;
                    let pos_y = y as f32 * square_size;

                    let color = if tile.is_goal {
                        Color::from_rgb(0.0, 1.0, 0.0)
                    } else if tile.walkable {
                        Color::WHITE
                    } else {
                        Color::BLACK
                    };
                    frame.fill_rectangle(
                        Point::new(pos_x, pos_y),
                        Size::new(square_size, square_size),
                        color,
                    );
                }
            }
            println!(
                "Tiles cache rebuild | Compute: {:.3}ms | Draw: {:.2}ms",
                compute_start.elapsed().as_secs_f64() * 1000.0,
                draw_start.elapsed().as_secs_f64() * 1000.0
            );
        });

        // Draw player (not cached)
        let player_geometry = canvas::Cache::default().draw(renderer, bounds.size(), |frame| {
            let pos_x = self.game_state.player.position.x as f32 * square_size;
            let pos_y = self.game_state.player.position.y as f32 * square_size;

            // Player background
            frame.fill_rectangle(
                Point::new(pos_x, pos_y),
                Size::new(square_size, square_size),
                Color::from_rgb(1.0, 0.0, 0.0),
            );

            // Direction indicator
            let indicator_color = Color::from_rgb(0.9, 0.9, 0.0);
            match self.game_state.player.direction {
                Direction::Up => frame.fill_rectangle(
                    Point::new(pos_x, pos_y),
                    Size::new(square_size, direction_size),
                    indicator_color,
                ),
                Direction::Down => frame.fill_rectangle(
                    Point::new(pos_x, pos_y + square_size - direction_size),
                    Size::new(square_size, direction_size),
                    indicator_color,
                ),
                Direction::Left => frame.fill_rectangle(
                    Point::new(pos_x, pos_y),
                    Size::new(direction_size, square_size),
                    indicator_color,
                ),
                Direction::Right => frame.fill_rectangle(
                    Point::new(pos_x + square_size - direction_size, pos_y),
                    Size::new(direction_size, square_size),
                    indicator_color,
                ),
            }

            // Draw player view overlay
            let view_positions = self.game_state.get_player_view();
            for pos in view_positions {
                let view_x = pos.x as f32 * square_size;
                let view_y = pos.y as f32 * square_size;

                frame.fill_rectangle(
                    Point::new(view_x, view_y),
                    Size::new(square_size, square_size),
                    Color::from_rgba(1.0, 1.0, 0.0, 0.3),
                );
            }

            println!(
                "Player draw: {:.3}ms",
                draw_start.elapsed().as_secs_f64() * 1000.0
            );
        });

        vec![tiles_geometry, player_geometry]
    }
}
