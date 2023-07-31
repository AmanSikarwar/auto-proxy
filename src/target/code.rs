use crate::{
    proxy::{GeneralProxy, ProxySettings},
    target::ProxyTarget,
};

use serde_json::Value;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct VSCode;

impl VSCode {
    const VS_CODE_SETTINGS_FILE: &'static str = "~/.config/Code/User/settings.json";

    fn parse_vscode_proxy_config(path: &str) -> Vec<ProxySettings> {
        let mut proxy_settings = Vec::new();
        let file = match File::open(path) {
            Ok(file) => file,
            Err(_) => return proxy_settings,
        };

        let reader = BufReader::new(file);
        let json_content: Value = match serde_json::from_reader(reader) {
            Ok(content) => content,
            Err(_) => return proxy_settings,
        };

        if let Some(http_proxy) = json_content["http.proxy"].as_str() {
            if let Ok(proxy) = GeneralProxy::from_string(http_proxy) {
                let mut setting = ProxySettings::default();
                setting.host = proxy.host().to_string();
                setting.port = proxy.port().to_string();
                if let Some(http_proxy_strict_ssl) = json_content["http.proxyStrictSSL"].as_bool() {
                    if http_proxy_strict_ssl {
                        setting.auth = Some(ProxyAuth {
                            username: String::from("username"), // Replace with actual username
                            password: String::from("password"), // Replace with actual password
                        });
                    }
                }
                proxy_settings.push(setting);
            }
        }

        proxy_settings
    }
}

impl ProxyTarget for VSCode {
    fn get(&self) -> Option<Vec<ProxySettings>> {
        let proxy_settings = VSCode::parse_vscode_proxy_config(VSCode::VS_CODE_SETTINGS_FILE);

        if !proxy_settings.is_empty() {
            Some(proxy_settings)
        } else {
            None
        }
    }

    fn set(&self, settings: Vec<&ProxySettings>) -> Result<(), Box<dyn Error>> {
        let json_content = VSCode::generate_vscode_proxy_content(&settings);
        let content_str = serde_json::to_string_pretty(&json_content)?;

        let settings_file_path = shellexpand::tilde(VSCode::VS_CODE_SETTINGS_FILE).into_owned();
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(settings_file_path)?;

        file.write_all(content_str.as_bytes())?;
        Ok(())
    }

    fn unset(&self) -> Result<(), Box<dyn Error>> {
        let settings_file_path = shellexpand::tilde(VSCode::VS_CODE_SETTINGS_FILE).into_owned();
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(settings_file_path)?;

        let mut content = String::new();
        file.read_to_string(&mut content)?;

        let json_content: Value = match serde_json::from_str(&content) {
            Ok(json) => json,
            Err(_) => return Err("Error parsing settings.json".into()),
        };

        let updated_json_content = VSCode::remove_vscode_proxy_settings(json_content);
        let updated_content_str = serde_json::to_string_pretty(&updated_json_content)?;

        file.set_len(0)?;
        file.seek(std::io::SeekFrom::Start(0))?;
        file.write_all(updated_content_str.as_bytes())?;

        Ok(())
    }
}
