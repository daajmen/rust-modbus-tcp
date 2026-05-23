use color_eyre::{Result}; 
use ratatui::{DefaultTerminal, Frame, style::{Color, Stylize}, text::Line, widgets::{Block, Borders, Clear, List, ListState, Paragraph}}; 
use ratatui::layout::{Layout, Direction, Constraint, Rect};
use ratatui::prelude::*; 
use std::{ time::{Duration, Instant}, vec};

use crate::ui::app::{AppState, ModbusRequestData, ModbusRequestPopupField, PopupField, UiStates, handle_modbus_data};
use crate::{modbus::modbus_client::{ModbusFunction, ModbusMaster}};
use crate::ui::handler::handle_event;

pub fn render(frame: &mut Frame, app: &mut AppState, list_state: &mut ListState) {

    fn render_register_popup(frame: &mut Frame, app: &mut AppState, list_state: &mut ListState) {

            let popup = Rect {
                x: frame.area().width / 4, 
                y: frame.area().height / 4,
                width: frame.area().width / 2, 
                height: frame.area().height / 3,  
            }; 

            frame.render_widget(
                Clear,
                popup,
            );

            let items = [
                ModbusFunction::ReadCoilRegister.as_str(),
                ModbusFunction::ReadInputStatusRegister.as_str(),
                ModbusFunction::ReadInputRegister.as_str(),
                ModbusFunction::ReadHoldingRegister.as_str(),
            ];

            let list = List::new(items)
                .style(Color::LightMagenta)
                .highlight_style(Modifier::REVERSED)
                .highlight_symbol("> ")
                .block(Block::new().title(" Add modbus register ").borders(Borders::ALL)); 

            frame.render_stateful_widget(list, popup, list_state);
        
    }

    fn render_register_configure_popup(frame: &mut Frame, app: &mut AppState, list_state: &mut ListState) {

            let popup = Rect {
                x: frame.area().width / 4, 
                y: frame.area().height / 4,
                width: frame.area().width / 2, 
                height: frame.area().height / 3,  
            }; 

            frame.render_widget(
                Clear,
                popup,
            );

            let items = [
                match app.modbus_request_data.slave_id {
                    Some(value) => format!("Slave id: {}", value),
                    None =>  format!("Slave id: None", ),
                },
                match app.modbus_request_data.start_addr {
                    Some(value) => format!("Start register: {}", value),
                    None =>  format!("Start register: None", ),
                },
                match app.modbus_request_data.quantity {
                    Some(value) => format!("Quantity: {}", value),
                    None =>  format!("Quantity: None", ),
                }                                  
            ];

            let list = List::new(items)
                .style(Color::LightMagenta)
                .highlight_style(Modifier::REVERSED)
                .highlight_symbol("> ")
                .block(Block::new().title(" Add modbus register ").borders(Borders::ALL)); 

            frame.render_stateful_widget(list, popup, list_state);
        
        
    }    

    fn render_config_popup(frame: &mut Frame, app: &mut AppState) {
        let popup = Rect {
            x: frame.area().width / 4, 
            y: frame.area().height / 4,
            width: frame.area().width / 2, 
            height: frame.area().height / 3,  
        }; 

        frame.render_widget(
            Clear,
            popup,
        );

        let highlight_ip = match app.active_popup_field {
            PopupField::Ip => Color::Green,
            _ => Color::Yellow,            
        };

        let highlight_port = match app.active_popup_field {
            PopupField::Port => Color::Green,
            _ => Color::Yellow,
        };

        let highlight_poll = match app.active_popup_field {
            PopupField::Poll => Color::Green,
            _ => Color::Yellow,            
        };

        frame.render_widget(
            Paragraph::new(vec![
                Line::from(vec![
                    "IP-Adress: ".into(),
                    Span::styled(
                        &app.ip_adress,
                        Style::default().fg(highlight_ip))]),

                Line::from(vec![    
                    "Gateway port: ".into(),
                    Span::styled(
                        &app.port,
                        Style::default().fg(highlight_port))]),

                Line::from(vec![    
                    "Polling time: ".into(),
                    Span::styled(
                        &app.poll_time_input.to_string(),
                        Style::default().fg(highlight_poll))]),                        
                    
                ])
                .block(Block::new().title("Connection settings ").borders(Borders::ALL ))
                .style(Color::LightMagenta),
            popup,
            );

        
    }        


    let instructions = Line::from(vec![
        " Quit ".into(),
        "<q> ".blue().bold(),
        " Connect ".into(), 
        "<c> ".blue().bold(),
        " Gateway Configuration ".into(),
        "<C> ".blue().bold(), 
        " Add modbus register ".into(),
        "<A> ".blue().bold(), 
    ]); 

    let main_block = Block::new()
        .title( " Rust Modbus TCP ")
        .title_alignment(Alignment::Center)
        .title_bottom(instructions.centered())
        .borders(Borders::ALL); 

    let area = main_block.inner(frame.area());     
    
    frame.render_widget(main_block, frame.area());
        
    let outer_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Percentage(25),
            Constraint::Percentage(75)])
        .split(area); 

    let inner_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(25),
            Constraint::Percentage(75)])
        .split(outer_layout[1]);     
    
    //let mut items: Vec<ModbusRequestData> = Vec::new();
    let mut items = [app.modbus_request_data.as_string()];

    // Temporary 

    let list = List::new(items)
            .block(
                Block::new()
                .title(" Modbus register ")
                .bold()
                .fg(Color::Blue)
                .borders(Borders::ALL)
            );
    frame.render_widget(list, outer_layout[0]);
 
    let connection_status: &str;
    let connection_color: Color;

    if app.connect_requested && !app.connection_error {
        connection_status = "CONNECTED";
        connection_color = Color::Green;

    } else if app.connection_error {
        connection_status = "CONNECTION FAILED!!";
        connection_color = Color::Red;        
    } else {
        connection_status = "DISCONNECTED";
        connection_color = Color::Red;        
    }

    let data_box = Text::from(vec![
        Line::from(vec![
            "IP-adress: ".into(),
             Span::styled(
                app.ip_adress.clone(), 
                Style::default().fg(Color::Yellow)),
        ]),
        Line::from(vec![
            "Port: ".into(),
             Span::styled(
                app.port.clone(), 
                Style::default().fg(Color::Yellow)),             
        ]),
        Line::from(vec![
            "Polling time: ".into(),
             Span::styled(
                app.poll_time.clone().to_string(), 
                Style::default().fg(Color::Yellow)),
        ]),
        Line::from(vec![
            "Poll counter: ".into(),
            Span::styled(
                app.counter.to_string(),
                Style::default().fg(Color::Yellow)),
        ]),         
        Line::from(vec![
            "Connection stats: ".into(),
            Span::styled(
                connection_status,
                Style::default().fg(connection_color)),
        ]),   
    ]);

    frame.render_widget(
        Paragraph::new(data_box)
        .block(Block::new().bold().fg(Color::Blue).borders(Borders::ALL)),
        inner_layout[0],
    );    
    frame.render_widget(
        Paragraph::new(app.modbus_data.clone())
        .block(Block::new().bold().fg(Color::Green).borders(Borders::ALL)),
        inner_layout[1],
    );


    match app.ui_state {
        UiStates::ConfGateway => {
            render_config_popup(frame, app);
        }
        UiStates::AddRegisters => {
            render_register_popup(frame, app, list_state);
        }
        UiStates::AddRegistersInput => {
            render_register_configure_popup(frame, app, list_state);
        }
        _ => {}
    }
    


}

pub fn run(mut terminal: DefaultTerminal, app: &mut AppState) -> Result<()> {
    

    // TODO CLEAN 
    let start_adress = 0;
    let quantity = 6;     
    let mut last_poll = Instant::now(); 
    let mut list_state = ListState::default().with_selected(Some(0));

    

    // Prepare modbus master 
    let mut master: Option<ModbusMaster> = None;

    loop {

        // Check request
        if app.connect_requested && last_poll.elapsed() >= Duration::from_millis(app.poll_time as u64) {
            last_poll = Instant::now(); 
            // if connection has not been made
            if master.is_none() {
                // Create instance
                let mut client = ModbusMaster::new(
                    &app.ip_adress,
                    &app.port
                );
                
            let connection_ok = client.connect(); 

            master = Some(client);

            // Connection failed
            app.connection_error = connection_ok.is_err();
            
            }
            // Fetch data 
            if let Some(master) = master.as_mut() {
                let data = [
                    master.read_modbus_register(ModbusFunction::ReadCoilRegister, app.slave_id, start_adress, quantity),
                    master.read_modbus_register(ModbusFunction::ReadInputStatusRegister,app.slave_id, start_adress,quantity),
                    master.read_modbus_register(ModbusFunction::ReadInputRegister, app.slave_id, start_adress, quantity),
                    master.read_modbus_register(ModbusFunction::ReadHoldingRegister,app.slave_id,start_adress,quantity),        
                ];

            // Not my solution -.-     
            if let [Ok(a), Ok(b), Ok(c), Ok(d)] = data {
                handle_modbus_data(app, vec![a, b, c, d]);
            } else {
                handle_modbus_data(app, vec![]);
            }
            }
    

        } if !app.connect_requested{
            master = None;
        }

        terminal.draw(|frame| render(frame, app, &mut list_state))?; 

        // Key event handle
        if handle_event(app, &mut list_state)? {
            break Ok(())
        }

    }
}

