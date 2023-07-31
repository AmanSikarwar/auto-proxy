use clap::{Arg, Command};
use dialoguer::{theme::ColorfulTheme, Confirm, Input, MultiSelect};
use regex::Regex;

use crate::{
    network::get_saved_wifi_networks,
    proxy::{ProxyAuth, ProxyProtocol, ProxySettings},
    proxy_profile::ProxyProfile,
};

pub fn cli() -> Command {
    Command::new("auto-proxy")
        .version("0.1.0")
        .author("Aman Sikarwar <amansikarwaar@gmail.com>")
        .about("Set system-wide proxy automatically")
        .arg_required_else_help(true)
        .subcommands([
            Command::new("config")
                .about("Configure proxy")
                .arg_required_else_help(true)
                .subcommands([
                    Command::new("new")
                        .about("Create a new proxy configuration")
                        .arg(
                            Arg::new("config-name")
                                .help("Name of the proxy configuration")
                                .required(false),
                        ),
                    Command::new("use").about("Use a proxy configuration").arg(
                        Arg::new("config-name")
                            .help("Name of the proxy configuration")
                            .required(true),
                    ),
                    Command::new("delete")
                        .about("Delete a proxy configuration")
                        .arg(
                            Arg::new("config-name")
                                .help("Name of the proxy configuration")
                                .required(true),
                        ),
                    Command::new("list").about("List all proxy configurations"),
                    Command::new("show").about("Show current proxy configuration"),
                ]),
            Command::new("set").about("Set proxy").args([
                Arg::new("http-host")
                    .help("HTTP host")
                    .long("http-host")
                    .required(false),
                Arg::new("http-port")
                    .default_value("8080")
                    .help("HTTP port")
                    .long("http-port")
                    .required(false),
                Arg::new("https-host")
                    .default_value("http-host")
                    .help("HTTPS host")
                    .long("https-host")
                    .required(false),
                Arg::new("https-port")
                    .default_value("8080")
                    .help("HTTPS port")
                    .long("https-port")
                    .required(false),
                Arg::new("ftp-host")
                    .default_value("http-host")
                    .help("FTP host")
                    .long("ftp-host")
                    .required(false),
                Arg::new("ftp-port")
                    .default_value("8080")
                    .help("FTP port")
                    .long("ftp-port")
                    .required(false),
                Arg::new("socks-host")
                    .default_value("http-host")
                    .help("SOCKS host")
                    .long("socks-host")
                    .required(false),
                Arg::new("socks-port")
                    .default_value("8080")
                    .help("SOCKS port")
                    .long("socks-port")
                    .required(false),
                Arg::new("no-proxy")
                    .default_value("localhost, 127.0.0.1, 192.168.1.1, ::1, *.local")
                    .help("No proxy")
                    .long("no-proxy")
                    .required(false),
                Arg::new("username")
                    .help("Username for proxy authentication")
                    .long("username")
                    .required(false),
                Arg::new("password")
                    .help("Password for proxy authentication")
                    .long("password")
                    .required(false),
            ]),
            Command::new("unset").about("Unset proxy"),
            Command::new("show").about("Show current proxy"),
            Command::new("auto-apply").about("Apply proxy automatically based on network"),
            Command::new("setup").about("Setup auto-proxy"),
        ])
}

pub fn prompt_new_config(name: Option<String>) -> ProxyProfile {
    let proxy_host: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Proxy host")
        .validate_with(|input: &String| -> Result<(), &str> {
            if input.is_empty() {
                return Err("Proxy host cannot be empty");
            }
            let re = Regex::new(r"^([a-zA-Z0-9]|[a-zA-Z0-9][a-zA-Z0-9\-]*[a-zA-Z0-9])$").unwrap();
            if !re.is_match(input) {
                return Err("Invalid proxy host");
            }
            Ok(())
        })
        .interact_text()
        .unwrap();

    let proxy_port: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Proxy port")
        .default("8080".to_string())
        .validate_with(|input: &String| -> Result<(), &str> {
            if input.is_empty() {
                return Err("Proxy port cannot be empty");
            }
            let re = Regex::new(r"^\d+$").unwrap();
            if !re.is_match(input) {
                return Err("Invalid Proxy port");
            }
            Ok(())
        })
        .interact_text()
        .unwrap();

    let protocols = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select protocols to use")
        .items(ProxyProtocol::all())
        .defaults(&[true, true, false, false])
        .interact()
        .unwrap();

    let selected_protocols = protocols
        .iter()
        .map(|&x| ProxyProtocol::all()[x])
        .collect::<Vec<ProxyProtocol>>();

    let use_auth: bool = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Use Authentication?")
        .default(false)
        .interact()
        .unwrap();

    let mut auth_info: Option<ProxyAuth> = None;

    if use_auth {
        auth_info = Some(prompt_auth());
    }

    let no_proxy = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("No proxy")
        .default("localhost,127.0.0.1,192.168.1.1,::1,*.local".to_string())
        .validate_with(|input: &String| -> Result<(), &str> {
            if input.is_empty() {
                return Err("No proxy cannot be empty");
            }
            Ok(())
        })
        .interact_text()
        .unwrap();

    let no_proxy_list = no_proxy
        .split(",")
        .map(|x| x.trim().to_string())
        .collect::<Vec<String>>();

    let network_list = get_saved_wifi_networks().unwrap();

    println!("List: {}", network_list[0]);

    let selected_networks_index = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select networks to apply proxy for")
        .items(&network_list)
        .interact()
        .unwrap();

    let selected_networks = selected_networks_index
        .iter()
        .map(|x| network_list[*x].clone())
        .collect::<Vec<String>>();

    let mut profile_name: String = String::new();

    if name.is_none() {
        profile_name = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Profile name")
            .validate_with(|input: &String| -> Result<(), &str> {
                if input.is_empty() {
                    return Err("Profile name cannot be empty");
                }
                Ok(())
            })
            .interact_text()
            .unwrap();
    } else {
        profile_name = name.unwrap();
    }

    let proxy_settings = ProxySettings::new(
        proxy_host,
        proxy_port,
        auth_info,
        selected_protocols,
        no_proxy_list,
    );

    let profile = ProxyProfile::new(profile_name, proxy_settings, selected_networks);

    profile
}

fn prompt_auth() -> ProxyAuth {
    let username: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Username")
        .validate_with(|input: &String| -> Result<(), &str> {
            if input.is_empty() {
                return Err("Username cannot be empty");
            }
            Ok(())
        })
        .interact_text()
        .unwrap();

    let password: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Password")
        .validate_with(|input: &String| -> Result<(), &str> {
            if input.is_empty() {
                return Err("Password cannot be empty");
            }
            Ok(())
        })
        .interact_text()
        .unwrap();

    ProxyAuth::new(username, password)
}
