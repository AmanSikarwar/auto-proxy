use crate::{
    proxy::{GeneralProxy, ProxySettings},
    target::ProxyTarget,
};

use std::error::Error;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Write};

pub struct Dnf;

impl Dnf {
    const DNF_PROXY_CONF_FILE: &'static str = "/etc/dnf/dnf.conf";

    fn generate_dnf_proxy_content(settings: &[&ProxySettings]) -> String {
        let mut content = String::new();
        for setting in settings {
            if !setting.host.is_empty() && !setting.port.is_empty() {
                if let Some(auth) = &setting.auth {
                    let proxy_string = format!(
                        "[main]\n\
                         proxy=http://{}:{}\n\
                         proxy_username={}\n\
                         proxy_password={}\n",
                        setting.host, setting.port, auth.username, auth.password,
                    );
                    content.push_str(&proxy_string);
                }
            }
        }
        content
    }
}

impl ProxyTarget for Dnf {
    fn get(&self) -> Option<Vec<ProxySettings>> {
        let mut proxy_settings = Vec::new();
        let file = match std::fs::File::open(Dnf::DNF_PROXY_CONF_FILE) {
            Ok(file) => file,
            Err(_) => return None,
        };

        let reader = BufReader::new(file);
        let mut in_main_section = false;

        for line in reader.lines() {
            let line = line.unwrap();
            let trimmed_line = line.trim();
            if trimmed_line.starts_with("[main]") {
                in_main_section = true;
            } else if in_main_section && trimmed_line.starts_with("proxy=") {
                if let Some(proxy) = trimmed_line.split("=").nth(1) {
                    if let Ok(proxy) = GeneralProxy::from_string(proxy) {
                        proxy_settings.push(proxy);
                    }
                }
            } else if in_main_section && trimmed_line.is_empty() {
                // Reached the end of the [main] section
                break;
            }
        }

        if !proxy_settings.is_empty() {
            Some(proxy_settings)
        } else {
            None
        }
    }

    fn set(&self, settings: Vec<&ProxySettings>) -> Result<(), Box<dyn Error>> {
        let content = Dnf::generate_dnf_proxy_content(&settings);
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(Dnf::DNF_PROXY_CONF_FILE)?;

        file.write_all(content.as_bytes())?; // Convert content to &[u8] using as_bytes()
        Ok(())
    }

    fn unset(&self) -> Result<(), Box<dyn Error>> {
        match std::fs::remove_file(Dnf::DNF_PROXY_CONF_FILE) {
            Ok(_) => Ok(()),
            Err(e) => Err(Box::new(e)),
        }
    }
}
