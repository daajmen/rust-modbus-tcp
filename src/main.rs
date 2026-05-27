use crate::runtime::run::run;
use crate::ui::app::AppState;
use color_eyre::Result;

mod modbus;
mod runtime;
mod ui;

fn main() -> Result<()> {
    color_eyre::install()?;
    let mut app_state = AppState::new();

    let terminal = ratatui::init();
    let result = run(terminal, &mut app_state);
    ratatui::restore();

    result
}
