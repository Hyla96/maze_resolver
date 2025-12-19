use iced::widget::canvas::{self, Cache};
use iced::{Color, Point, Renderer, Size};
use logic::{Direction, GameState};

const SQUARE_SIZE: u32 = 20;
const DIRECTION_INDICATOR_SIZE: u32 = 4;

pub struct PlayerView;

impl PlayerView {
    pub fn draw(
        game_state: &GameState,
        renderer: &Renderer,
        bounds_size: Size,
    ) -> canvas::Geometry {
        Cache::default().draw(renderer, bounds_size, |frame| {
            let square_size = SQUARE_SIZE as f32;
            let direction_size = DIRECTION_INDICATOR_SIZE as f32;

            let pos_x = game_state.player.position.x as f32 * square_size;
            let pos_y = game_state.player.position.y as f32 * square_size;

            // Player background
            frame.fill_rectangle(
                Point::new(pos_x, pos_y),
                Size::new(square_size, square_size),
                Color::from_rgb(1.0, 0.0, 0.0),
            );

            // Direction indicator
            let indicator_color = Color::from_rgb(0.9, 0.9, 0.0);
            match game_state.player.direction {
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
            let view_positions = game_state.get_player_view();
            for pos in view_positions {
                let view_x = pos.x as f32 * square_size;
                let view_y = pos.y as f32 * square_size;

                frame.fill_rectangle(
                    Point::new(view_x, view_y),
                    Size::new(square_size, square_size),
                    Color::from_rgba(1.0, 1.0, 0.0, 0.3),
                );
            }
        })
    }
}
