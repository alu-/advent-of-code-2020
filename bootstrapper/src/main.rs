use std::env::current_dir;
use std::error;
use std::fs::File;
use std::io::{Read, Write};

use chrono;
use chrono::Datelike;
use handlebars::Handlebars;
use reqwest;
use reqwest::Error;
use serde_json::json;
use toml;
use toml::Value;

pub fn main() {
    println!("Bootstrapping current day puzzle ...");

    let now = chrono::offset::Local::now();
    //let now = Utc.ymd(2019, 12, 1).and_hms(12, 0, 9);

    if now.month() == 12 && now.day() < 26 {
        create_source_file(&now.day());
        download_input(&now.year(), &now.day()).expect("Input download");
        download_description(&now.year(), &now.day()).expect("Description download");
    } else {
        println!("Tried to run bootstrapper outside the Advent. Better luck next year!");
    }
}

fn create_source_file(day: &u32) {
    println!("Creating rust file for day {}", day);

    let file_to_create = current_dir()
        .unwrap()
        .join("solutions")
        .join("src")
        .join(format!("day{:02}.rs", &day));

    if !file_to_create.exists() {
        let path_to_source = current_dir()
            .unwrap()
            .join("bootstrapper")
            .join("day_template.hbs");

        let mut template_source = File::open(&path_to_source).unwrap();
        let mut output_file = File::create(file_to_create).unwrap();
        let data = json!({"day": day});

        let handlebars = Handlebars::new();
        handlebars.render_template_source_to_write(&mut template_source, &data, &mut output_file).unwrap();
    }
}

fn download_input(year: &i32, day: &u32) -> Result<(), Box<dyn error::Error>> {
    let path_to_input = current_dir()
        .unwrap()
        .join("inputs")
        .join(format!("input{}.txt", day));
    if !path_to_input.exists() {
        let url = format!("https://adventofcode.com/{}/day/{}/input", &year, &day);
        let input = download(&url).unwrap();
        File::create(path_to_input).unwrap().write_all(input.as_bytes())?;
    }

    Ok(())
}

fn download_description(year: &i32, day: &u32) -> Result<(), Box<dyn error::Error>> {
    let path_to_question = current_dir()
        .unwrap()
        .join("questions")
        .join(format!("day{}.html", day));
    if !path_to_question.exists() {
        let url = format!("https://adventofcode.com/{}/day/{}", &year, &day);
        let input = download(&url).unwrap();
        File::create(path_to_question).unwrap().write_all(input.as_bytes())?;
    }

    Ok(())
}

fn download(url: &String) -> Result<String, Error> {
    let response = reqwest::blocking::Client::new()
        .get(url)
        .header("Cookie", get_session())
        .send()?
        .text()?;

    return Ok(response);
}

fn get_session() -> String {
    let path_to_config = current_dir()
        .unwrap()
        .join("bootstrapper")
        .join("config.toml");
    let mut buffer = String::new();
    File::open(&path_to_config).unwrap().read_to_string(&mut buffer).expect("Read to string");
    let config = buffer.parse::<Value>().expect("Parsing toml from buffer");
    let session = config["session"].as_str().unwrap();
    return format!("session={}", session);
}
