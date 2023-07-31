use crate::{
    proxy::{GeneralProxy, ProxySettings},
    target::ProxyTarget,
};

use std::error::Error;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Write};

pub struct Fish;

impl Fish {
    fn generate_config_fish_content(settings: &[&ProxySettings]) -> String {
        let mut content = String::new();
        for setting in settings {
            if !setting.host.is_empty() && !setting.port.is_empty() {
                if let Some(auth) = &setting.auth {
                    let proxy_string = format!(
                        "set -x http_proxy http://{}:{}\n\
                         set -x https_proxy http://{}:{}\n\
                         set -x ftp_proxy http://{}:{}\n\
                         set -x socks_proxy http://{}:{}\n\
                         set -x all_proxy http://{}:{}\n\
                         set -x no_proxy {}\n",
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
                        setting.no_proxy.join(" ")
                    );

                    if !auth.username.is_empty() && !auth.password.is_empty() {
                        let auth_string = format!(
                            "set -x HTTP_PROXY_USER {}\n\
                             set -x HTTP_PROXY_PASS {}\n",
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

impl ProxyTarget for Fish {
    fn get(&self) -> Option<Vec<ProxySettings>> {
        let config_fish_file =
            std::fs::read_to_string("~/.config/fish/config.fish").unwrap_or_default();
        let mut proxy_settings = ProxySettings::default();

        for line in config_fish_file.lines() {
            let line = line.trim();
            if let Some((key, value)) = line.split_once(' ') {
                match key.trim() {
                    "set" => {
                        if let Some(proxy_type) = value.split_once(' ').map(|(t, _)| t) {
                            match proxy_type.trim() {
                                "-x" => {
                                    if let Some((var, value)) =
                                        value.split_once(' ').map(|(v, _)| v)
                                    {
                                        match var.trim() {
                                            "http_proxy" | "https_proxy" | "ftp_proxy"
                                            | "socks_proxy" | "all_proxy" => {
                                                if let Ok(proxy) =
                                                    GeneralProxy::from_string(value.trim())
                                                {
                                                    if proxy_settings.host.is_empty()
                                                        && proxy_settings.port.is_empty()
                                                    {
                                                        proxy_settings.host =
                                                            proxy.host().to_string();
                                                        proxy_settings.port =
                                                            proxy.port().to_string();
                                                        proxy_settings.auth = proxy.auth().cloned();
                                                    }
                                                }
                                            }
                                            "no_proxy" => {
                                                proxy_settings
                                                    .no_proxy
                                                    .extend(value.split(' ').map(String::from));
                                            }
                                            _ => continue,
                                        }
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

        if !proxy_settings.host.is_empty() && !proxy_settings.port.is_empty() {
            Some(vec![proxy_settings])
        } else {
            None
        }
    }

    fn set(&self, settings: Vec<&ProxySettings>) -> Result<(), Box<dyn Error>> {
        let content = Fish::generate_config_fish_content(&settings);
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open("~/.config/fish/config.fish")?;

        file.write_all(content.as_bytes())?; // Convert content to &[u8] using as_bytes()
        Ok(())
    }

    fn unset(&self) -> Result<(), Box<dyn Error>> {
        let file_path = "~/.config/fish/config.fish";
        let mut file = OpenOptions::new().read(true).write(true).open(file_path)?;

        let mut updated_content = String::new();
        let reader = BufReader::new(&file);
        let mut unset_lines = vec![];

        for line in reader.lines() {
            let line = line?;
            if !line.contains("set -x http_proxy")
                && !line.contains("set -x https_proxy")
                && !line.contains("set -x ftp_proxy")
                && !line.contains("set -x socks_proxy")
                && !line.contains("set -x all_proxy")
                && !line.contains("set -x HTTP_PROXY_USER")
                && !line.contains("set -x HTTP_PROXY_PASS")
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
