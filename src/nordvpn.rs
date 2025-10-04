use crate::types::{ConnectionStatus, Country};
use color_eyre::Result;
use std::process::Command;

pub struct NordVPN;

impl NordVPN {
    /// Get list of available countries
    pub fn get_countries() -> Result<Vec<Country>> {
        let output = Command::new("nordvpn").arg("countries").output()?;

        if !output.status.success() {
            return Err(color_eyre::eyre::eyre!(
                "Failed to get countries: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let countries: Vec<Country> = stdout
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(|line| Country::new(line.trim().to_string()))
            .collect();

        Ok(countries)
    }

    /// Get current connection status
    pub fn get_status() -> Result<ConnectionStatus> {
        let output = Command::new("nordvpn").arg("status").output()?;

        if !output.status.success() {
            return Err(color_eyre::eyre::eyre!(
                "Failed to get status: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);

        // Parse the status output
        if stdout.contains("Status: Disconnected") {
            return Ok(ConnectionStatus::Disconnected);
        }

        if stdout.contains("Status: Connected") {
            let mut country = None;
            let mut city = None;
            let mut server = None;
            let mut ip = None;

            for line in stdout.lines() {
                let line = line.trim();
                if line.starts_with("Country:") {
                    country = Some(line.strip_prefix("Country:").unwrap().trim().to_string());
                } else if line.starts_with("City:") {
                    city = Some(line.strip_prefix("City:").unwrap().trim().to_string());
                } else if line.starts_with("Server:") || line.starts_with("Hostname:") {
                    server = line
                        .strip_prefix("Server:")
                        .or_else(|| line.strip_prefix("Hostname:"))
                        .map(|s| s.trim().to_string());
                } else if line.starts_with("IP:") {
                    ip = Some(line.strip_prefix("IP:").unwrap().trim().to_string());
                }
            }

            if let Some(country) = country {
                return Ok(ConnectionStatus::Connected {
                    country,
                    city,
                    server,
                    ip,
                });
            }
        }

        // Default to disconnected if we can't parse
        Ok(ConnectionStatus::Disconnected)
    }

    /// Connect to a country
    pub fn connect(country: &str) -> Result<()> {
        let output = Command::new("nordvpn")
            .arg("connect")
            .arg(country)
            .output()?;

        if !output.status.success() {
            return Err(color_eyre::eyre::eyre!(
                "Failed to connect: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }

        Ok(())
    }

    /// Disconnect from VPN
    pub fn disconnect() -> Result<()> {
        let output = Command::new("nordvpn").arg("disconnect").output()?;

        if !output.status.success() {
            return Err(color_eyre::eyre::eyre!(
                "Failed to disconnect: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }

        Ok(())
    }
}
