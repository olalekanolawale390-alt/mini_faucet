use crate::prelude::*;
use crate::components::ui_buttons::render_buttons;

pub fn render(frame: &mut Frame, app: &mut App) {
    let show_input = app.selected_button == 0 || app.state != AppState::Input;
    
    let chunks = if show_input {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(7),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Min(5),
                Constraint::Length(1),
            ])
            .split(frame.area())
    } else {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(7),
                Constraint::Length(3),
                Constraint::Min(5),
                Constraint::Length(1),
            ])
            .split(frame.area())
    };

    render_header(frame, chunks[0]);
    
    if show_input {
        render_input(frame, chunks[1], app);
        render_buttons(frame, chunks[2], app);
        render_status(frame, chunks[3], app);
        render_footer(frame, chunks[4]);
    } else {
        render_buttons(frame, chunks[1], app);
        render_status(frame, chunks[2], app);
        render_footer(frame, chunks[3]);
    }
}

fn render_header(frame: &mut Frame, area: Rect) {
    let header_text = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled("╔═══════════════════════════════════════╗", Style::default().fg(Color::Green)),
        ]),
        Line::from(vec![
            Span::styled("║  ", Style::default().fg(Color::Green)),
            Span::styled("SEPOLIA FAUCET", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            Span::styled("  ║", Style::default().fg(Color::Green)),
        ]),
        Line::from(vec![
            Span::styled("╚═══════════════════════════════════════╝", Style::default().fg(Color::Green)),
        ]),
    ];

    let header = Paragraph::new(header_text)
        .alignment(Alignment::Center);
    frame.render_widget(header, area);
}

fn render_input(frame: &mut Frame, area: Rect, app: &mut App) {
    let input_enabled = app.state == AppState::Input;
    let border_color = if input_enabled { Color::Green } else { Color::DarkGray };
    
    let input_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color))
        .title(" Address ");

    let input_text = if input_enabled && app.cursor_visible {
        let mut text = app.input.clone();
        text.insert(app.cursor_position, '█');
        text
    } else if input_enabled {
        let mut text = app.input.clone();
        text.insert(app.cursor_position, ' ');
        text
    } else {
        app.input.clone()
    };

    let input_paragraph = Paragraph::new(input_text)
        .style(Style::default().fg(Color::Green))
        .block(input_block);

    frame.render_widget(input_paragraph, area);
}

fn render_status(frame: &mut Frame, area: Rect, app: &mut App) {
    match &app.state {
        AppState::Input => {
            let lines = vec![
                Line::from(""),
                Line::from(vec![
                    Span::styled("> ", Style::default().fg(Color::Green)),
                    Span::styled("Enter your Ethereum address above", Style::default().fg(Color::DarkGray)),
                ]),
                Line::from(vec![
                    Span::styled("> ", Style::default().fg(Color::Green)),
                    Span::styled("Press ENTER to submit", Style::default().fg(Color::DarkGray)),
                ]),
            ];
            let status_block = Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Green))
                .title(" Status ");
            let status_paragraph = Paragraph::new(lines)
                .block(status_block)
                .wrap(Wrap { trim: true });
            frame.render_widget(status_paragraph, area);
        },
        AppState::Connecting => {
            let inner_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Min(1),
                ])
                .split(area);

            let status_block = Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan))
                .title(" Status ");
            frame.render_widget(status_block, area);

            let msg_area = Rect {
                x: inner_chunks[0].x + 2,
                y: inner_chunks[0].y + 1,
                width: inner_chunks[0].width.saturating_sub(4),
                height: 1,
            };
            let msg = Paragraph::new(vec![
                Line::from(vec![
                    Span::styled("[", Style::default().fg(Color::Cyan)),
                    Span::styled("◉", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
                    Span::styled("] ", Style::default().fg(Color::Cyan)),
                    Span::styled("Establishing connection...", Style::default().fg(Color::Cyan)),
                ]),
            ]);
            frame.render_widget(msg, msg_area);

            let gauge_area = Rect {
                x: inner_chunks[1].x + 2,
                y: inner_chunks[1].y,
                width: inner_chunks[1].width.saturating_sub(4),
                height: 1,
            };
            let gauge = Gauge::default()
                .gauge_style(Style::default().fg(Color::Cyan).bg(Color::Black))
                .ratio(app.progress.min(1.0));
            frame.render_widget(gauge, gauge_area);
        },
        AppState::Connected => {
            let lines = vec![
                Line::from(""),
                Line::from(vec![
                    Span::styled("[", Style::default().fg(Color::Green)),
                    Span::styled("✓", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
                    Span::styled("] ", Style::default().fg(Color::Green)),
                    Span::styled("Request received!", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
                ]),
                Line::from(""),
                Line::from(vec![
                    Span::styled("⚡ ", Style::default().fg(Color::Yellow)),
                    Span::styled("Processing blockchain transaction...", Style::default().fg(Color::DarkGray)),
                ]),
            ];
            let status_block = Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Green))
                .title(" Status ");
            let status_paragraph = Paragraph::new(lines)
                .block(status_block)
                .wrap(Wrap { trim: true });
            frame.render_widget(status_paragraph, area);
        },
        AppState::Claiming => {
            let inner_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Min(1),
                ])
                .split(area);

            let status_block = Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Yellow))
                .title(" Status ");
            frame.render_widget(status_block, area);

            let msg_area = Rect {
                x: inner_chunks[0].x + 2,
                y: inner_chunks[0].y + 1,
                width: inner_chunks[0].width.saturating_sub(4),
                height: 1,
            };
            let msg = Paragraph::new(vec![
                Line::from(vec![
                    Span::styled("[", Style::default().fg(Color::Yellow)),
                    Span::styled("⛓", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                    Span::styled("] ", Style::default().fg(Color::Yellow)),
                    Span::styled("Submitting transaction to blockchain...", Style::default().fg(Color::Yellow)),
                ]),
            ]);
            frame.render_widget(msg, msg_area);

            let gauge_area = Rect {
                x: inner_chunks[1].x + 2,
                y: inner_chunks[1].y,
                width: inner_chunks[1].width.saturating_sub(4),
                height: 1,
            };
            let gauge = Gauge::default()
                .gauge_style(Style::default().fg(Color::Yellow).bg(Color::Black))
                .ratio(app.progress.min(1.0));
            frame.render_widget(gauge, gauge_area);

            let hint_area = Rect {
                x: inner_chunks[2].x + 2,
                y: inner_chunks[2].y,
                width: inner_chunks[2].width.saturating_sub(4),
                height: 1,
            };
            let hint = Paragraph::new(vec![
                Line::from(vec![
                    Span::styled("⏳ Waiting for blockchain confirmation...", Style::default().fg(Color::DarkGray)),
                ]),
            ]);
            frame.render_widget(hint, hint_area);
        },
        AppState::Success(msg) => {
            let lines = vec![
                Line::from(""),
                Line::from(vec![
                    Span::styled("[", Style::default().fg(Color::Green)),
                    Span::styled("✓", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
                    Span::styled("] ", Style::default().fg(Color::Green)),
                    Span::styled("SUCCESS", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
                ]),
                Line::from(""),
                Line::from(vec![
                    Span::styled(msg.as_str(), Style::default().fg(Color::Green)),
                ]),
                Line::from(""),
                Line::from(vec![
                    Span::styled("> Press 'r' to claim again or 'q' to quit", Style::default().fg(Color::DarkGray)),
                ]),
            ];
            let status_block = Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Green))
                .title(" Status ");
            let status_paragraph = Paragraph::new(lines)
                .block(status_block)
                .wrap(Wrap { trim: true });
            frame.render_widget(status_paragraph, area);
        },
        AppState::Error(msg) => {
            let lines = vec![
                Line::from(""),
                Line::from(vec![
                    Span::styled("[", Style::default().fg(Color::Red)),
                    Span::styled("✗", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
                    Span::styled("] ", Style::default().fg(Color::Red)),
                    Span::styled("ERROR", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
                ]),
                Line::from(""),
                Line::from(vec![
                    Span::styled(msg.as_str(), Style::default().fg(Color::Red)),
                ]),
                Line::from(""),
                Line::from(vec![
                    Span::styled("> Press 'r' to try again or 'q' to quit", Style::default().fg(Color::DarkGray)),
                ]),
            ];
            let status_block = Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Red))
                .title(" Status ");
            let status_paragraph = Paragraph::new(lines)
                .block(status_block)
                .wrap(Wrap { trim: true });
            frame.render_widget(status_paragraph, area);
        },
    }
}

fn render_footer(frame: &mut Frame, area: Rect) {
    let footer = Paragraph::new(vec![
        Line::from(vec![
            Span::styled("ESC", Style::default().fg(Color::Yellow)),
            Span::styled(": quit  ", Style::default().fg(Color::DarkGray)),
            Span::styled("q", Style::default().fg(Color::Yellow)),
            Span::styled(": quit (results screen)", Style::default().fg(Color::DarkGray)),
        ]),
    ])
    .alignment(Alignment::Center);

    frame.render_widget(footer, area);
}
