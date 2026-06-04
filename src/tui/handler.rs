use crate::modbus::types::ModbusFunction;
use crate::tui::app::{AppState, UiStates};
use color_eyre::Result;
use crossterm::event::{self, Event, KeyEventKind};
use ratatui::widgets::ListState;
use std::time::Duration;

pub fn handle_event(app: &mut AppState, list_state: &mut ListState) -> Result<bool> {
    /// function to help with to remove the single value u8
    fn backspace_rm_u8(input_value: Option<u8>) -> Option<u8> {
        input_value.and_then(|value| {
            let new_value = value / 10;

            if new_value == 0 {
                None
            } else {
                Some(new_value)
            }
        })
    }

    /// function to help with to remove the single value u16
    fn backspace_rm_u16(input_value: Option<u16>) -> Option<u16> {
        input_value.and_then(|value| {
            let new_value = value / 10;

            if new_value == 0 {
                None
            } else {
                Some(new_value)
            }
        })
    }

    /// Write to values in item list u8
    fn write_to_u8(input_value: Option<u8>, c: char) -> Option<u8> {
        let mut temp = match input_value {
            Some(value) => value.to_string(),
            None => String::new(),
        };
        temp.push(c);
        temp.parse::<u8>().ok()
    }

    /// Write to values in item list u16
    fn write_to_u16(input_value: Option<u16>, c: char) -> Option<u16> {
        let mut temp = match input_value {
            Some(value) => value.to_string(),
            None => String::new(),
        };
        temp.push(c);
        temp.parse::<u16>().ok()
    }

    if event::poll(Duration::from_millis(50))?
        && let Event::Key(key) = event::read()?
        && key.kind == KeyEventKind::Press
    {
        match key.code {
            event::KeyCode::Char('q') => return Ok(true),
            event::KeyCode::Char('c') => app.connect_requested = !app.connect_requested,
            event::KeyCode::Char('C') => app.ui_state = UiStates::ConfGateway,
            event::KeyCode::Char('A') => app.ui_state = UiStates::AddRegisters,
            event::KeyCode::Char(c) => match app.ui_state {
                UiStates::ConfGateway => {
                    let index_state = list_state.selected();
                    match index_state {
                        Some(0) => {
                            app.connection_settings.ip_adress.push(c);
                        }
                        Some(1) => {
                            app.connection_settings.port.push(c);
                        }
                        Some(2) => {
                            app.connection_settings.poll_time =
                                write_to_u16(app.connection_settings.poll_time, c)
                        }
                        _ => {}
                    }
                }
                UiStates::AddRegistersInput => {
                    let index_state = list_state.selected();
                    match index_state {
                        Some(0) => {
                            app.modbus_request_data.slave_id =
                                write_to_u8(app.modbus_request_data.slave_id, c);
                        }
                        Some(1) => {
                            app.modbus_request_data.start_addr =
                                write_to_u16(app.modbus_request_data.start_addr, c);
                        }
                        Some(2) => {
                            app.modbus_request_data.quantity =
                                write_to_u8(app.modbus_request_data.quantity, c);
                        }
                        _ => {}
                    }
                }
                _ => {}
            },
            // Escape
            event::KeyCode::Esc => match app.ui_state {
                UiStates::ConfGateway => {
                    app.ui_state = UiStates::Home;
                }
                UiStates::AddRegisters => {
                    app.ui_state = UiStates::Home;
                }
                UiStates::AddRegistersInput => {
                    app.ui_state = UiStates::AddRegisters;
                }
                _ => {}
            },
            // Backspace
            event::KeyCode::Backspace => match app.ui_state {
                UiStates::ConfGateway => {
                    let index_state = list_state.selected();
                    match index_state {
                        Some(0) => {
                            app.connection_settings.ip_adress.pop();
                        }
                        Some(1) => {
                            app.connection_settings.port.pop();
                        }
                        Some(2) => {
                            app.connection_settings.poll_time =
                                backspace_rm_u16(app.connection_settings.poll_time);
                        }
                        _ => {}
                    }
                }

                UiStates::AddRegistersInput => {
                    let index_state = list_state.selected();
                    match index_state {
                        Some(0) => {
                            app.modbus_request_data.slave_id =
                                backspace_rm_u8(app.modbus_request_data.slave_id);
                        }
                        Some(1) => {
                            app.modbus_request_data.start_addr =
                                backspace_rm_u16(app.modbus_request_data.start_addr);
                        }
                        Some(2) => {
                            app.modbus_request_data.quantity =
                                backspace_rm_u8(app.modbus_request_data.quantity);
                        }
                        _ => {}
                    }
                }
                _ => {}
            },
            // Down
            event::KeyCode::Down => match app.ui_state {
                UiStates::AddRegisters | UiStates::AddRegistersInput | UiStates::ConfGateway => {
                    list_state.select_next();
                }
                _ => {}
            },
            // Up
            event::KeyCode::Up => match app.ui_state {
                UiStates::AddRegisters | UiStates::AddRegistersInput | UiStates::ConfGateway => {
                    list_state.select_previous();
                }
                _ => {}
            },
            // Enter
            event::KeyCode::Enter => match app.ui_state {
                UiStates::AddRegisters => {
                    let index_state = list_state.selected();
                    match index_state {
                        Some(0) => {
                            app.modbus_request_data.function = ModbusFunction::Coil;
                            app.ui_state = UiStates::AddRegistersInput;
                        }
                        Some(1) => {
                            app.modbus_request_data.function = ModbusFunction::DiscreteInput;
                            app.ui_state = UiStates::AddRegistersInput;
                        }
                        Some(2) => {
                            app.modbus_request_data.function = ModbusFunction::InputRegister;
                            app.ui_state = UiStates::AddRegistersInput;
                        }
                        Some(3) => {
                            app.modbus_request_data.function = ModbusFunction::HoldingRegister;
                            app.ui_state = UiStates::AddRegistersInput;
                        }
                        _ => {}
                    }
                }
                UiStates::AddRegistersInput
                    if app.modbus_request_data.slave_id.is_some()
                        && app.modbus_request_data.start_addr.is_some()
                        && app.modbus_request_data.quantity.is_some() =>
                {
                    app.modbus_write_request = true;
                    app.ui_state = UiStates::AddRegisters;
                }
                UiStates::ConfGateway => app.ui_state = UiStates::Home,
                _ => {}
            },
            _ => {}
        }
    }
    Ok(false)
}
