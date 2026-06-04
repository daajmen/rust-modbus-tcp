use crate::modbus::modbus_client::ModbusMaster;
use crate::tui::app::ConnectionStatus;
use crate::tui::handler::handle_event;
use crate::tui::ui::render;
use crate::{AppState, modbus::types::RegisterData};
use color_eyre::Result;
use ratatui::{DefaultTerminal, widgets::ListState};
use std::time::{Duration, Instant};

pub fn run(mut terminal: DefaultTerminal, app: &mut AppState) -> Result<()> {
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
            None => 1500_u64,
        };

        match app.connection_status {
            ConnectionStatus::InitilizeConnection => {
                app.connection_error = false;
                if master.is_none() {
                    // Create instance
                    let mut client = ModbusMaster::new(
                        &app.connection_settings.ip_adress,
                        &app.connection_settings.port,
                    );

                    match client.connect() {
                        Ok(_) => {
                            master = Some(client);
                            app.connection_settings.init = true;
                        }
                        Err(_e) => {
                            app.connect_requested = false;
                            app.connection_settings.init = false;
                            app.connection_error = true;
                        }
                    }
                }
            }
            ConnectionStatus::Connected => {
                if last_poll.elapsed() >= Duration::from_millis(loop_time) {
                    app.connection_error = false;
                    last_poll = Instant::now();
                    app.counter += 1;
                    // Fetch data
                    if let Some(master) = master.as_mut() {
                        app.modbus_data.clear();

                        for r in app.modbus_requests.iter() {
                            match master.read_modbus_register(r.clone()) {
                                Ok(response) => {
                                    for (register, value) in response {
                                        app.modbus_data.push(RegisterData {
                                            register,
                                            data: value,
                                        })
                                    }
                                }
                                Err(_) => {
                                    app.connection_error = true;
                                }
                            }
                        }
                    }
                }
            }
            ConnectionStatus::Disconnected => {
                app.counter = 0;
                app.connection_settings.init = false;
            }
            ConnectionStatus::ConnectionErrorTimeOut => {
                app.connection_error = true;
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
