use color_eyre::{Result}; 
use crossterm::event::{self, Event, KeyEventKind};
use ratatui::widgets::ListState; 
use crate::ui::app::{AppState, ModbusRequestPopupField, PopupField, UiStates, handle_modbus_data};
use crate::{modbus::modbus_client::{ModbusFunction, ModbusMaster}};
use std::time::{Duration, Instant};

pub fn handle_event(app: &mut AppState, list_state: &mut ListState) -> Result<bool>{
    /// function to help with to remove the single value u8
    fn backspace_rm_u8(input_value: Option<u8> ) -> Option<u8> {
        let mut temp = String::new(); 
        match input_value {
            Some(value) => {
                if value < 10 {
                    return None; 
                }
                temp = format!("{}", value);
                temp.pop(); 
                if let Ok(value) = temp.parse::<u8>() {
                    return Some(value); 
                } else {
                    return None;
                }                   
            }
        _ => {None}
        }
    }

    /// function to help with to remove the single value u16
    fn backspace_rm_u16(input_value: Option<u16> ) -> Option<u16> {
        let mut temp = String::new(); 
        match input_value {
            Some(value) => {
                if value < 10 {
                    return None; 
                }
                temp = format!("{}", value);
                temp.pop(); 
                if let Ok(value) = temp.parse::<u16>() {
                    return Some(value); 
                } else {
                    return None;
                }                   
            }
        _ => {None}
        }
    }

    /// Write to values in item list u8
    fn write_to_u8(input_value: Option<u8>, c: char ) -> Option<u8> {
        let mut temp = String::new(); 
        match input_value {
            Some(value) => temp = format!("{}", value),
            None => () 
        }
        temp.push(c);
        if let Ok(value) = temp.parse::<u8>() {
            return Some(value);
        } else {
            return None;
        }
    }

    /// Write to values in item list u16
    fn write_to_u16(input_value: Option<u16>, c: char ) -> Option<u16> {
        let mut temp = String::new(); 
        match input_value {
            Some(value) => temp = format!("{}", value),
            None => () 
        }
        temp.push(c);
        if let Ok(value) = temp.parse::<u16>() {
            return Some(value);
        } else {
            return None;
        }
    }

    if event::poll(Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press{
                    match key.code {
                        event::KeyCode::Char('q') => return Ok(true),
                        event::KeyCode::Char('c') => app.connect_requested = !app.connect_requested,
                        event::KeyCode::Char('C') => app.ui_state = UiStates::ConfGateway,
                        event::KeyCode::Char('A') => app.ui_state = UiStates::AddRegisters,
                        event::KeyCode::Char(c) => {
                            match app.ui_state {
                                UiStates::ConfGateway => {
                                    match app.active_popup_field {
                                        PopupField::Ip => app.ip_adress.push(c),
                                        PopupField::Port => app.port.push(c),
                                        PopupField::Poll => app.poll_time_input.push(c),
                                    }
                                }
                                UiStates::AddRegistersInput => {
                                    let index_state = list_state.selected();
                                        match index_state {
                                            Some(0) => {
                                                app.modbus_request_data.slave_id = write_to_u8(app.modbus_request_data.slave_id, c);
                                            },
                                            Some(1) => {
                                                app.modbus_request_data.start_addr = write_to_u16(app.modbus_request_data.start_addr, c);
                                            },
                                            Some(2) => {
                                                app.modbus_request_data.quantity = write_to_u8(app.modbus_request_data.quantity, c);
                                            }
                                        _ => {}
                                    }
                                    
                                }
                            _ => {}
                            }
                        }
                        event::KeyCode::Esc => {
                            match app.ui_state {
                                UiStates::ConfGateway => {
                                    app.ui_state = UiStates::Home;
                                    if let Ok(value) = app.poll_time_input.parse::<u16>() {
                                        app.poll_time = value;
                                    }
                                },
                                UiStates::AddRegisters => {
                                    app.ui_state = UiStates::Home; 
                                },
                                UiStates::AddRegistersInput => {
                                    app.ui_state = UiStates::AddRegisters; 
                                }                                    
                            _ => {}
                            }
                        }
                        event::KeyCode::Tab => {
                            match app.ui_state {
                                UiStates::ConfGateway => {
                                    app.active_popup_field = match app.active_popup_field {
                                        PopupField::Ip => PopupField::Port,
                                        PopupField::Port => PopupField::Poll,
                                        PopupField::Poll => PopupField::Ip,
                                        }
                                }
                            _ => {}
                            }
                        }
                        event::KeyCode::Backspace => {
                            match app.ui_state {
                                UiStates::ConfGateway => {
                                    match app.active_popup_field {
                                        PopupField::Ip => { app.ip_adress.pop(); }
                                        PopupField::Port => { app.port.pop(); }
                                        PopupField::Poll => { 
                                            app.poll_time_input.pop();
                                            
                                            if let Ok(value) = app.poll_time_input.parse::<u16>() {
                                                app.poll_time = value;
                                            }
                                        
                                        }
                                    }
                                }
                                UiStates::AddRegistersInput => {
                                    let index_state = list_state.selected();
                                        match index_state {
                                            Some(0) => {
                                                app.modbus_request_data.slave_id = backspace_rm_u8(app.modbus_request_data.slave_id);
                                            } 
                                            Some(1) => {
                                                app.modbus_request_data.start_addr = backspace_rm_u16(app.modbus_request_data.start_addr);
                                            }
                                            Some(2) => {
                                                app.modbus_request_data.quantity = backspace_rm_u8(app.modbus_request_data.quantity);
                                            }
                                        _ => {}
                                    }
                                    
                                }                                    
                            _ => {}
                            }
                        }
                        event::KeyCode::Down => {
                            match app.ui_state {
                                UiStates::AddRegisters | UiStates::AddRegistersInput => {
                                    list_state.select_next();
                                },
                            _ => {}
                            }
                            
                        }
                        event::KeyCode::Up => {
                            match app.ui_state {
                                UiStates::AddRegisters | UiStates::AddRegistersInput => {
                                    list_state.select_previous();
                                },
                            _ => {}
                            }
                            
                        }
                        event::KeyCode::Enter => {
                            match app.ui_state {
                                UiStates::AddRegisters => {
                                    let index_state = list_state.selected();
                                    match index_state {
                                    Some(0) => {
                                        app.modbus_request_data.function = ModbusFunction::ReadCoilRegister;
                                        app.ui_state = UiStates::AddRegistersInput;
                                    },
                                    Some(1) => {
                                        app.modbus_request_data.function = ModbusFunction::ReadInputStatusRegister;
                                        app.ui_state = UiStates::AddRegistersInput;
                                    },
                                    Some(2) => {
                                        app.modbus_request_data.function = ModbusFunction::ReadInputRegister;
                                        app.ui_state = UiStates::AddRegistersInput;
                                    },
                                    Some(3) => {
                                        app.modbus_request_data.function = ModbusFunction::ReadHoldingRegister;
                                        app.ui_state = UiStates::AddRegistersInput;
                                    }
                                    _ => {}
                                    } 
                                },
                            _ => {}
                            }
                            
                        }                            
                        _ => {},
                    }
            }
        }
    }
Ok(false)    
}