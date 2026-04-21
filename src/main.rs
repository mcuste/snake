//! Snake game binary entry point.

use bevy::prelude::*;
use bevy_ratatui::RatatuiPlugins;

use snake::plugin::GamePlugin;

fn main() {
    install_panic_hook();

    App::new()
        .add_plugins(MinimalPlugins.set(bevy::app::ScheduleRunnerPlugin::run_loop(
            std::time::Duration::from_secs_f64(1.0 / 60.0),
        )))
        .add_plugins(bevy::state::app::StatesPlugin)
        .add_plugins(RatatuiPlugins::default())
        .add_plugins(GamePlugin)
        .run();
}

/// Restores the terminal to a usable state before printing any panic message.
///
/// Without this hook a panic in raw mode leaves the terminal broken (no echo,
/// no cursor, garbled input).
fn install_panic_hook() {
    std::panic::set_hook(Box::new(|info| {
        // Best-effort cleanup — ignore errors, we are already panicking.
        let _ = bevy_ratatui::crossterm::terminal::disable_raw_mode();
        let _ = bevy_ratatui::crossterm::execute!(
            std::io::stderr(),
            bevy_ratatui::crossterm::terminal::LeaveAlternateScreen,
            bevy_ratatui::crossterm::cursor::Show,
        );
        eprintln!("{info}");
    }));
}
