use color_eyre::{Result}; 
use crossterm::{event::{self, Event}, terminal}; 
use ratatui::{DefaultTerminal, Frame, style::{Color, Stylize}, text::{Line, ToSpan}, widgets::{Block, Borders, Paragraph, Widget}}; 
use ratatui::layout::{Layout, Direction, Constraint};
use ratatui::prelude::*; 
use std::collections::BTreeMap;

use crate::ui::app::App;


pub fn render(frame: &mut Frame, app: &App, data: String) {
 
    let outer_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Percentage(25),
            Constraint::Percentage(75)])
        .split(frame.area()); 

    let inner_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(25),
            Constraint::Percentage(75)])
        .split(outer_layout[1]);     

    frame.render_widget(
        Paragraph::new(" Configuration")
        .block(Block::new().bold().fg(Color::Blue).borders(Borders::ALL)),
        outer_layout[0],
    );

    // TODO 
    let ip = "192.168.50.21"; 

    let data_box = Text::from(vec![
        Line::from(vec![
            "IP-adress: ".into(),
             ip.into()
        ]),
        Line::from("Port: "), 
        Line::from("Poll time: "), 
        Line::from("Connection status: ")        
    ]);

    frame.render_widget(
        Paragraph::new(data_box)
        .block(Block::new().bold().fg(Color::Blue).borders(Borders::ALL)),
        inner_layout[0],
    );    
    frame.render_widget(
        Paragraph::new(data)
        .block(Block::new().bold().fg(Color::Green).borders(Borders::ALL)),
        inner_layout[1],
    );
}

pub fn run(mut terminal: DefaultTerminal, data: Vec<BTreeMap<u16, u16>>) -> Result<()> {

    let mut app = App {
        ip_adress: "127.0.0.1".to_string(), 
        port: "502".to_string(), 
        slave_id: 1, 
    }; 

    //let modbus_data = format!("{:?}", data);
    let mut modbus_data = String::new();

    for x in data {
        modbus_data.push_str(&format!("{:?}", x));
        modbus_data.push_str("\n");
    } 



    loop {
        terminal.draw(|frame| render(frame, &app, modbus_data.clone()))?; 


        if let Event::Key(key) = event::read()? {
            match key.code {
                event::KeyCode::Char(c) => {
                    if c == 'q' {
                        break Ok(()); 
                    }
                }
                _ => {}
            }
        }
    }
}

