use crate::prelude::*;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> io::Result<()> {
    let mut last_tick = std::time::Instant::now();
    let tick_rate = Duration::from_millis(500);

    loop {
        terminal.draw(|f| ui::render(f, app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            match event::read()? {
                Event::Mouse(mouse_event) => {
                    if let crossterm::event::MouseEventKind::Down(crossterm::event::MouseButton::Left) = mouse_event.kind {
                        if let Some(button_idx) = app.check_button_click(mouse_event.column, mouse_event.row) {
                            app.selected_button = button_idx;
                            
                            if app.state == AppState::Input {
                                if button_idx == 0 && !app.input.is_empty() {
                                    app.state = AppState::Connecting;
                                    app.progress = 0.0;
                                    
                                    for i in 0..=10 {
                                        app.progress = i as f64 / 10.0;
                                        terminal.draw(|f| ui::render(f, app))?;
                                        std::thread::sleep(Duration::from_millis(50));
                                    }
                                    
                                    let address = app.input.clone();
                                    let result = tokio::runtime::Runtime::new()
                                        .unwrap()
                                        .block_on(async {
                                            crate::handlers::claim::claim_with_address(address).await
                                        });

                                    match result {
                                        Responder::Success(_) => {
                                            app.state = AppState::Connected;
                                            terminal.draw(|f| ui::render(f, app))?;
                                            std::thread::sleep(Duration::from_millis(500));
                                            
                                            app.state = AppState::Claiming;
                                            app.progress = 0.0;
                                            
                                            for i in 0..=20 {
                                                app.progress = i as f64 / 20.0;
                                                terminal.draw(|f| ui::render(f, app))?;
                                                std::thread::sleep(Duration::from_millis(100));
                                            }
                                            
                                            if let Responder::Success(msg) = result {
                                                app.state = AppState::Success(msg);
                                            }
                                        }
                                        Responder::Error(msg) => {
                                            app.state = AppState::Error(msg);
                                        }
                                    }
                                } else if button_idx == 1 {
                                    return Ok(());
                                }
                            } else if matches!(app.state, AppState::Success(_) | AppState::Error(_)) {
                                if button_idx == 0 {
                                    app.reset();
                                } else {
                                    return Ok(());
                                }
                            }
                        }
                    }
                }
                Event::Key(key) if key.kind == KeyEventKind::Press => {
                    match app.state {
                        AppState::Input => {
                            match key.code {
                                KeyCode::Esc => {
                                    return Ok(());
                                }
                                KeyCode::Tab => {
                                    app.select_next_button();
                                }
                                KeyCode::BackTab => {
                                    app.select_prev_button();
                                }
                                KeyCode::Char(c) => {
                                    app.handle_char(c);
                                }
                                KeyCode::Backspace => {
                                    app.handle_backspace();
                                }
                                KeyCode::Left => {
                                    app.handle_left();
                                }
                                KeyCode::Right => {
                                    app.handle_right();
                                }
                                KeyCode::Enter => {
                                    if app.selected_button == 0 && !app.input.is_empty() {
                                        app.state = AppState::Connecting;
                                        app.progress = 0.0;
                                        
                                        for i in 0..=10 {
                                            app.progress = i as f64 / 10.0;
                                            terminal.draw(|f| ui::render(f, app))?;
                                            std::thread::sleep(Duration::from_millis(50));
                                        }
                                        
                                        let address = app.input.clone();
                                        let result = tokio::runtime::Runtime::new()
                                            .unwrap()
                                            .block_on(async {
                                                crate::handlers::claim::claim_with_address(address).await
                                            });

                                        match result {
                                            Responder::Success(_) => {
                                                app.state = AppState::Connected;
                                                terminal.draw(|f| ui::render(f, app))?;
                                                std::thread::sleep(Duration::from_millis(500));
                                                
                                                app.state = AppState::Claiming;
                                                app.progress = 0.0;
                                                
                                                for i in 0..=20 {
                                                    app.progress = i as f64 / 20.0;
                                                    terminal.draw(|f| ui::render(f, app))?;
                                                    std::thread::sleep(Duration::from_millis(100));
                                                }
                                                
                                                if let Responder::Success(msg) = result {
                                                    app.state = AppState::Success(msg);
                                                }
                                            }
                                            Responder::Error(msg) => {
                                                app.state = AppState::Error(msg);
                                            }
                                        }
                                    } else if app.selected_button == 1 {
                                        return Ok(());
                                    }
                                }
                                _ => {}
                            }
                        }
                        AppState::Success(_) | AppState::Error(_) => {
                            match key.code {
                                KeyCode::Tab => {
                                    app.select_next_button();
                                }
                                KeyCode::BackTab => {
                                    app.select_prev_button();
                                }
                                KeyCode::Enter => {
                                    if app.selected_button == 0 {
                                        app.reset();
                                    } else {
                                        return Ok(());
                                    }
                                }
                                KeyCode::Char('r') => {
                                    app.reset();
                                }
                                KeyCode::Esc | KeyCode::Char('q') => {
                                    return Ok(());
                                }
                                _ => {}
                            }
                        }
                        AppState::Connecting | AppState::Connected | AppState::Claiming => {}
                    }
                }
                _ => {}
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.toggle_cursor();
            last_tick = std::time::Instant::now();
        }
    }
}
