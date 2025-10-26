use mousefood::prelude::*;
use mousefood::ratatui::widgets::{Block, Borders, BorderType, Gauge, List, ListItem, Paragraph, Sparkline, Tabs};
use ratatui_wlan_scanner::button::{Button, ButtonPressType};
use ratatui_wlan_scanner::setup::App;
use rand::Rng;

/// Application state with multi-tab navigation.
#[derive(Default)]
pub struct AppState {
    /// Currently selected tab (0 = Main, 1 = Stats, 2 = Settings)
    selected_tab: usize,
    /// Scroll offset for list navigation
    scroll_offset: usize,
    /// Sample data (replace with your actual data)
    data: Vec<String>,
    /// Status message shown in footer
    status_message: String,
    /// Tracks the last button that was pressed
    last_button: Option<Button>,
    stats_tab_counter: u16,
}

impl AppState {
    /// Create a new app state with default values
    pub fn new() -> Self {
        Self {
            selected_tab: 0,
            scroll_offset: 0,
            data: vec![
                "Item 1".to_string(),
                "Item 2".to_string(),
                "Item 3".to_string(),
                "Item 4".to_string(),
                "Item 5".to_string(),
            ],
            status_message: "Ready".to_string(),
            last_button: None,
            stats_tab_counter: 0,
        }
    }

    /// Move to the next tab
    fn next_tab(&mut self) {
        self.selected_tab = (self.selected_tab + 1) % 3;
        self.status_message = format!("Switched to tab {}", self.selected_tab + 1);
    }

    /// Perform action based on current tab
    fn perform_action(&mut self) {
        match self.selected_tab {
            0 => {
                // Main tab: scroll down
                self.scroll_offset = self.scroll_offset.saturating_add(1);
                self.status_message = format!("Scrolled to {}", self.scroll_offset);
            }
            1 => {
                // Stats tab: refresh data
                self.status_message = "Stats refreshed!".to_string();
                self.stats_tab_counter = (self.stats_tab_counter + 5) % 100;
            }
            2 => {
                // Settings tab: some action
                self.status_message = "Settings updated!".to_string();
            }
            _ => {}
        }
    }
}

/// The main application trait implementation.
impl App for AppState {
    /// Draw the UI frame.
    fn draw(&self, frame: &mut Frame) {
        // Create main layout: header, content, footer
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),  // Header with tabs
                Constraint::Min(0),     // Main content area
                Constraint::Length(3),  // Footer with status
            ])
            .split(frame.area());

        // Render header with tabs
        self.draw_header(frame, chunks[0]);

        // Render content based on selected tab
        match self.selected_tab {
            0 => self.draw_main_tab(frame, chunks[1]),
            1 => self.draw_stats_tab(frame, chunks[1]),
            2 => self.draw_settings_tab(frame, chunks[1]),
            _ => {}
        }

        // Render footer
        self.draw_footer(frame, chunks[2]);
    }

    /// Handle button press events.
    fn handle_press(&mut self, button: Button) {
        self.last_button = Some(button);

        match button {
            Button::Button1(ButtonPressType::Short) => self.next_tab(),
            Button::Button2(ButtonPressType::Short) => self.perform_action(),
            _ => {}
        }
    }
}

impl AppState {
    /// Draw the header with tab navigation
    fn draw_header(&self, frame: &mut Frame, area: Rect) {
        let titles = vec!["Main", "Stats", "Settings"];
        let tabs = Tabs::new(titles)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .title(" 🐀 ESP32 Workshop ")
                    .title_alignment(Alignment::Center)
                    .border_style(Style::new().yellow()),
            )
            .select(self.selected_tab)
            .style(Style::default().white())
            .highlight_style(Style::default().yellow().bold());

        frame.render_widget(tabs, area);
    }

    /// Draw the main tab with a list view
    fn draw_main_tab(&self, frame: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
            .split(area);

        // Main list
        let items: Vec<ListItem> = self
            .data
            .iter()
            .enumerate()
            .map(|(i, item)| {
                let content = Line::from(vec![
                    Span::styled(
                        format!("{:2}. ", i + 1),
                        Style::default().dark_gray(),
                    ),
                    Span::raw(item),
                ]);
                ListItem::new(content)
            })
            .collect();

        let list = List::new(items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .title(" Data View ")
                    .border_style(Style::new().cyan()),
            )
            .style(Style::default().white());

        frame.render_widget(list, chunks[0]);

        // Info panel
        let info_text = vec![
            Line::from(vec![
                "Total Items: ".dark_gray(),
                self.data.len().to_string().cyan(),
            ]),
            Line::from(vec![
                "Scroll Offset: ".dark_gray(),
                self.scroll_offset.to_string().cyan(),
            ]),
            Line::from(""),
            Line::from("BTN2 to scroll down".dark_gray()),
        ];

        let info = Paragraph::new(info_text)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .title(" Info ")
                    .border_style(Style::new().cyan()),
            );

        frame.render_widget(info, chunks[1]);
    }

    /// Draw the stats tab with charts and gauges
    fn draw_stats_tab(&self, frame: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Length(10),
                Constraint::Min(0),
            ])
            .split(area);

        // Gauge example
        let gauge = Gauge::default()
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .title(" Progress ")
                    .border_style(Style::new().green()),
            )
            .gauge_style(Style::default().green())
            .percent(self.stats_tab_counter);

        frame.render_widget(gauge, chunks[0]);

        // Sparkline for time-series data
        //let data: Vec<u64> = vec![0, 2, 3, 4, 1, 4, 10, 15, 8, 10, 12, 15, 20, 18, 15, 12];
        let mut rng = rand::thread_rng();
        let data: Vec<u64> = (0..16)
        .map(|_| rng.gen_range(0..20))
        .collect();

        let sparkline = Sparkline::default()
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .title(" Activity ")
                    .border_style(Style::new().yellow()),
            )
            .data(&data)
            .style(Style::default().yellow());

        frame.render_widget(sparkline, chunks[1]);

        // Stats text
        let stats_text = vec![
            Line::from(vec![
                "CPU Usage: ".dark_gray(),
                "45%".green(),
            ]),
            Line::from(vec![
                "Memory Free: ".dark_gray(),
                "128 KB".cyan(),
            ]),
            Line::from(vec![
                "Uptime: ".dark_gray(),
                "1h 23m".yellow(),
            ]),
            Line::from(""),
            Line::from("System running smoothly ✓".green()),
        ];

        let stats = Paragraph::new(stats_text)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .title(" System Stats ")
                    .border_style(Style::new().green()),
            );

        frame.render_widget(stats, chunks[2]);
    }

    /// Draw the settings/about tab
    fn draw_settings_tab(&self, frame: &mut Frame, area: Rect) {
        let text = vec![
            Line::from(""),
            Line::from("ESP32 Workshop Project".yellow().bold()),
            Line::from(""),
            Line::from("Built with:"),
            Line::from("  • Rust 🦀"),
            Line::from("  • Ratatui 🐀"),
            Line::from("  • esp-idf-hal"),
            Line::from(""),
            Line::from("\"Anyone can code!\"".cyan().italic()),
            Line::from(""),
            Line::from("Made with ❤️ at the workshop".red()),
        ];

        let paragraph = Paragraph::new(text)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .title(" About ")
                    .border_style(Style::new().magenta()),
            )
            .alignment(Alignment::Center);

        frame.render_widget(paragraph, area);
    }

    /// Draw the footer with status and button hints
    fn draw_footer(&self, frame: &mut Frame, area: Rect) {
        let button_info = match self.last_button {
            Some(Button::Button1(ButtonPressType::Short)) => " [BTN1] ",
            Some(Button::Button2(ButtonPressType::Short)) => " [BTN2] ",
            _ => "",
        };

        //let button_info = "";

        let text = vec![Line::from(vec![
            /*"BTN1: ".yellow(),
            "Next Tab  ".white(),
            "BTN2: ".yellow(),
            "Action  ".white(),
            "| ".dark_gray(),*/
            button_info.cyan(),
            self.status_message.as_str().green(),
        ])];

        let footer = Paragraph::new(text)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::new().dark_gray()),
            )
            .alignment(Alignment::Center);

        frame.render_widget(footer, area);
    }
}

fn main() {
    AppState::new().run()
}
