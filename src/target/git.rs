use crate::{
    proxy::{GeneralProxy, ProxySettings},
    target::ProxyTarget,
};

use std::error::Error;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Write};

pub struct Git;

impl Git {
    fn parse_gitconfig_file(path: &str) -> Vec<ProxySettings> {
        let mut proxy_settings = Vec::new();
        let file = match std::fs::File::open(path) {
            Ok(file) => file,
            Err(_) => return proxy_settings,
        };

        let reader = BufReader::new(file);
        let mut current_section = None;

        for line in reader.lines() {
            if let Ok(line) = line {
                let line = line.trim();
                if line.starts_with("[") && line.ends_with("]") {
                    current_section = Some(line.to_string());
                } else if let Some(section) = &current_section {
                    match section.as_str() {
                        "[http]" | "[https]" => {
                            if let Some((key, value)) = line.split_once("=") {
                                match key.trim() {
                                    "proxy" => {
                                        if let Ok(proxy) = GeneralProxy::from_string(value.trim()) {
                                            proxy_settings.push(proxy);
                                        }
                                    }
                                    _ => continue,
                                }
                            }
                        }
                        _ => continue,
                    }
                }
            }
        }

        proxy_settings
    }

    fn generate_gitconfig_content(settings: &[&ProxySettings]) -> String {
        let mut content = String::new();
        for setting in settings {
            if !setting.host.is_empty() && !setting.port.is_empty() {
                if let Some(auth) = &setting.auth {
                    let proxy_string = format!(
                        "[http]\n\
                         \tproxy = http://{}:{}\n\
                         [https]\n\
                         \tproxy = http://{}:{}\n",
                        setting.host, setting.port, setting.host, setting.port,
                    );

                    if !auth.username.is_empty() && !auth.password.is_empty() {
                        let auth_string = format!(
                            "[http]\n\
                             \tproxyAuth = {}:{}\n\
                             [https]\n\
                             \tproxyAuth = {}:{}\n",
                            auth.username, auth.password, auth.username, auth.password
                        );
                        content.push_str(&auth_string);
                    }

                    content.push_str(&proxy_string);
                }
            }
        }
        content
    }
}

impl ProxyTarget for Git {
    fn get(&self) -> Option<Vec<ProxySettings>> {
        let global_gitconfig_path = format!("{}/.gitconfig", dirs::home_dir()?.display());
        let mut proxy_settings = Git::parse_gitconfig_file(&global_gitconfig_path);

        if !proxy_settings.is_empty() {
            Some(proxy_settings)
        } else {
            None
        }
    }

    fn set(&self, settings: Vec<&ProxySettings>) -> Result<(), Box<dyn Error>> {
        let content = Git::generate_gitconfig_content(&settings);
        let global_gitconfig_path = format!("{}/.gitconfig", dirs::home_dir()?.display());
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(global_gitconfig_path)?;

        file.write_all(content.as_bytes())?; // Convert content to &[u8] using as_bytes()
        Ok(())
    }

    fn unset(&self) -> Result<(), Box<dyn Error>> {
        let global_gitconfig_path = format!("{}/.gitconfig", dirs::home_dir()?.display());
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(global_gitconfig_path)?;

        let mut updated_content = String::new();
        let reader = BufReader::new(&file);
        let mut in_http_section = false;

        for line in reader.lines() {
            let line = line?;
            if !line.starts_with("proxy =") && !line.starts_with("proxyAuth =") {
                // Keep all lines except for proxy and proxyAuth settings
                updated_content.push_str(&line);
                updated_content.push('\n');
            }

            if line.starts_with("[http]") {
                // Found the start of [http] section, skip lines until end of section
                in_http_section = true;
            } else if in_http_section && line.starts_with('[') {
                // Reached the end of [http] section, stop skipping lines
                in_http_section = false;
            }
        }

        // If no changes are needed, return early
        if updated_content.is_empty() {
            return Ok(());
        }

        // Write the updated content to the file
        file.set_len(0)?;
        file.seek(std::io::SeekFrom::Start(0))?;
        file.write_all(updated_content.as_bytes())?;

        Ok(())
    }
}
