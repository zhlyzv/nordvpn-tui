use crate::app::App;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols::{border, scrollbar},
    text::{Line, Span},
    widgets::{
        Block, BorderType, Borders, List, ListItem, Paragraph, Scrollbar, ScrollbarOrientation,
    },
};

pub fn render(app: &mut App, frame: &mut Frame) {
    // Outer wrapper block
    let outer_block = Block::default()
        .title(Span::styled(
            " NordVPN ",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_set(border::ROUNDED)
        .border_type(BorderType::QuadrantInside)
        .border_style(Style::default().fg(Color::LightCyan));

    let inner_area = outer_block.inner(frame.area());
    frame.render_widget(outer_block, frame.area());

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Status bar
            Constraint::Length(3), // Filter bar
            Constraint::Min(0),    // Country list
            Constraint::Length(3), // Help/message bar
        ])
        .split(inner_area);

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

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Thick)
        .border_style(Style::default().fg(status_color))
        .title(Span::styled(
            " Status ",
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ));

    let status_widget = Paragraph::new(status_line).block(block);

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

    let (text_style, border_color) = if app.filter_mode {
        (
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
            Color::Yellow,
        )
    } else if !app.filter.is_empty() {
        (Style::default().fg(Color::Cyan), Color::Cyan)
    } else {
        (Style::default().fg(Color::DarkGray), Color::DarkGray)
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .border_set(border::PLAIN)
        .border_style(Style::default().fg(border_color))
        .title(Span::styled(
            " Filter ",
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ));

    let filter_widget = Paragraph::new(filter_text).style(text_style).block(block);

    frame.render_widget(filter_widget, area);
}

fn render_country_list(app: &mut App, frame: &mut Frame, area: Rect) {
    // Get connected country name if any
    let connected_country = match &app.status {
        crate::types::ConnectionStatus::Connected { country, .. } => Some(country.to_lowercase()),
        _ => None,
    };

    let items: Vec<ListItem> = app
        .filtered_countries
        .iter()
        .enumerate()
        .map(|(i, country)| {
            let is_connected = connected_country
                .as_ref()
                .map(|c| country.display_name.to_lowercase() == *c)
                .unwrap_or(false);

            let content = if i == app.selected_index && is_connected {
                format!("▶ {} ●", country.display_name)
            } else if i == app.selected_index {
                format!("▶ {}", country.display_name)
            } else if is_connected {
                format!("  {} ●", country.display_name)
            } else {
                format!("  {}", country.display_name)
            };

            let style = if i == app.selected_index {
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD)
            } else if is_connected {
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Gray)
            };

            ListItem::new(content).style(style)
        })
        .collect();

    let (title, border_color) = if app.filtered_countries.is_empty() {
        (" Countries (No matches) ", Color::Red)
    } else {
        (" Countries ", Color::Blue)
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .border_set(border::PLAIN)
        .border_style(Style::default().fg(border_color))
        .title(Span::styled(
            title,
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ));

    let list = List::new(items).block(block);

    frame.render_widget(list, area);

    // Render scrollbar
    let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
        .symbols(scrollbar::VERTICAL)
        // .begin_symbol(Some("↑"))
        // .end_symbol(Some("↓"))
        .style(Style::default().fg(Color::Cyan));

    frame.render_stateful_widget(
        scrollbar,
        area.inner(ratatui::layout::Margin {
            vertical: 2,
            horizontal: 2,
        }),
        &mut app.scroll_state,
    );
}

fn render_help(app: &App, frame: &mut Frame, area: Rect) {
    let help_text = if let Some(error) = &app.error_message {
        Line::from(vec![
            Span::styled(
                "✗ ",
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            ),
            Span::styled(error, Style::default().fg(Color::Red)),
        ])
    } else if let Some(success) = &app.success_message {
        Line::from(vec![
            Span::styled(
                "✓ ",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(success, Style::default().fg(Color::Green)),
        ])
    } else if app.filter_mode {
        Line::from(vec![
            Span::styled("Type", Style::default().fg(Color::Yellow)),
            Span::raw(" to filter | "),
            Span::styled("↑/↓", Style::default().fg(Color::Cyan)),
            Span::raw(": Navigate | "),
            Span::styled("Enter/Esc", Style::default().fg(Color::Magenta)),
            Span::raw(": Exit filter mode"),
        ])
    } else {
        Line::from(vec![
            Span::styled("↑/↓/j/k", Style::default().fg(Color::Cyan)),
            Span::raw(": Navigate | "),
            Span::styled("Enter", Style::default().fg(Color::Green)),
            Span::raw(": Connect | "),
            Span::styled("Ctrl+D", Style::default().fg(Color::Red)),
            Span::raw(": Disconnect | "),
            Span::styled("Ctrl+R", Style::default().fg(Color::Yellow)),
            Span::raw(": Refresh | "),
            Span::styled("Esc/q", Style::default().fg(Color::Magenta)),
            Span::raw(": Quit"),
        ])
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .border_set(border::PLAIN)
        .border_style(Style::default().fg(Color::DarkGray))
        .title(Span::styled(
            " Help ",
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ));

    let help_widget = Paragraph::new(help_text).block(block);

    frame.render_widget(help_widget, area);
}
