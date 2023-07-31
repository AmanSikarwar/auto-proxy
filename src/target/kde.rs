use crate::{
    proxy::{ProxyAuth, ProxySettings},
    target::ProxyTarget,
};

use shellexpand;
use std::error::Error;
use std::process::Command;
use std::str::from_utf8;

pub struct KDE;

impl KDE {
    const KDE_CONFIG_FILE: &'static str = "~/.config/kioslaverc";

    fn generate_kde_proxy_content(proxy_settings: &ProxySettings) -> String {
        let mut content = String::new();

        let http_proxy = format!("http://{}:{}", proxy_settings.host, proxy_settings.port);
        let https_proxy = format!("https://{}:{}", proxy_settings.host, proxy_settings.port);

        content.push_str(&format!(
            "[Proxy Settings]\n\
             AuthMode=1\n\
             HttpProxy={}\n\
             HttpsProxy={}\n\
             NoProxy={}\n",
            http_proxy,
            https_proxy,
            proxy_settings.no_proxy.join(",")
        ));

        content
    }
}

impl ProxyTarget for KDE {
    fn get(&self) -> Option<Vec<ProxySettings>> {
        let kde_config_file = shellexpand::tilde(KDE::KDE_CONFIG_FILE).into_owned();
        let output = Command::new("kreadconfig5")
            .args(&[
                "--file",
                kde_config_file.as_str(),
                "--group",
                "Proxy Settings",
                "HttpProxy",
            ])
            .output()
            .ok()?;

        let http_proxy = from_utf8(&output.stdout).ok()?.trim().to_string();

        if http_proxy.is_empty() {
            // No proxy settings set
            return None;
        }

        let output = Command::new("kreadconfig5")
            .args(&[
                "--file",
                kde_config_file.as_str(),
                "--group",
                "Proxy Settings",
                "HttpsProxy",
            ])
            .output()
            .ok()?;

        let https_proxy = from_utf8(&output.stdout).ok()?.trim().to_string();

        let output = Command::new("kreadconfig5")
            .args(&[
                "--file",
                kde_config_file.as_str(),
                "--group",
                "Proxy Settings",
                "NoProxy",
            ])
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
            auth: None,               // KDE proxy settings don't provide authentication support
            protocols: vec!["http".to_string(), "https".to_string()],
            no_proxy,
        };

        Some(vec![proxy_settings])
    }

    fn set(&self, settings: Vec<&ProxySettings>) -> Result<(), Box<dyn Error>> {
        if let Some(proxy_settings) = settings.first() {
            // Set KDE proxy settings using kwriteconfig5
            let kde_config_file = shellexpand::tilde(KDE::KDE_CONFIG_FILE).into_owned();
            let content = KDE::generate_kde_proxy_content(proxy_settings);

            Command::new("kwriteconfig5")
                .args(&[
                    "--file",
                    kde_config_file.as_str(),
                    "--group",
                    "Proxy Settings",
                    "--key",
                    "HttpProxy",
                    proxy_settings.host.as_str(),
                ])
                .output()
                .ok();

            Command::new("kwriteconfig5")
                .args(&[
                    "--file",
                    kde_config_file.as_str(),
                    "--group",
                    "Proxy Settings",
                    "--key",
                    "HttpsProxy",
                    proxy_settings.host.as_str(),
                ])
                .output()
                .ok();

            Command::new("kwriteconfig5")
                .args(&[
                    "--file",
                    kde_config_file.as_str(),
                    "--group",
                    "Proxy Settings",
                    "--key",
                    "NoProxy",
                    proxy_settings.no_proxy.join(",").as_str(),
                ])
                .output()
                .ok();

            // KDE proxy settings don't provide authentication support
            // Ignoring setting auth.username and auth.password
        }

        Ok(())
    }

    fn unset(&self) -> Result<(), Box<dyn Error>> {
        // Unset KDE proxy settings using kwriteconfig5
        let kde_config_file = shellexpand::tilde(KDE::KDE_CONFIG_FILE).into_owned();

        Command::new("kwriteconfig5")
            .args(&[
                "--file",
                kde_config_file.as_str(),
                "--group",
                "Proxy Settings",
                "--key",
                "HttpProxy",
                "''", // Empty value to unset
            ])
            .output()
            .ok();

        Command::new("kwriteconfig5")
            .args(&[
                "--file",
                kde_config_file.as_str(),
                "--group",
                "Proxy Settings",
                "--key",
                "HttpsProxy",
                "''", // Empty value to unset
            ])
            .output()
            .ok();

        Command::new("kwriteconfig5")
            .args(&[
                "--file",
                kde_config_file.as_str(),
                "--group",
                "Proxy Settings",
                "--key",
                "NoProxy",
                "''", // Empty value to unset
            ])
            .output()
            .ok();

        Ok(())
    }
}
