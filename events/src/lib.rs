use colored::*;
use std::{fmt, time::Duration};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Position {
    Top,
    Bottom,
    Center,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Notification {
    pub size: u32,
    pub color: (u8, u8, u8),
    pub position: Position,
    pub content: String,
}

#[derive(Clone, Copy)]
pub enum Event<'a> {
    Remainder(&'a str),
    Registration(Duration),
    Appointment(&'a str),
    Holiday,
}

impl fmt::Display for Notification {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}, {}, {}", self.position, self.size, self.content)?;
        Ok(())
    }
}

impl Event<'_> {
    pub fn notify(self) -> Notification {
        match self {
            Event::Remainder(str) => Notification {
                size: 50,
                color: (50, 50, 50),
                position: Position::Bottom,
                content: str.to_string(),
            },
            Event::Registration(duration) => {
                let secs = duration.as_secs();
                let h = secs / 3600;
                let m = (secs % 3600) / 60;
                let s = secs % 60;

                Notification {
                size: 30,
                color: (255, 2, 22),
                position: Position::Top,
                content: format!("You have {}H:{}M:{}S left before the registration ends", h, m, s ),
            }},
            Event::Appointment(str) => Notification {
                size: 100,
                color: (200, 200, 3),
                position: Position::Center,
                content: str.to_string(),
            },
            Event::Holiday => Notification {
                size: 25,
                color: (0, 255, 0),
                position: Position::Top,
                content: "Enjoy your holiday".to_string(),
            },
        }
    }
}
