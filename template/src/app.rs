use mousefood::prelude::*;
use mousefood::ratatui::widgets::{Block, Paragraph};

use crate::button::Button;

#[derive(Default)]
pub struct App {
    button_pressed: Option<Button>,
}

impl App {
    pub fn draw(&self, frame: &mut Frame) {
        let text = Line::from("Ratatui on embedded devices!").dark_gray();

        let button_text = match self.button_pressed {
            Some(button) => Line::from(vec![
                "You pressed: ".dark_gray(),
                button.to_string().yellow(),
            ]),
            None => Line::from("Press a button...").dark_gray(),
        };

        let paragraph = Paragraph::new(Text::from(vec![text, button_text]));

        let bordered_block = Block::bordered()
            .border_style(Style::new().yellow())
            .title("Mousefood");

        frame.render_widget(paragraph.block(bordered_block), frame.area());
    }

    pub fn handle_press(&mut self, button: Button) {
        self.button_pressed = Some(button);
    }
}
