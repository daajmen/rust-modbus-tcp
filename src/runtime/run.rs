use crate::AppState;
use crate::modbus::modbus_client::ModbusMaster;
use crate::ui::app::handle_modbus_data;
use crate::ui::handler::handle_event;
use crate::ui::ui::render;
use color_eyre::Result;
use ratatui::{DefaultTerminal, widgets::ListState};
use std::time::{Duration, Instant};

pub fn run(mut terminal: DefaultTerminal, app: &mut AppState) -> Result<()> {
    // TODO CLEAN
    let mut last_poll = Instant::now();
    let mut list_state = ListState::default().with_selected(Some(0));

    // Prepare modbus master
    let mut master: Option<ModbusMaster> = None;

    loop {
        app.update_state();

        if app.modbus_write_request {
            app.modbus_requests.push(app.modbus_request_data.clone());
            app.modbus_request_data.clear_data();
            app.modbus_write_request = false;
        }

        let loop_time = match app.connection_settings.poll_time {
            Some(value) => value as u64,
            None => 1500 as u64,
        };

        if app.connect_requested && last_poll.elapsed() >= Duration::from_millis(loop_time) {
            last_poll = Instant::now();
            // if connection has not been made
            if master.is_none() {
                // Create instance
                let mut client = ModbusMaster::new(
                    &app.connection_settings.ip_adress,
                    &app.connection_settings.port,
                );

                let connection_ok = client.connect();

                master = Some(client);

                // Connection failed
                app.connection_error = connection_ok.is_err();
            }
            let mut data = vec![];
            // Fetch data
            if let Some(master) = master.as_mut() {
                for r in app.modbus_requests.iter() {
                    if let Ok(response) = master.read_modbus_register(r.clone()) {
                        data.push(response);
                    }
                }

                handle_modbus_data(app, data);
            }
        }
        if !app.connect_requested {
            master = None;
        }

        terminal.draw(|frame| render(frame, app, &mut list_state))?;

        // Key event handle
        if handle_event(app, &mut list_state)? {
            break Ok(());
        }
    }
}
