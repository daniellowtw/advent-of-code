use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;
use std::io::{self};
use std::path::PathBuf;

const BASE_URL: &str = "https://adventofcode.com";
const CONFIG_FILE: &str = "aoc_config.json";

#[derive(Serialize, Deserialize)]
struct Config {
    session_cookie: String,
}

fn config_file_path() -> PathBuf {
    dirs::config_dir()
        .expect("Unable to find config directory")
        .join(CONFIG_FILE)
}

pub fn get_example(year: u32, day: u32) -> Result<String, Box<dyn Error>> {
    let (client, headers) = get_client()?;
    let url = format!("{}/{}/day/{}", BASE_URL, year, day);
    let response = client.get(&url).headers(headers).send()?.text()?;
    let mut flag = false;
    let mut stringbuf = String::new();
    for line in response.lines() {
        if line.contains("<p>For example:</p>") {
            flag = true;
            continue;
        }
        if flag {
            if line.contains("</pre>") {
                break;
            } else {
                stringbuf.push_str(line.strip_prefix("<pre><code>").unwrap_or(line));
                stringbuf.push_str("\n");
            }
        }
    }
    return Ok(stringbuf);
}

pub fn get_puzzle_input(year: u32, day: u32) -> Result<String, Box<dyn std::error::Error>> {
    let (client, headers) = get_client()?;
    let url = format!("{}/{}/day/{}/input", BASE_URL, year, day);
    let response = client.get(&url).headers(headers).send()?.text()?;
    Ok(response)
}

fn get_client() -> Result<(Client, HeaderMap), Box<dyn Error>> {
    let config = load_or_create_config()?;
    let client = Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(
        USER_AGENT,
        HeaderValue::from_static("advent-of-code-fetcher"),
    );
    headers.insert(
        "Cookie",
        HeaderValue::from_str(&format!("session={}", config.session_cookie))?,
    );
    Ok((client, headers))
}

fn load_or_create_config() -> Result<Config, Box<dyn std::error::Error>> {
    let config_path = config_file_path();

    // Load existing config
    if config_path.exists() {
        let config: Config = serde_json::from_str(&fs::read_to_string(config_path)?)?;
        return Ok(config);
    }

    // Create new config if it doesn't exist
    println!("Session cookie not found. Please paste your Advent of Code session cookie:");
    let mut session_cookie = String::new();
    io::stdin().read_line(&mut session_cookie)?;
    let session_cookie = session_cookie.trim().to_string();

    let config = Config { session_cookie };
    fs::create_dir_all(config_path.parent().unwrap())?;
    fs::write(config_path, serde_json::to_string_pretty(&config)?)?;

    Ok(config)
}
