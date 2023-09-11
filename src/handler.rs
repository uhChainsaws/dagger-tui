use crate::app::{App, AppResult};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers, KeyEventKind};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    if key_event.kind == KeyEventKind::Press{
        match key_event.code {
            // Exit application on `ESC` or `q`
            KeyCode::Esc | KeyCode::Char('q') => {
                app.quit();
            }
            // Exit application on `Ctrl-C`
            KeyCode::Char('c') | KeyCode::Char('C') => {
                if key_event.modifiers == KeyModifiers::CONTROL {
                    app.quit();
                }
            }
            
            KeyCode::Left | KeyCode::Char('a') => {
                app.cycle_tab_forward();
            }
            
            KeyCode::Right | KeyCode::Char('d') => {
                app.cycle_tab_backward();
            }
            // Other handlers you could add here.
            _ => {}

        }

    }
    Ok(())
}
