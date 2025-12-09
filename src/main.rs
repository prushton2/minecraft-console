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

fn main() -> Result<(), io::Error> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut logger = log_manager::new("itzg/minecraft-server", "minecraft-mc-srv").unwrap();

    let mut uistate = ui::UIState {
        players: vec!["Encursed", "AmazingLex52", "Remixalotl"],
        logs: vec!["Log_1", "Log_2"],
        chat: vec!["Hello", "There", "Whats", "Up"],
        message_box: "/execute as AmazingLex52 run say Hello".to_owned(),
    };

    uistate.players = logger.fetch_players();
    // let mut clipboard = Clipboard::new()?;

    loop {
        terminal.draw(|f| ui::ui(f, &uistate))?;  // Draw UI
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(KeyEvent {code, modifiers, ..}) = event::read()? {
                // println!("Full key event:");
                // println!("  Code: {:?}", code);
                // println!("  Modifiers: {:?} (bits: {})", modifiers, modifiers.bits());
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
                    
                    (KeyCode::Enter, _) => {
                        // send message
                        uistate.message_box = String::from("");
                    }

                    // (KeyCode::Char('v'), KeyModifiers::CONTROL) => {
                    //     uistate.message_box.push_str(clipboard.get_text().unwrap().as_str())
                    // }
                    _ => {
                        uistate.message_box.push('_');
                    }
                }
            }
        }
        thread::sleep(Duration::from_millis(10));
    }


    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}