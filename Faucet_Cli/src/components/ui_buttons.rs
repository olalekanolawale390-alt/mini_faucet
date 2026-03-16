use crate::prelude::*;

pub fn render_buttons(frame: &mut Frame, area: Rect, app: &mut App) {
    app.button_areas.clear();
    
    match &app.state {
        AppState::Input => {
            let button_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(40),
                    Constraint::Length(20),
                    Constraint::Length(20),
                    Constraint::Percentage(40),
                ])
                .split(area);

            app.button_areas.push((button_layout[1].x, button_layout[1].y, button_layout[1].width, button_layout[1].height));
            app.button_areas.push((button_layout[2].x, button_layout[2].y, button_layout[2].width, button_layout[2].height));

            let submit_selected = app.selected_button == 0;
            let quit_selected = app.selected_button == 1;

            let submit_style = if submit_selected {
                Style::default().fg(Color::Black).bg(Color::Green).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Green).add_modifier(Modifier::DIM)
            };

            let quit_style = if quit_selected {
                Style::default().fg(Color::Black).bg(Color::Red).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Red).add_modifier(Modifier::DIM)
            };

            let submit_text = if submit_selected { "[ SUBMIT ]" } else { "  SUBMIT  " };
            let quit_text = if quit_selected { "[ QUIT ]" } else { "  QUIT  " };

            let submit_button = Paragraph::new(submit_text)
                .style(submit_style)
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::ALL).border_style(
                    if submit_selected {
                        Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(Color::DarkGray)
                    }
                ));

            let quit_button = Paragraph::new(quit_text)
                .style(quit_style)
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::ALL).border_style(
                    if quit_selected {
                        Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(Color::DarkGray)
                    }
                ));

            frame.render_widget(submit_button, button_layout[1]);
            frame.render_widget(quit_button, button_layout[2]);
        },
        AppState::Success(_) | AppState::Error(_) => {
            let button_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(40),
                    Constraint::Length(20),
                    Constraint::Length(20),
                    Constraint::Percentage(40),
                ])
                .split(area);

            app.button_areas.push((button_layout[1].x, button_layout[1].y, button_layout[1].width, button_layout[1].height));
            app.button_areas.push((button_layout[2].x, button_layout[2].y, button_layout[2].width, button_layout[2].height));

            let retry_selected = app.selected_button == 0;
            let quit_selected = app.selected_button == 1;

            let retry_style = if retry_selected {
                Style::default().fg(Color::Black).bg(Color::Yellow).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Yellow).add_modifier(Modifier::DIM)
            };

            let quit_style = if quit_selected {
                Style::default().fg(Color::Black).bg(Color::Red).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Red).add_modifier(Modifier::DIM)
            };

            let retry_text = if retry_selected { "[ RETRY ]" } else { "  RETRY  " };
            let quit_text = if quit_selected { "[ QUIT ]" } else { "  QUIT  " };

            let retry_button = Paragraph::new(retry_text)
                .style(retry_style)
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::ALL).border_style(
                    if retry_selected {
                        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(Color::DarkGray)
                    }
                ));

            let quit_button = Paragraph::new(quit_text)
                .style(quit_style)
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::ALL).border_style(
                    if quit_selected {
                        Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(Color::DarkGray)
                    }
                ));

            frame.render_widget(retry_button, button_layout[1]);
            frame.render_widget(quit_button, button_layout[2]);
        },
        _ => {}
    }
}
