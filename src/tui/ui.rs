use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::prelude::*;
use ratatui::{
    Frame,
    style::{Color, Stylize},
    text::Line,
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph, Row, Table},
};

use crate::modbus::types::ModbusFunction;
use crate::tui::app::{AppState, ConnectionStatus, UiStates};

/// Creats a centered popup windows Rect
fn centered_rect(frame: &mut Frame) -> Rect {
    Rect {
        x: frame.area().width / 4,
        y: frame.area().height / 4,
        width: frame.area().width / 2,
        height: frame.area().height / 3,
    }
}

/// Builds window that contains the list function
fn list_window<'a>(title: &'a str, items: Vec<String>) -> List<'a> {
    List::new(items)
        .style(Color::Yellow)
        .highlight_style(Modifier::REVERSED)
        .highlight_symbol("> ")
        .block(Block::new().title(title).borders(Borders::ALL))
}

fn line_helper(text: impl Into<String>, value: impl Into<String>, color: Color) -> Line<'static> {
    Line::from(vec![
        text.into().into(),
        Span::styled(value.into(), Style::default().fg(color)),
    ])
}

/// Render add modbus register popup
fn render_register_popup(frame: &mut Frame, list_state: &mut ListState) {
    let popup = centered_rect(frame);

    frame.render_widget(Clear, popup);

    let items = [
        ModbusFunction::Coil.as_str().to_string(),
        ModbusFunction::DiscreteInput.as_str().to_string(),
        ModbusFunction::InputRegister.as_str().to_string(),
        ModbusFunction::HoldingRegister.as_str().to_string(),
    ];

    frame.render_stateful_widget(
        list_window("Add modbus register", items.to_vec()),
        popup,
        list_state,
    );
}

/// Render popup for configuration of the added modbus register
fn render_register_configure_popup(frame: &mut Frame, app: &AppState, list_state: &mut ListState) {
    let popup = centered_rect(frame);

    frame.render_widget(Clear, popup);

    let items = [
        match app.modbus_request_data.slave_id {
            Some(value) => format!("Slave id: {}", value),
            None => "Slave id: None".to_string(),
        },
        match app.modbus_request_data.start_addr {
            Some(value) => format!("Start register: {}", value),
            None => "Start register: None".to_string(),
        },
        match app.modbus_request_data.quantity {
            Some(value) => format!("Quantity: {}", value),
            None => "Quantity: None".to_string(),
        },
    ];

    frame.render_stateful_widget(
        list_window("Add modbus register", items.to_vec()),
        popup,
        list_state,
    );
}

/// Popup configure connection settings to gateway
fn render_connection_settings(frame: &mut Frame, app: &AppState, list_state: &mut ListState) {
    let popup = centered_rect(frame);

    frame.render_widget(Clear, popup);

    let items = [
        format!("IP-adress: {}", app.connection_settings.ip_adress),
        format!("Port: {}", app.connection_settings.port),
        match app.connection_settings.poll_time {
            Some(value) => format!("Modbus requests delay: {}ms", value),
            None => String::from("Modbus requests delay: ---ms"),
        },
    ];

    frame.render_stateful_widget(
        list_window("Gateway connection settings", items.to_vec()),
        popup,
        list_state,
    );
}

/// Main render function
pub fn render(frame: &mut Frame, app: &AppState, list_state: &mut ListState) {
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
        .title(" Rust Modbus TCP ")
        .title_alignment(Alignment::Center)
        .title_bottom(instructions.centered())
        .borders(Borders::ALL);

    let area = main_block.inner(frame.area());

    frame.render_widget(main_block, frame.area());

    let outer_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(25), Constraint::Percentage(75)])
        .split(area);

    let inner_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(25), Constraint::Percentage(75)])
        .split(outer_layout[1]);

    let items: Vec<ListItem> = app
        .modbus_requests
        .iter()
        .map(|request| ListItem::new(request.as_string()))
        .collect();

    let list = List::new(items)
        .block(
            Block::new()
                .title(" Modbus register ")
                .bold()
                .fg(Color::Blue)
                .borders(Borders::ALL),
        )
        .style(Color::Yellow);
    frame.render_widget(list, outer_layout[0]);

    let connection_status: &str;
    let connection_color: Color;
    let standard_color = Color::Yellow;

    match app.connection_status {
        ConnectionStatus::Disconnected => {
            connection_status = "DISCONNECTED";
            connection_color = Color::Red;
        }
        ConnectionStatus::Connected => {
            connection_status = "CONNECTED";
            connection_color = Color::Green;
        }
        ConnectionStatus::ConnectionErrorTimeOut => {
            connection_status = "CONNECTION FAILED!!";
            connection_color = Color::Red;
        }
        ConnectionStatus::InitilizeConnection => {
            connection_status = "Trying to connect...";
            connection_color = Color::Red;
        }
    }

    let data_box = Text::from(vec![
        line_helper(
            "IP-adress: ",
            app.connection_settings.ip_adress.clone(),
            standard_color,
        ),
        line_helper(
            "Port: ",
            app.connection_settings.port.clone(),
            standard_color,
        ),
        line_helper(
            "Polling time: ",
            format!("{}", app.connection_settings.poll_time.unwrap_or(0)),
            standard_color,
        ),
        line_helper("Poll counter: ", app.counter.to_string(), standard_color),
        line_helper("Connection stats: ", connection_status, connection_color),
    ]);

    // Widget display modbus gateway data
    frame.render_widget(
        Paragraph::new(data_box).block(Block::new().bold().fg(Color::Blue).borders(Borders::ALL)),
        inner_layout[0],
    );

    let rows: Vec<Row> = app
        .modbus_data
        .chunks(5)
        .map(|chunk| {
            let mut cols = Vec::new();

            for r in chunk {
                cols.push(r.register.to_string());
                cols.push(r.data.to_string());
            }

            Row::new(cols).style(Style::default().fg(Color::Yellow))
        })
        .collect();

    let table = Table::new(rows, [10, 8, 10, 8, 10, 8, 10, 8, 10, 8])
        .header(
            Row::new(vec![
                "Reg", "Val", "Reg", "Val", "Reg", "Val", "Reg", "Val", "Reg", "Val",
            ])
            .style(Style::default().fg(Color::Blue).bold()),
        )
        .block(
            Block::new()
                .title(" Modbus data ")
                .borders(Borders::ALL)
                .fg(Color::Blue),
        );

    // Widget display modbus data
    frame.render_widget(table, inner_layout[1]);

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
