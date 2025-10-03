use embedded_graphics_simulator::{OutputSettings, SimulatorDisplay, SimulatorEvent, Window};
use mousefood::embedded_graphics::geometry;
use mousefood::prelude::*;

use anyhow::Error;
use mousefood::ratatui::widgets::{Block, Paragraph, Wrap};
use mousefood::ratatui::{Frame, Terminal};

fn draw(frame: &mut Frame) {
    let text = "Ratatui on embedded devices!";
    let paragraph = Paragraph::new(text.dark_gray()).wrap(Wrap { trim: true });
    let bordered_block = Block::bordered()
        .border_style(Style::new().yellow())
        .title("Mousefood");
    frame.render_widget(paragraph.block(bordered_block), frame.area());
}

fn main() -> Result<(), Error> {
    // Create window where the simulation will happen
    let mut simulator_window = Window::new(
        "mousefood simulator",
        &OutputSettings {
            scale: 4,
            max_fps: 30,
            ..Default::default()
        },
    );

    // Define properties of the display which will be shown in the simulator window
    let mut display = SimulatorDisplay::<Bgr565>::new(geometry::Size::new(128, 64));

    let backend_config = EmbeddedBackendConfig {
        // Define how to display newly rendered widgets to the simulator window
        flush_callback: Box::new(move |display| {
            simulator_window.update(display);
            if simulator_window.events().any(|e| e == SimulatorEvent::Quit) {
                panic!("simulator window closed");
            }
        }),
        ..Default::default()
    };
    let backend: EmbeddedBackend<SimulatorDisplay<_>, _> =
        EmbeddedBackend::new(&mut display, backend_config);

    // Start ratatui with our simulator backend
    let mut terminal = Terminal::new(backend)?;

    // Run an infinite loop, where widgets will be rendered
    loop {
        terminal.draw(draw)?;
    }
}
