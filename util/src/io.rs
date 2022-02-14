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

fn build_client() -> Client {
    let session_token = read_session_token();
    let session_cookie = format!("session={}", &session_token);
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

fn aoc_url(year: u32, day: u32) -> String {
    format!("{}/{}/day/{}", AOC_URL, year, day)
}

fn get_from_aoc(url: &str) -> Result<String, reqwest::Error> {
    let client = build_client();
    let response = client.get(url).send()?;
    let response = response.error_for_status()?;
    response.text()
}

fn parse_feedback(s: &str) -> (String, bool) {
    let main_begin = match s.find("<main>") {
        Some(i) => i + "<main>".len(),
        None => 0,
    };
    let main_len = match s[main_begin..].find("</main>") {
        Some(i) => i,
        None => s.len(),
    };
    let feedback = &s[main_begin..main_begin + main_len];
    let is_wrong = !feedback.to_lowercase().contains("that's the right answer");
    (feedback.to_owned(), is_wrong)
}

fn post_to_aoc(url: &str, level: u32, answer: &str) -> (String, bool) {
    let client = build_client();
    let form = [("answer", answer), ("level", &format!("{}", level))];
    let response = match client.post(url).form(&form).send() {
        Ok(res) => res,
        Err(e) => panic!("failed POSTing answer to {:?}: {}", url, e),
    };
    parse_feedback(&response.text().expect("failed parsing response to POST"))
}

pub fn get(year: u32, day: u32) -> String {
    let input_dir: PathBuf =
        [&get_workdir(), "..", "txt", "input", &format!("{}", year)]
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
        let url = aoc_url(year, day) + "/input";
        println!("input {} does not exist, fetching from {}", year, url);
        let input = get_from_aoc(&url).expect("failed GETting input");
        match fs::write(&path, input) {
            Ok(_) => (),
            Err(e) => panic!("failed to write input to {:?}: {}", path, e),
        }
    }
    match fs::read_to_string(&path) {
        Ok(input) => input.trim().to_owned(),
        Err(e) => panic!("failed to read input from {:?}: {}", path, e),
    }
}

pub fn post(year: u32, day: u32, level: u32, answers: &str) {
    let url = aoc_url(year, day) + "/answer";
    let (a, b) = answers.split_once(' ').expect("unable to split answers");
    let answer = match level {
        1 => a,
        2 => b,
        _ => panic!("will not POST unknown level {}", level),
    };
    println!(
        "POSTing year {} day {} level {} answer {} to {}",
        year, day, level, answer, url
    );
    let (feedback, is_wrong) = post_to_aoc(&url, level, answer);
    if is_wrong {
        eprintln!("{}", feedback);
        return;
    } else {
        println!("correct");
    }
    if level == 1 {
        return;
    }
    let correct_dir: PathBuf =
        [&get_workdir(), "..", "txt", "correct", &format!("{}", year)]
            .iter()
            .collect();
    if !correct_dir.is_dir() {
        panic!(
            "correct answers dir {:?} does not exist, it should be created manually",
            correct_dir
        );
    }
    let path = correct_dir.join(format!("{:02}.txt", day));
    if !path.exists() {
        println!("writing {:?}", path);
        match fs::write(&path, answers) {
            Ok(_) => (),
            Err(e) => {
                panic!("failed to write correct answer to {:?}: {}", path, e)
            }
        }
    }
}
