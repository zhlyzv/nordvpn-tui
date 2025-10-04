use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Country {
    pub name: String,
    pub display_name: String,
}

impl Country {
    pub fn new(name: String) -> Self {
        let display_name = name.replace('_', " ");
        Self { name, display_name }
    }
}

impl fmt::Display for Country {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display_name)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConnectionStatus {
    Connected {
        country: String,
        city: Option<String>,
        server: Option<String>,
        ip: Option<String>,
    },
    Disconnected,
    Connecting,
}

impl fmt::Display for ConnectionStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConnectionStatus::Connected {
                country,
                city,
                server,
                ..
            } => {
                if let Some(city) = city {
                    write!(f, "Connected to {}, {}", city, country)?;
                } else {
                    write!(f, "Connected to {}", country)?;
                }
                if let Some(server) = server {
                    write!(f, " ({})", server)?;
                }
                Ok(())
            }
            ConnectionStatus::Disconnected => write!(f, "Disconnected"),
            ConnectionStatus::Connecting => write!(f, "Connecting..."),
        }
    }
}
