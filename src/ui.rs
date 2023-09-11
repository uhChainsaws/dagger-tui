use tui::{
    backend::Backend,
    layout::Alignment,
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Sparkline},
    Frame,
};

use crate::app::App;

/// Renders the user interface widgets.
pub fn render<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui-org/ratatui/tree/master/examples
    let maxxx = match app.tab {
        crate::app::DaggerTab::Processes => {*app.trail.iter().max().unwrap_or(&500)},
        crate::app::DaggerTab::RAM => {app.get_all_memory()},
    };
    frame.render_widget(
        Sparkline::default()
        .data(&app.trail)
        .max(maxxx)
        .block(
            Block::default()
                .title(format!("[dagger: |{}|]", app.tab))
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Double),
        )
        .style(Style::default().fg(Color::Red).bg(Color::Black)),
        frame.size(),
    )
}
