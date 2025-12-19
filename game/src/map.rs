use iced::mouse::Cursor;
use iced::widget::canvas::{self, Canvas};
use iced::widget::{Column, button, column, container, row};
use iced::{Color, Point, Rectangle, Renderer, Size, Theme};
use rand::prelude::*;
use std::time::Instant;

#[derive(Debug)]
pub struct Map {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<Vec<Tile>>,
    pub player: Player,
    pub goal: Position,
    pub game_over: bool,
    tiles_cache: canvas::Cache,
}

impl Map {
    fn clear_tiles_cache(&mut self) {
        self.tiles_cache.clear();
    }
}

#[derive(Debug, Clone)]
pub struct Tile {
    pub walkable: bool,
    pub is_goal: bool,
}

#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Right,
    Left,
}

#[derive(Debug)]
pub struct Player {
    pub position: Position,
    pub direction: Direction,
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
        let mut rng = rand::rng();

        let mut tiles = Map::create_maze(width, height);

        let direction = Direction::Up;

        let position = Position {
            x: rng.random_range(..width),
            y: rng.random_range(..height),
        };

        let player = Player {
            direction,
            position,
        };

        // Generate goal position different from player
        let mut goal = Position {
            x: rng.random_range(..width),
            y: rng.random_range(..height),
        };
        while goal.x == position.x && goal.y == position.y {
            goal = Position {
                x: rng.random_range(..width),
                y: rng.random_range(..height),
            };
        }

        // Mark goal tile
        tiles[goal.x][goal.y].is_goal = true;

        Map {
            width,
            height,
            player,
            goal,
            game_over: false,
            tiles,
            tiles_cache: canvas::Cache::default(),
        }
    }

    pub fn get_player_view(&self) -> Vec<Position> {
        // returns up to 3 positions in front of the player based on direction
        // stops at walls or non-walkable tiles

        let mut view_positions = Vec::new();
        let (dx, dy) = match self.player.direction {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        };
        let mut x = self.player.position.x as isize;
        let mut y = self.player.position.y as isize;

        for _ in 0..3 {
            x += dx;
            y += dy;

            if x < 0 || y < 0 || x >= self.width as isize || y >= self.height as isize {
                break;
            }

            let tile = &self.tiles[x as usize][y as usize];
            if !tile.walkable {
                break;
            }

            view_positions.push(Position {
                x: x as usize,
                y: y as usize,
            });
        }

        view_positions
    }

    pub fn create_maze(width: usize, height: usize) -> Vec<Vec<Tile>> {
        let mut rng = rand::rng();
        let tiles: Vec<Vec<Tile>> = (0..width)
            .map(|_| {
                (0..height)
                    .map(|_| Tile {
                        walkable: rng.random_bool(0.8),
                        is_goal: false,
                    })
                    .collect()
            })
            .collect();

        tiles
    }

    pub fn view(&'_ self) -> Column<'_, Message> {
        let canvas_width = (self.width as u32 * SQUARE_SIZE) as f32;
        let canvas_height = (self.height as u32 * SQUARE_SIZE) as f32;

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
        if self.game_over {
            return;
        }

        match message {
            Message::Reset => {
                *self = Map::new(self.width, self.height);
            }
            Message::Up => {
                if self.player.direction == Direction::Up {
                    if self.player.position.y > 0 {
                        let tile = &self.tiles[self.player.position.x][self.player.position.y - 1];
                        if tile.walkable {
                            self.player.position.y -= 1;
                            self.check_goal();
                        }
                    }
                } else {
                    self.player.direction = Direction::Up
                }
            }
            Message::Down => {
                if self.player.direction == Direction::Down {
                    if self.player.position.y < self.height - 1 {
                        let tile = &self.tiles[self.player.position.x][self.player.position.y + 1];

                        if tile.walkable {
                            self.player.position.y += 1;
                            self.check_goal();
                        }
                    }
                } else {
                    self.player.direction = Direction::Down
                }
            }
            Message::Right => {
                if self.player.direction == Direction::Right {
                    if self.player.position.x < self.width - 1 {
                        let tile = &self.tiles[self.player.position.x + 1][self.player.position.y];
                        if tile.walkable {
                            self.player.position.x += 1;
                            self.check_goal();
                        }
                    }
                } else {
                    self.player.direction = Direction::Right
                }
            }
            Message::Left => {
                if self.player.direction == Direction::Left {
                    if self.player.position.x > 0 {
                        let tile = &self.tiles[self.player.position.x - 1][self.player.position.y];
                        if tile.walkable {
                            self.player.position.x -= 1;
                            self.check_goal();
                        }
                    }
                } else {
                    self.player.direction = Direction::Left
                }
            }
        }
    }

    fn check_goal(&mut self) {
        if self.player.position.x == self.goal.x && self.player.position.y == self.goal.y {
            self.game_over = true;
            println!("Goal reached! Game over!");
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
            for y in 0..self.height {
                for x in 0..self.width {
                    let tile = &self.tiles[x][y];
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
            let pos_x = self.player.position.x as f32 * square_size;
            let pos_y = self.player.position.y as f32 * square_size;

            // Player background
            frame.fill_rectangle(
                Point::new(pos_x, pos_y),
                Size::new(square_size, square_size),
                Color::from_rgb(1.0, 0.0, 0.0),
            );

            // Direction indicator
            let indicator_color = Color::from_rgb(0.9, 0.9, 0.0);
            match self.player.direction {
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
            let view_positions = self.get_player_view();
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
