use iced::widget::{button, column, container, row, Column};

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Reset,
    Up,
    Down,
    Right,
    Left,
}

const BUTTON_SIZE: u32 = 30;

pub struct ControlsView;

impl ControlsView {
    pub fn view() -> Column<'static, Message> {
        column![
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
        ]
    }
}
