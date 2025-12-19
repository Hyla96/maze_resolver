mod map;

use iced::Result;
use map::Map;

fn main() -> Result {
    iced::run(Map::update, Map::view)
}
