use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
    DefaultTerminal, Frame,
};
use std::io::{self, Write};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

#[derive(Debug)]
pub struct App {
    counter: u8,
    exit: bool,
    connect_requested: bool,
    ip_address_input: String,
    poll_time_input: String,
    active_field: InputField,
}

#[derive(Debug)]
enum InputField {
    IpAddress,
    PollTime,
}

pub enum AppCommand {
    Connect,
}

impl Default for App {
    fn default() -> Self {
        Self {
            counter: 0, 
            exit: false, 
            connect_requested: false, 
            ip_address_input: "127.0.0.1:502".to_string(), 
            poll_time_input: "1500".to_string(), 
            active_field: InputField::IpAddress, 
        }
    }
}


impl App {

    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    pub fn should_exit(&self) -> bool {
    self.exit
    }

    pub fn connect_requested(&self) -> bool {
        self.connect_requested
    }

    pub fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    pub fn handle_events(&mut self) -> io::Result<Option<AppCommand>> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(Some(AppCommand::Connect))
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit = true,
            
            KeyCode::Char(c) => {
                match self.active_field {
                    InputField::IpAddress => self.ip_address_input.push(c),
                    InputField::PollTime => self.poll_time_input.push(c),
                }
            }

            KeyCode::Backspace => {
                match self.active_field {
                    InputField::IpAddress => { self.ip_address_input.pop(); }
                    InputField::PollTime => { self.poll_time_input.pop(); }
                }
            }

            KeyCode::Tab => {
                self.active_field = match self.active_field {
                    InputField::IpAddress => InputField::PollTime,
                    InputField::PollTime => InputField::IpAddress,
                };
            }     

            KeyCode::Enter => {
                if self.connect_requested {
                    self.connect_requested = false
                } else {
                    self.connect_requested = true
                }
            }
                
            _ => {}
        }
    }
}




impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" rust-modbus-tcp ".bold());
        let instructions = Line::from(vec![           
            " Quit ".into(),
            "<Q> ".blue().bold(),
            " Connect ".into(), 
            "<Enter> ".blue().bold()
        ]);

        let ip_style = match self.active_field {
            InputField::IpAddress => self.ip_address_input.as_str().black().on_yellow(),
            _ => self.ip_address_input.as_str().yellow(),
        }; 

        let poll_style = match self.active_field {
            InputField::PollTime => self.poll_time_input.as_str().black().on_yellow(), 
            _ => self.poll_time_input.as_str().yellow(), 
        }; 



        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        let text = Text::from(vec![
            Line::from(vec![
            "   IP Modbus Gateway: ".into(),
            ip_style,
            ]),
            Line::from(vec![
            "   Poll time in ms: ".into(),
            poll_style,
            ]),                            
        
        
        ]);


        Paragraph::new(text)
            .left_aligned()
            .block(block)
            .render(area, buf);
        
    }
}


mod tests {
    use super::*;
    use ratatui::style::Style;

    #[test]
    fn render() {
        let app = App::default();
        let mut buf = Buffer::empty(Rect::new(0, 0, 50, 4));

        app.render(buf.area, &mut buf);

        let mut expected = Buffer::with_lines(vec![
            "┏━━━━━━━━━━━━━ Counter App Tutorial ━━━━━━━━━━━━━┓",
            "┃                    Value: 0                    ┃",
            "┃                                                ┃",
            "┗━ Decrement <Left> Increment <Right> Quit <Q> ━━┛",
        ]);
        let title_style = Style::new().bold();
        let counter_style = Style::new().yellow();
        let key_style = Style::new().blue().bold();
        expected.set_style(Rect::new(14, 0, 22, 1), title_style);
        expected.set_style(Rect::new(28, 1, 1, 1), counter_style);
        expected.set_style(Rect::new(13, 3, 6, 1), key_style);
        expected.set_style(Rect::new(30, 3, 7, 1), key_style);
        expected.set_style(Rect::new(43, 3, 4, 1), key_style);

        assert_eq!(buf, expected);
    }
    #[test]
    fn handle_key_event() {
        let mut app = App::default();
        app.handle_key_event(KeyCode::Right.into());
        assert_eq!(app.counter, 1);

        app.handle_key_event(KeyCode::Left.into());
        assert_eq!(app.counter, 0);

        let mut app = App::default();
        app.handle_key_event(KeyCode::Char('q').into());
        assert!(app.exit);
    }    
}