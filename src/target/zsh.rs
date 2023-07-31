use crate::{
    proxy::{GeneralProxy, ProxySettings},
    target::ProxyTarget,
};

use std::error::Error;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Write};

pub struct Zsh;

impl Zsh {
    fn generate_zshrc_content(settings: &[&ProxySettings]) -> String {
        let mut content = String::new();
        for setting in settings {
            if !setting.host.is_empty() && !setting.port.is_empty() {
                if let Some(auth) = &setting.auth {
                    let proxy_string = format!(
                        "export http_proxy=http://{}:{}\n\
                         export https_proxy=http://{}:{}\n\
                         export ftp_proxy=http://{}:{}\n\
                         export socks_proxy=http://{}:{}\n\
                         export all_proxy=http://{}:{}\n\
                         export no_proxy={}\n",
                        setting.host,
                        setting.port,
                        setting.host,
                        setting.port,
                        setting.host,
                        setting.port,
                        setting.host,
                        setting.port,
                        setting.host,
                        setting.port,
                        setting.no_proxy.join(",")
                    );

                    if !auth.username.is_empty() && !auth.password.is_empty() {
                        let auth_string = format!(
                            "export HTTP_PROXY_USER={}\n\
                             export HTTP_PROXY_PASS={}\n",
                            auth.username, auth.password
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

impl ProxyTarget for Zsh {
    fn get(&self) -> Option<Vec<ProxySettings>> {
        let zshrc_file = std::fs::read_to_string("~/.zshrc").unwrap_or_default();
        let mut proxy_settings = ProxySettings::default();

        for line in zshrc_file.lines() {
            let line = line.trim();
            if let Some((key, value)) = line.split_once('=') {
                match key.trim().to_ascii_uppercase().as_str() {
                    "HTTP_PROXY" | "HTTPS_PROXY" | "FTP_PROXY" | "SOCKS_PROXY" | "ALL_PROXY" => {
                        if let Ok(proxy) = GeneralProxy::from_string(value.trim()) {
                            if proxy_settings.host.is_empty() && proxy_settings.port.is_empty() {
                                proxy_settings.host = proxy.host().to_string();
                                proxy_settings.port = proxy.port().to_string();
                                proxy_settings.auth = proxy.auth().cloned();
                            }
                        }
                    }
                    "NO_PROXY" => {
                        proxy_settings
                            .no_proxy
                            .extend(value.split(',').map(str::trim).map(String::from));
                    }
                    _ => continue,
                }
            }
        }

        if !proxy_settings.host.is_empty() && !proxy_settings.port.is_empty() {
            Some(vec![proxy_settings])
        } else {
            None
        }
    }

    fn set(&self, settings: Vec<&ProxySettings>) -> Result<(), Box<dyn Error>> {
        let content = Zsh::generate_zshrc_content(&settings);
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open("~/.zshrc")?;

        file.write_all(content.as_bytes())?; // Convert content to &[u8] using as_bytes()
        Ok(())
    }

    fn unset(&self) -> Result<(), Box<dyn Error>> {
        let file_path = "~/.zshrc";
        let mut file = OpenOptions::new().read(true).write(true).open(file_path)?;

        let mut updated_content = String::new();
        let reader = BufReader::new(&file);
        let mut unset_lines = vec![];

        for line in reader.lines() {
            let line = line?;
            if !line.contains("export http_proxy")
                && !line.contains("export https_proxy")
                && !line.contains("export ftp_proxy")
                && !line.contains("export socks_proxy")
                && !line.contains("export all_proxy")
                && !line.contains("export HTTP_PROXY_USER")
                && !line.contains("export HTTP_PROXY_PASS")
            {
                updated_content.push_str(&line);
                updated_content.push('\n');
            } else {
                unset_lines.push(line);
            }
        }

        // If no changes are needed, return early
        if unset_lines.is_empty() {
            return Ok(());
        }

        // Write the updated content to the file
        file.set_len(0)?;
        file.seek(std::io::SeekFrom::Start(0))?;
        file.write_all(updated_content.as_bytes())?;

        Ok(())
    }
}
