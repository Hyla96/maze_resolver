use iced::widget::{Column, button, column, container, row, text};
use iced::{Background, Color, Element};
use rand::prelude::*;
use std::time::Instant;

#[derive(Debug)]
pub struct Map {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<Vec<Tile>>,
    pub player: Player,
}

#[derive(Debug, Clone)]
pub struct Tile {
    pub walkable: bool,
}

#[derive(Debug)]
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
const DEFAULT_WIDTH: usize = 350;
const DEFAULT_HIGHT: usize = 30;

impl Map {
    pub fn new(width: usize, height: usize) -> Self {
        let player = Player {
            direction: Direction::Up,
            position: Position { x: 0, y: 0 },
        };
        Map {
            width,
            height,
            player,
            tiles: Map::create_maze(width, height),
        }
    }

    pub fn create_maze(width: usize, height: usize) -> Vec<Vec<Tile>> {
        let mut rng = rand::rng();

        let tiles: Vec<Vec<Tile>> = (0..width)
            .map(|_| {
                (0..height)
                    .map(|_| Tile {
                        walkable: rng.random_bool(0.8),
                    })
                    .collect()
            })
            .collect();

        tiles
    }

    pub fn view(&'_ self) -> Column<'_, Message> {
        let start = Instant::now();

        // Build the grid of tiles
        let mut grid_column = column![];

        for y in 0..self.height {
            let mut grid_row = row![];

            for x in 0..self.width {
                let tile = &self.tiles[x][y];

                // Check if this is the player's position
                if self.player.position.x == x && self.player.position.y == y {
                    // Player tile (red) with directional indicator
                    let direction_indicator = match self.player.direction {
                        Direction::Up => container("")
                            .width(SQUARE_SIZE)
                            .height(DIRECTION_INDICATOR_SIZE)
                            .style(|_theme| container::Style {
                                background: Some(Background::Color(Color::from_rgb(0.9, 0.9, 0.0))),
                                ..Default::default()
                            }),
                        Direction::Down => container("")
                            .width(SQUARE_SIZE)
                            .height(DIRECTION_INDICATOR_SIZE)
                            .style(|_theme| container::Style {
                                background: Some(Background::Color(Color::from_rgb(0.9, 0.9, 0.0))),
                                ..Default::default()
                            }),
                        Direction::Right => container("")
                            .width(DIRECTION_INDICATOR_SIZE)
                            .height(SQUARE_SIZE)
                            .style(|_theme| container::Style {
                                background: Some(Background::Color(Color::from_rgb(0.9, 0.9, 0.0))),
                                ..Default::default()
                            }),
                        Direction::Left => container("")
                            .width(DIRECTION_INDICATOR_SIZE)
                            .height(SQUARE_SIZE)
                            .style(|_theme| container::Style {
                                background: Some(Background::Color(Color::from_rgb(0.9, 0.9, 0.0))),
                                ..Default::default()
                            }),
                    };

                    let player_element: Element<Message> = match self.player.direction {
                        Direction::Up => column![
                            direction_indicator,
                            container("")
                                .width(SQUARE_SIZE)
                                .height(SQUARE_SIZE - DIRECTION_INDICATOR_SIZE)
                                .style(|_theme| container::Style {
                                    background: Some(Background::Color(Color::from_rgb(
                                        1.0, 0.0, 0.0
                                    ))),
                                    ..Default::default()
                                }),
                        ]
                        .into(),
                        Direction::Down => column![
                            container("")
                                .width(SQUARE_SIZE)
                                .height(SQUARE_SIZE - DIRECTION_INDICATOR_SIZE)
                                .style(|_theme| container::Style {
                                    background: Some(Background::Color(Color::from_rgb(
                                        1.0, 0.0, 0.0
                                    ))),
                                    ..Default::default()
                                }),
                            direction_indicator,
                        ]
                        .into(),
                        Direction::Right => row![
                            container("")
                                .width(SQUARE_SIZE - DIRECTION_INDICATOR_SIZE)
                                .height(SQUARE_SIZE)
                                .style(|_theme| container::Style {
                                    background: Some(Background::Color(Color::from_rgb(
                                        1.0, 0.0, 0.0
                                    ))),
                                    ..Default::default()
                                }),
                            direction_indicator,
                        ]
                        .into(),
                        Direction::Left => row![
                            direction_indicator,
                            container("")
                                .width(SQUARE_SIZE - DIRECTION_INDICATOR_SIZE)
                                .height(SQUARE_SIZE)
                                .style(|_theme| container::Style {
                                    background: Some(Background::Color(Color::from_rgb(
                                        1.0, 0.0, 0.0
                                    ))),
                                    ..Default::default()
                                }),
                        ]
                        .into(),
                    };

                    grid_row = grid_row.push(
                        container(player_element)
                            .height(SQUARE_SIZE)
                            .width(SQUARE_SIZE),
                    );
                } else {
                    // Regular tile
                    let color = if tile.walkable {
                        Color::WHITE
                    } else {
                        Color::BLACK
                    };

                    grid_row =
                        grid_row.push(container("").height(SQUARE_SIZE).width(SQUARE_SIZE).style(
                            move |_theme| container::Style {
                                background: Some(Background::Color(color)),
                                ..Default::default()
                            },
                        ));
                }
            }

            grid_column = grid_column.push(grid_row);
        }

        column![
            grid_column,
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
            text(format!(
                "UI render time: {:.2}ms",
                start.elapsed().as_secs_f64() * 1000.0
            )),
        ]
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Reset => self.tiles = Map::create_maze(self.width, self.height),
            Message::Up => {
                if self.player.direction == Direction::Up {
                    if self.player.position.y > 0 {
                        let tile = &self.tiles[self.player.position.x][self.player.position.y - 1];
                        if tile.walkable {
                            self.player.position.y -= 1;
                            println!("Position is {:#?}", self.player.position)
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
                            println!("Position is {:#?}", self.player.position)
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
                            println!("Position is {:#?}", self.player.position)
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
                            println!("Position is {:#?}", self.player.position)
                        }
                    }
                } else {
                    self.player.direction = Direction::Left
                }
            }
        }
    }
}

impl Default for Map {
    fn default() -> Self {
        Map::new(DEFAULT_WIDTH, DEFAULT_HIGHT)
    }
}
