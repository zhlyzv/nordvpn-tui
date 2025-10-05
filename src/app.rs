use crate::nordvpn::NordVPN;
use crate::types::{ConnectionStatus, Country};
use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::DefaultTerminal;

pub struct App {
    /// Is the application running?
    pub running: bool,
    /// List of all available countries
    pub countries: Vec<Country>,
    /// Filtered list of countries based on search
    pub filtered_countries: Vec<Country>,
    /// Currently selected index in the filtered list
    pub selected_index: usize,
    /// Current connection status
    pub status: ConnectionStatus,
    /// Search/filter text
    pub filter: String,
    /// Error message to display (if any)
    pub error_message: Option<String>,
    /// Success message to display (if any)
    pub success_message: Option<String>,
    /// Whether we're in filter mode
    pub filter_mode: bool,
    /// Scroll state for the country list
    pub scroll_state: ratatui::widgets::ScrollbarState,
    /// List state for scrolling
    pub list_state: ratatui::widgets::ListState,
}

impl App {
    /// Construct a new instance of [`App`].
    pub fn new() -> Result<Self> {
        let countries = NordVPN::get_countries()?;
        let status = NordVPN::get_status().unwrap_or(ConnectionStatus::Disconnected);

        let filtered_countries = countries.clone();

        let scroll_state =
            ratatui::widgets::ScrollbarState::new(filtered_countries.len()).position(0);

        let mut list_state = ratatui::widgets::ListState::default();
        list_state.select(Some(0));

        Ok(Self {
            running: true,
            countries,
            filtered_countries,
            selected_index: 0,
            status,
            filter: String::new(),
            error_message: None,
            success_message: None,
            filter_mode: false,
            scroll_state,
            list_state,
        })
    }

    /// Run the application's main loop.
    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        while self.running {
            terminal.draw(|frame| crate::ui::render(&mut self, frame))?;
            self.handle_crossterm_events()?;
        }
        Ok(())
    }

    /// Update the filtered countries list based on current filter
    fn update_filter(&mut self) {
        if self.filter.is_empty() {
            self.filtered_countries = self.countries.clone();
        } else {
            let filter_lower = self.filter.to_lowercase();
            self.filtered_countries = self
                .countries
                .iter()
                .filter(|c| c.display_name.to_lowercase().contains(&filter_lower))
                .cloned()
                .collect();
        }

        // Reset selection if out of bounds
        if self.selected_index >= self.filtered_countries.len()
            && !self.filtered_countries.is_empty()
        {
            self.selected_index = self.filtered_countries.len() - 1;
        }

        // Update scrollbar state
        self.scroll_state = self
            .scroll_state
            .content_length(self.filtered_countries.len());
        self.list_state.select(Some(self.selected_index));
    }

    /// Refresh the connection status
    pub fn refresh_status(&mut self) {
        match NordVPN::get_status() {
            Ok(status) => self.status = status,
            Err(e) => self.error_message = Some(format!("Failed to get status: {}", e)),
        }
    }

    /// Connect to the selected country
    fn connect_selected(&mut self) {
        if self.filtered_countries.is_empty() {
            self.error_message = Some("No country selected".to_string());
            return;
        }

        let country = &self.filtered_countries[self.selected_index];
        self.status = ConnectionStatus::Connecting;

        match NordVPN::connect(&country.name) {
            Ok(_) => {
                self.success_message = Some(format!("Connecting to {}...", country.display_name));
                // Refresh status after a moment
                self.refresh_status();
            }
            Err(e) => {
                self.error_message = Some(format!("Failed to connect: {}", e));
                self.status = ConnectionStatus::Disconnected;
            }
        }
    }

    /// Disconnect from VPN
    fn disconnect(&mut self) {
        match NordVPN::disconnect() {
            Ok(_) => {
                self.success_message = Some("Disconnected".to_string());
                self.status = ConnectionStatus::Disconnected;
            }
            Err(e) => {
                self.error_message = Some(format!("Failed to disconnect: {}", e));
            }
        }
    }

    /// Navigate up in the country list
    fn move_up(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
            self.scroll_state = self.scroll_state.position(self.selected_index);
            self.list_state.select(Some(self.selected_index));
        }
    }

    /// Navigate down in the country list
    fn move_down(&mut self) {
        if self.selected_index + 1 < self.filtered_countries.len() {
            self.selected_index += 1;
            self.scroll_state = self.scroll_state.position(self.selected_index);
            self.list_state.select(Some(self.selected_index));
        }
    }

    /// Handle crossterm events
    fn handle_crossterm_events(&mut self) -> Result<()> {
        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => self.on_key_event(key),
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
            _ => {}
        }
        Ok(())
    }

    /// Handle key events
    fn on_key_event(&mut self, key: KeyEvent) {
        // Clear messages on any key press
        self.error_message = None;
        self.success_message = None;

        if self.filter_mode {
            match key.code {
                KeyCode::Esc => {
                    self.filter_mode = false;
                }
                KeyCode::Enter => {
                    self.filter_mode = false;
                }
                KeyCode::Char(c) => {
                    self.filter.push(c);
                    self.update_filter();
                }
                KeyCode::Backspace => {
                    self.filter.pop();
                    self.update_filter();
                }
                KeyCode::Up | KeyCode::Down => {
                    // Allow navigation while filtering
                    self.filter_mode = false;
                    if key.code == KeyCode::Up {
                        self.move_up();
                    } else {
                        self.move_down();
                    }
                }
                _ => {}
            }
        } else {
            match (key.modifiers, key.code) {
                (_, KeyCode::Esc | KeyCode::Char('q'))
                | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => self.quit(),
                (_, KeyCode::Up | KeyCode::Char('k')) => self.move_up(),
                (_, KeyCode::Down | KeyCode::Char('j')) => self.move_down(),
                (_, KeyCode::Enter) => self.connect_selected(),
                (KeyModifiers::CONTROL, KeyCode::Char('d') | KeyCode::Char('D')) => {
                    self.disconnect()
                }
                (KeyModifiers::CONTROL, KeyCode::Char('r') | KeyCode::Char('R')) => {
                    self.refresh_status()
                }
                (_, KeyCode::Char('/')) => {
                    self.filter_mode = true;
                }
                (KeyModifiers::NONE, KeyCode::Char(c)) if c.is_alphanumeric() => {
                    // Start filtering on any alphanumeric character
                    self.filter_mode = true;
                    self.filter.push(c);
                    self.update_filter();
                }
                (KeyModifiers::NONE, KeyCode::Backspace) => {
                    // Backspace in normal mode enters filter mode and deletes
                    if !self.filter.is_empty() {
                        self.filter_mode = true;
                        self.filter.pop();
                        self.update_filter();
                    }
                }
                _ => {}
            }
        }
    }

    /// Set running to false to quit the application.
    fn quit(&mut self) {
        self.running = false;
    }
}
