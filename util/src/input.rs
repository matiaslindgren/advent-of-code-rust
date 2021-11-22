use reqwest::{blocking::Client, cookie::Jar, Url};
use std::{env, fs, path::PathBuf, sync::Arc};

const AOC_URL: &str = "https://adventofcode.com";

fn get_workdir() -> String {
    match env::var("AOC_WORKDIR") {
        Ok(dir) => dir,
        Err(_) => ".".to_owned(),
    }
}

fn read_session_token() -> String {
    let path: PathBuf = [&get_workdir(), "session_token"].iter().collect();
    match fs::read_to_string(&path) {
        Ok(token) => token,
        Err(e) => panic!("failed to read session token from {:?}: {}", path, e),
    }
}

fn build_client(session_token: &str) -> Client {
    let session_cookie = format!("session={}", session_token);
    let aoc_url = str::parse::<Url>(AOC_URL).unwrap();
    let jar = Jar::default();
    jar.add_cookie_str(&session_cookie, &aoc_url);
    Client::builder()
        .cookie_store(true)
        .cookie_provider(Arc::new(jar))
        .https_only(true)
        .build()
        .unwrap()
}

fn aoc_input_url(year: u32, day: u32) -> String {
    format!("{}/{}/day/{}/input", AOC_URL, year, day)
}

fn get_from_aoc(url: &str) -> String {
    let session_token = read_session_token();
    let client = build_client(&session_token);
    let response = match client.get(url).send() {
        Ok(res) => res,
        Err(e) => panic!("failed getting input from {}: {}", AOC_URL, e),
    };
    match response.text() {
        Ok(text) => text,
        Err(e) => panic!("failed parsing response: {}", e),
    }
}

pub fn get(year: u32, day: u32) -> String {
    let input_dir: PathBuf = [&get_workdir(), "..", "input", &format!("{}", year)]
        .iter()
        .collect();
    if !input_dir.is_dir() {
        panic!(
            "input dir {:?} does not exist, it should be created manually",
            input_dir
        );
    }
    let path = input_dir.join(format!("{:02}.txt", day));
    if !path.exists() {
        let url = aoc_input_url(year, day);
        println!("input {} does not exist, fetching from {}", year, url);
        match fs::write(&path, get_from_aoc(&url)) {
            Ok(_) => (),
            Err(e) => panic!("failed to write input to {:?}: {}", path, e),
        }
    }
    match fs::read_to_string(&path) {
        Ok(input) => input,
        Err(e) => panic!("failed to read input from {:?}: {}", path, e),
    }
}
