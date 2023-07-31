use crate::{
    proxy::{ProxyAuth, ProxySettings},
    target::ProxyTarget,
};

use std::error::Error;
use std::process::Command;
use std::str::from_utf8;

pub struct Npm;

impl Npm {
    fn generate_npm_proxy_command(proxy_settings: &ProxySettings) -> Vec<(&'static str, String)> {
        let mut command_list = Vec::new();

        if !proxy_settings.host.is_empty() && !proxy_settings.port.is_empty() {
            let http_proxy = format!("http://{}:{}", proxy_settings.host, proxy_settings.port);
            let https_proxy = format!("https://{}:{}", proxy_settings.host, proxy_settings.port);

            command_list.push(("proxy", http_proxy));
            command_list.push(("https-proxy", https_proxy));
        }

        if !proxy_settings.no_proxy.is_empty() {
            let no_proxy = proxy_settings.no_proxy.join(",");
            command_list.push(("noproxy", no_proxy));
        }

        command_list
    }
}

impl ProxyTarget for Npm {
    fn get(&self) -> Option<Vec<ProxySettings>> {
        let output = Command::new("npm")
            .args(&["config", "get", "proxy"])
            .output()
            .ok()?;

        let http_proxy = from_utf8(&output.stdout).ok()?.trim().to_string();

        if http_proxy.is_empty() {
            // No proxy settings set
            return None;
        }

        let output = Command::new("npm")
            .args(&["config", "get", "https-proxy"])
            .output()
            .ok()?;

        let https_proxy = from_utf8(&output.stdout).ok()?.trim().to_string();

        let output = Command::new("npm")
            .args(&["config", "get", "noproxy"])
            .output()
            .ok()?;

        let no_proxy = from_utf8(&output.stdout)
            .ok()?
            .trim()
            .split(',')
            .map(|s| s.to_string())
            .collect();

        let proxy_settings = ProxySettings {
            host: http_proxy.to_string(),
            port: "8080".to_string(), // Replace with the actual port if needed
            auth: None,               // npm proxy settings don't provide authentication support
            protocols: vec!["http".to_string(), "https".to_string()],
            no_proxy,
        };

        Some(vec![proxy_settings])
    }

    fn set(&self, settings: Vec<&ProxySettings>) -> Result<(), Box<dyn Error>> {
        if let Some(proxy_settings) = settings.first() {
            // Set npm proxy settings using npm config command
            let command_list = Npm::generate_npm_proxy_command(proxy_settings);

            for (key, value) in command_list {
                Command::new("npm")
                    .args(&["config", "set", key, value.as_str()])
                    .output()
                    .ok();
            }
        }

        Ok(())
    }

    fn unset(&self) -> Result<(), Box<dyn Error>> {
        // Unset npm proxy settings using npm config command
        Command::new("npm")
            .args(&["config", "delete", "proxy"])
            .output()
            .ok();

        Command::new("npm")
            .args(&["config", "delete", "https-proxy"])
            .output()
            .ok();

        Command::new("npm")
            .args(&["config", "delete", "noproxy"])
            .output()
            .ok();

        Ok(())
    }
}
