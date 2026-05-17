use color_eyre::{Result}; 
use crossterm::event::{self, Event, KeyEventKind}; 
use ratatui::{DefaultTerminal, Frame, style::{Color, Stylize}, text::{Line}, widgets::{Block, Borders, Paragraph, Clear}}; 
use ratatui::layout::{Layout, Direction, Constraint, Rect};
use ratatui::prelude::*; 
use std::time::{Duration, Instant};

use crate::ui::app::{AppState, PopupField, handle_modbus_data};
use crate::{modbus::modbus_client::{ModbusFunction, ModbusMaster}};


pub fn render(frame: &mut Frame, app: &mut AppState) {
 
    let instructions = Line::from(vec![
        " Quit ".into(),
        "<q> ".blue().bold(),
        " Connect ".into(), 
        "<c> ".blue().bold(),
        " Gateway Configuration ".into(),
        "<C> ".blue().bold(), 
        " Add modbus register ".into(),
        "<a> ".blue().bold(), 
        

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


    frame.render_widget(
        Paragraph::new(" Modbus registers ")
        .block(Block::new().bold().fg(Color::Blue).borders(Borders::ALL)),
        outer_layout[0],
    );
 
    let connection_status: &str;
    let connection_color: Color;

    if app.connect_requested {
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


    // Gateway configuration popup 
    if app.show_config_popup {
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
                        &app.poll_time.to_string(),
                        Style::default().fg(highlight_poll))]),                        
                
                ])
                .block(Block::new().title("Connection settings ").borders(Borders::ALL)),
        popup,
        );
    }

}

pub fn run(mut terminal: DefaultTerminal, app: &mut AppState) -> Result<()> {
    // TODO CLEAN 
    let start_adress = 0;
    let quantity = 6;     
    let mut last_poll = Instant::now(); 
    

    // Prepare modbus master 
    let mut master: Option<ModbusMaster> = None;

    loop {

        // Check request
        if app.connect_requested && last_poll.elapsed() >= Duration::from_millis(app.poll_time as u64) {
            last_poll = Instant::now(); 
            // reset connection flag 
            app.connection_error = false; 
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

        terminal.draw(|frame| render(frame, app))?; 


        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press{

                    if app.show_config_popup {
                        match key.code {

                            event::KeyCode::Tab => {
                                app.active_popup_field = match app.active_popup_field {
                                    PopupField::Ip => PopupField::Port,
                                    PopupField::Port => PopupField::Poll,
                                    PopupField::Poll => PopupField::Ip,
                                }
                            }

                            event::KeyCode::Backspace => {
                                match app.active_popup_field {
                                    PopupField::Ip => { app.ip_adress.pop(); }
                                    PopupField::Port => { app.port.pop(); }
                                    PopupField::Poll => { app.poll_time; }
                                }
                            }

                            event::KeyCode::Char(c) => {
                                match app.active_popup_field {
                                    PopupField::Ip => app.ip_adress.push(c),
                                    PopupField::Port => app.port.push(c),
                                    PopupField::Poll => todo!(),
                                }
                            }

                            event::KeyCode::Esc => app.show_config_popup = false,
                            event::KeyCode::Enter => app.show_config_popup = false,
                            _ => {}
                        }                        
                    } else {
                        match key.code {
                            event::KeyCode::Char('q') => break Ok(()),
                            event::KeyCode::Char('c') => app.connect_requested = !app.connect_requested,
                            event::KeyCode::Char('C') => app.show_config_popup = true,
                            event::KeyCode::Char('a') => todo!(),
                            event::KeyCode::Esc => app.show_config_popup = false,
                            _ => {},
                        }

                    }

                }
            }
        }
    }
}


