use crate::app::App;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

pub fn render(app: &mut App, frame: &mut Frame) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Status bar
            Constraint::Length(3), // Filter bar
            Constraint::Min(0),    // Country list
            Constraint::Length(3), // Help/message bar
        ])
        .split(frame.area());

    render_status(app, frame, chunks[0]);
    render_filter(app, frame, chunks[1]);
    render_country_list(app, frame, chunks[2]);
    render_help(app, frame, chunks[3]);
}

fn render_status(app: &App, frame: &mut Frame, area: Rect) {
    let status_text = app.status.to_string();
    let (status_color, status_symbol) = match &app.status {
        crate::types::ConnectionStatus::Connected { .. } => (Color::Green, "●"),
        crate::types::ConnectionStatus::Disconnected => (Color::Red, "●"),
        crate::types::ConnectionStatus::Connecting => (Color::Yellow, "◐"),
    };

    let status_line = Line::from(vec![
        Span::styled(status_symbol, Style::default().fg(status_color)),
        Span::raw(" "),
        Span::styled(
            status_text,
            Style::default()
                .fg(status_color)
                .add_modifier(Modifier::BOLD),
        ),
    ]);

    let status_widget =
        Paragraph::new(status_line).block(Block::default().borders(Borders::ALL).title(" Status "));

    frame.render_widget(status_widget, area);
}

fn render_filter(app: &App, frame: &mut Frame, area: Rect) {
    let filter_text = if app.filter_mode {
        format!("/{}_", app.filter)
    } else if app.filter.is_empty() {
        "Type to filter countries".to_string()
    } else {
        format!("Filter: {}", app.filter)
    };

    let filter_style = if app.filter_mode {
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default()
    };

    let filter_widget = Paragraph::new(filter_text)
        .style(filter_style)
        .block(Block::default().borders(Borders::ALL).title(" Filter "));

    frame.render_widget(filter_widget, area);
}

fn render_country_list(app: &App, frame: &mut Frame, area: Rect) {
    let items: Vec<ListItem> = app
        .filtered_countries
        .iter()
        .enumerate()
        .map(|(i, country)| {
            let content = if i == app.selected_index {
                format!("▶ {}", country.display_name)
            } else {
                format!("  {}", country.display_name)
            };

            let style = if i == app.selected_index {
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };

            ListItem::new(content).style(style)
        })
        .collect();

    let title = if app.filtered_countries.is_empty() {
        " Countries (No matches) "
    } else {
        " Countries "
    };

    let list = List::new(items).block(Block::default().borders(Borders::ALL).title(title));

    frame.render_widget(list, area);
}

fn render_help(app: &App, frame: &mut Frame, area: Rect) {
    let help_text = if let Some(error) = &app.error_message {
        Line::from(vec![
            Span::styled("✗ ", Style::default().fg(Color::Red)),
            Span::styled(error, Style::default().fg(Color::Red)),
        ])
    } else if let Some(success) = &app.success_message {
        Line::from(vec![
            Span::styled("✓ ", Style::default().fg(Color::Green)),
            Span::styled(success, Style::default().fg(Color::Green)),
        ])
    } else if app.filter_mode {
        Line::from("Type to filter | ↑/↓: Navigate | Enter/Esc: Exit filter mode")
    } else {
        Line::from(
            "Type/↑/↓/j/k: Filter/Navigate | Enter: Connect | Ctrl+D: Disconnect | Ctrl+R: Refresh | q/Esc: Quit",
        )
    };

    let help_widget = Paragraph::new(help_text).block(Block::default().borders(Borders::ALL));

    frame.render_widget(help_widget, area);
}
