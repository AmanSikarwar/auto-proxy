use crate::{
    proxy::{ProxyAuth, ProxySettings},
    target::ProxyTarget,
};

use std::error::Error;
use std::process::Command;
use std::str::from_utf8;

pub struct Gnome;

impl ProxyTarget for Gnome {
    fn get(&self) -> Option<Vec<ProxySettings>> {
        // Get current GNOME proxy settings using gsettings
        let output = Command::new("gsettings")
            .args(&["get", "org.gnome.system.proxy", "mode"])
            .output()
            .ok()?;

        let mode = from_utf8(&output.stdout).ok()?.trim_matches('"');

        if mode == "none" {
            // No proxy settings set
            return None;
        }

        let output = Command::new("gsettings")
            .args(&["get", "org.gnome.system.proxy", "http"])
            .output()
            .ok()?;

        let http_proxy = from_utf8(&output.stdout).ok()?.trim_matches('"');

        let output = Command::new("gsettings")
            .args(&["get", "org.gnome.system.proxy", "https"])
            .output()
            .ok()?;

        let https_proxy = from_utf8(&output.stdout).ok()?.trim_matches('"');

        let output = Command::new("gsettings")
            .args(&["get", "org.gnome.system.proxy", "ftp"])
            .output()
            .ok()?;

        let ftp_proxy = from_utf8(&output.stdout).ok()?.trim_matches('"');

        let output = Command::new("gsettings")
            .args(&["get", "org.gnome.system.proxy", "socks"])
            .output()
            .ok()?;

        let socks_proxy = from_utf8(&output.stdout).ok()?.trim_matches('"');

        let output = Command::new("gsettings")
            .args(&["get", "org.gnome.system.proxy", "ignore-hosts"])
            .output()
            .ok()?;

        let no_proxy = from_utf8(&output.stdout).ok()?.trim_matches('"');

        let proxy_settings = ProxySettings {
            host: http_proxy.to_string(),
            port: "8080".to_string(), // Replace with the actual port if needed
            auth: None,               // GNOME proxy settings don't provide authentication support
            protocols: vec!["http".to_string(), "https".to_string()],
            no_proxy: no_proxy.split(',').map(|s| s.to_string()).collect(),
        };

        Some(vec![proxy_settings])
    }

    fn set(&self, settings: Vec<&ProxySettings>) -> Result<(), Box<dyn Error>> {
        if let Some(proxy_settings) = settings.first() {
            // Set GNOME proxy settings using gsettings
            let http_proxy = format!("http://{}:{}", proxy_settings.host, proxy_settings.port);
            let https_proxy = format!("https://{}:{}", proxy_settings.host, proxy_settings.port);

            Command::new("gsettings")
                .args(&["set", "org.gnome.system.proxy", "mode", "manual"])
                .output()
                .ok();

            Command::new("gsettings")
                .args(&["set", "org.gnome.system.proxy", "http", &http_proxy])
                .output()
                .ok();

            Command::new("gsettings")
                .args(&["set", "org.gnome.system.proxy", "https", &https_proxy])
                .output()
                .ok();

            // GNOME proxy settings don't provide authentication support
            // Ignoring setting auth.username and auth.password
        }

        Ok(())
    }

    fn unset(&self) -> Result<(), Box<dyn Error>> {
        // Unset GNOME proxy settings using gsettings
        Command::new("gsettings")
            .args(&["set", "org.gnome.system.proxy", "mode", "none"])
            .output()
            .ok();

        Ok(())
    }
}
