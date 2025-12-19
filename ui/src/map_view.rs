use iced::widget::canvas::{self, Cache};
use iced::{Color, Point, Renderer, Size};
use logic::{GameState, TileType};
use std::time::Instant;

const SQUARE_SIZE: u32 = 20;

pub struct MapView;

impl MapView {
    pub fn draw(
        game_state: &GameState,
        tiles_cache: &Cache,
        renderer: &Renderer,
        bounds_size: Size,
    ) -> canvas::Geometry {
        tiles_cache.draw(renderer, bounds_size, |frame| {
            let compute_start = Instant::now();
            let square_size = SQUARE_SIZE as f32;

            for y in 0..game_state.height {
                for x in 0..game_state.width {
                    let tile = &game_state.tiles[x][y];
                    let pos_x = x as f32 * square_size;
                    let pos_y = y as f32 * square_size;

                    let color = match tile.tile_type {
                        TileType::Goal => Color::from_rgb(0.0, 1.0, 0.0),
                        TileType::Walkable => Color::WHITE,
                        TileType::Wall => Color::BLACK,
                    };
                    frame.fill_rectangle(
                        Point::new(pos_x, pos_y),
                        Size::new(square_size, square_size),
                        color,
                    );
                }
            }
            println!(
                "Tiles cache rebuild | Compute: {:.3}ms",
                compute_start.elapsed().as_secs_f64() * 1000.0
            );
        })
    }
}
