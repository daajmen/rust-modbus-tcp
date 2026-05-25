use color_eyre::{Result}; 
use ratatui::{DefaultTerminal, Frame, style::{Color, Stylize}, text::Line, widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph}}; 
use ratatui::layout::{Layout, Direction, Constraint, Rect};
use ratatui::prelude::*; 
use std::time::{Duration, Instant};

use crate::ui::app::{AppState, UiStates, handle_modbus_data};
use crate::{modbus::modbus_client::{ModbusFunction, ModbusMaster}};
use crate::ui::handler::handle_event;

pub fn render(frame: &mut Frame, app: &mut AppState, list_state: &mut ListState) {

    fn render_register_popup(frame: &mut Frame, list_state: &mut ListState) {

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
                .style(Color::Yellow)
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
                .style(Color::Yellow)
                .highlight_style(Modifier::REVERSED)
                .highlight_symbol("> ")
                .block(Block::new().title(" Add modbus register ").borders(Borders::ALL)); 

            frame.render_stateful_widget(list, popup, list_state);
        
        
    }    

    fn render_connection_settings(frame: &mut Frame, app: &mut AppState, list_state: &mut ListState) {

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
                format!("IP-adress: {}", app.ip_adress),
                format!("Port: {}", app.port),
                match app.poll_time {
                    Some(value) => format!("Modbus requests delay: {}ms", value),
                    None =>  String::from("Modbus requests delay: ---ms")
                }                  
            ];

            let list = List::new(items)
                .style(Color::Yellow)
                .highlight_style(Modifier::REVERSED)
                .highlight_symbol("> ")
                .block(Block::new().title(" Add modbus register ").borders(Borders::ALL)); 

            frame.render_stateful_widget(list, popup, list_state);        
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
    

    if app.modbus_write_request {
        app.modbus_requests.push(app.modbus_request_data.clone()); 
        app.modbus_request_data.clear_data();
        app.modbus_write_request = false; 
    }

    let items: Vec<ListItem> = app
        .modbus_requests
        .iter()
        .map(|request| {
            ListItem::new(request.as_string())
        })
        .collect();

    let list = List::new(items)
            .block(
                Block::new()
                .title(" Modbus register ")
                .bold()
                .fg(Color::Blue)
                .borders(Borders::ALL)
            )
            .style(Color::Yellow);
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
                format!("{}", app.poll_time.unwrap_or(0)), 
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
        .style(Color::Yellow)
        .block(Block::new().bold().fg(Color::Blue).borders(Borders::ALL)),
        inner_layout[1],
    );


    match app.ui_state {
        UiStates::ConfGateway => {
            render_connection_settings(frame, app, list_state);
        }
        UiStates::AddRegisters => {
            render_register_popup(frame, list_state);
        }
        UiStates::AddRegistersInput => {
            render_register_configure_popup(frame, app, list_state);
        }
        _ => {}
    }
    


}

pub fn run(mut terminal: DefaultTerminal, app: &mut AppState) -> Result<()> {
    

    // TODO CLEAN 
    let mut last_poll = Instant::now(); 
    let mut list_state = ListState::default().with_selected(Some(0));

    

    // Prepare modbus master 
    let mut master: Option<ModbusMaster> = None;

    loop {

        let loop_time = match app.poll_time{
            Some(value) => value as u64,
            None => 1500 as u64
        };

        if app.connect_requested && last_poll.elapsed() >= Duration::from_millis(loop_time) {
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

