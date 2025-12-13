use std::{io, thread, time::Duration};
use tui::{
    backend::CrosstermBackend,    
    Terminal
};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};

mod ui;
mod log_manager;

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    // setup terminal

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut logger = log_manager::new("itzg/minecraft-server", "minecraft-mc-srv").unwrap();
    let _ = logger.process().await;
    
    let mut uistate = ui::UIState {
        players: vec![],
        logs: vec![],
        chat: vec![],
        horizontal_scroll: 0,
        vertical_scroll: 0,
        stdout: "".to_owned(),
        message_box: "".to_owned(),
    };
    
    let mut iteration = 0;
    let mut i_no = 0;
    loop {
        if iteration % 100 == 0 {
            iteration = 0;
            i_no += 1;
            let _ = logger.process().await;

            uistate.logs = logger.get_logs();
            uistate.chat = logger.get_chat();
            // uistate.chat.push(format!("Iteration {}", i_no));
            uistate.players = logger.get_players();
            uistate.stdout = logger.get_command_output();
        }
        terminal.draw(|f| ui::ui(f, &uistate))?;  // Draw UI
        if event::poll(Duration::from_millis(1))? {
            if let Event::Key(KeyEvent {code, modifiers, ..}) = event::read()? {
                match (code, modifiers) {
                    (KeyCode::Esc, KeyModifiers::NONE) => {
                        break
                    }

                    (KeyCode::Backspace, KeyModifiers::CONTROL) => {
                        uistate.message_box.pop();
                        while uistate.message_box.chars().last().is_some() && uistate.message_box.chars().last().unwrap() != ' ' {
                            uistate.message_box.pop();
                        }
                    }
                    (KeyCode::Char('h'), KeyModifiers::CONTROL) => { // also ctrl backspace
                        uistate.message_box.pop();
                        while uistate.message_box.chars().last().is_some() && uistate.message_box.chars().last().unwrap() != ' ' {
                            uistate.message_box.pop();
                        }
                    }
                    (KeyCode::Backspace, KeyModifiers::NONE) => {
                        uistate.message_box.pop();
                    }


                    (KeyCode::Char(c), KeyModifiers::NONE) => {
                        uistate.message_box.push(c);
                    }
                    (KeyCode::Char(c), KeyModifiers::SHIFT) => {
                        uistate.message_box.push(c.to_ascii_uppercase())
                    }

                    (KeyCode::Left, KeyModifiers::SHIFT) => {
                        if uistate.horizontal_scroll < 10 {
                            uistate.horizontal_scroll = 0;
                        } else {
                            uistate.horizontal_scroll -= 10;
                        }
                    }
                    (KeyCode::Right, KeyModifiers::SHIFT) => {
                        if uistate.horizontal_scroll > u16::MAX - 10 {
                            uistate.horizontal_scroll = u16::MAX;
                        } else {
                            uistate.horizontal_scroll += 10;
                        }
                    }

                    (KeyCode::Down, KeyModifiers::SHIFT) => {
                        if uistate.vertical_scroll < 10 {
                            uistate.vertical_scroll = 0;
                        } else {
                            uistate.vertical_scroll -= 10;
                        }
                    }
                    (KeyCode::Up, KeyModifiers::SHIFT) => {
                        if uistate.vertical_scroll > u16::MAX - 10 {
                            uistate.vertical_scroll = u16::MAX;
                        } else {
                            uistate.vertical_scroll += 10;
                        }
                    }
                    
                    (KeyCode::Enter, _) => {
                        logger.send_message(&uistate.message_box).await;
                        uistate.message_box = String::from("");
                    }
                    _ => {
                        uistate.message_box.push('_');
                    }
                }
            }
        }
        thread::sleep(Duration::from_millis(1));
        iteration += 1;
    }


    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if uistate.stdout != "" {
        println!("Command Output:\n{}", uistate.stdout);
    }

    Ok(())
}