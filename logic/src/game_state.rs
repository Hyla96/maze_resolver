use rand::prelude::*;
use crate::types::{Tile, Position, Direction, Player, Action};

#[derive(Debug)]
pub struct GameState {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<Vec<Tile>>,
    pub player: Player,
    pub goal: Position,
    pub game_over: bool,
}

impl GameState {
    pub fn new(width: usize, height: usize) -> Self {
        let mut rng = rand::rng();

        let mut tiles = Self::create_maze(width, height);

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

        GameState {
            width,
            height,
            player,
            goal,
            game_over: false,
            tiles,
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

    pub fn apply_action(&mut self, action: Action) {
        if self.game_over {
            return;
        }

        match action {
            Action::Up => {
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
            Action::Down => {
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
            Action::Right => {
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
            Action::Left => {
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

    pub fn reset(&mut self) {
        *self = GameState::new(self.width, self.height);
    }
}
