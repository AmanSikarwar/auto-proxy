use crate::{
    proxy::{GeneralProxy, ProxySettings},
    target::ProxyTarget,
};

use std::error::Error;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Write};

pub struct Apt;

impl Apt {
    const APT_PROXY_CONF_FILE: &'static str = "/etc/apt/apt.conf.d/99-proxy";

    fn generate_apt_proxy_content(settings: &[&ProxySettings]) -> String {
        let mut content = String::new();
        for setting in settings {
            if !setting.host.is_empty() && !setting.port.is_empty() {
                if let Some(auth) = &setting.auth {
                    let proxy_string = format!(
                        "Acquire::http::Proxy \"http://{}:{}\";\n\
                         Acquire::https::Proxy \"http://{}:{}\";\n",
                        setting.host, setting.port, setting.host, setting.port,
                    );

                    if !auth.username.is_empty() && !auth.password.is_empty() {
                        let auth_string = format!(
                            "Acquire::http::Proxy-Authorization \"basic {}\";\n\
                             Acquire::https::Proxy-Authorization \"basic {}\";\n",
                            base64::encode(format!("{}:{}", auth.username, auth.password)),
                            base64::encode(format!("{}:{}", auth.username, auth.password)),
                        );
                        content.push_str(&auth_string);
                    }

                    content.push_str(&proxy_string);
                }
            }
        }
        content
    }

    fn parse_apt_proxy_config(path: &str) -> Vec<ProxySettings> {
        let mut proxy_settings = Vec::new();
        let file = match File::open(path) {
            Ok(file) => file,
            Err(_) => return proxy_settings,
        };

        let reader = BufReader::new(file);

        for line in reader.lines() {
            if let Ok(line) = line {
                let line = line.trim();
                if line.starts_with("Acquire::http::Proxy")
                    || line.starts_with("Acquire::https::Proxy")
                {
                    if let Some(proxy) = line.split_whitespace().nth(2) {
                        if let Ok(proxy) = GeneralProxy::from_string(proxy) {
                            proxy_settings.push(proxy);
                        }
                    }
                }
            }
        }

        proxy_settings
    }
}

impl ProxyTarget for Apt {
    fn get(&self) -> Option<Vec<ProxySettings>> {
        let proxy_settings = Apt::parse_apt_proxy_config(Apt::APT_PROXY_CONF_FILE);

        if !proxy_settings.is_empty() {
            Some(proxy_settings)
        } else {
            None
        }
    }

    fn set(&self, settings: Vec<&ProxySettings>) -> Result<(), Box<dyn Error>> {
        let content = Apt::generate_apt_proxy_content(&settings);
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(Apt::APT_PROXY_CONF_FILE)?;

        file.write_all(content.as_bytes())?; // Convert content to &[u8] using as_bytes()
        Ok(())
    }

    fn unset(&self) -> Result<(), Box<dyn Error>> {
        match std::fs::remove_file(Apt::APT_PROXY_CONF_FILE) {
            Ok(_) => Ok(()),
            Err(e) => Err(Box::new(e)),
        }
    }
}
