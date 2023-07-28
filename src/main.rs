#![allow(unused)]

use clap::{Arg, ArgAction, Command, Parser};
use reqwest::header;
use serde::Deserialize;
use serde_json;
use std::collections::HashMap;
use std::str::FromStr;
/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

#[derive(Debug)]
enum JokeCategory {
    Programming,
    Christmas,
    Spooky,
    Misc,
    Dark,
    Pun,
}

impl FromStr for JokeCategory {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "programming" => Ok(JokeCategory::Programming),
            "christmas" => Ok(JokeCategory::Christmas),
            "spooky" => Ok(JokeCategory::Spooky),
            "misc" => Ok(JokeCategory::Misc),
            "dark" => Ok(JokeCategory::Dark),
            "pun" => Ok(JokeCategory::Pun),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
enum Language {
    English,
    German,
    French,
    Spanish,
}

impl FromStr for Language {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "en" => Ok(Language::English),
            "de" => Ok(Language::German),
            "fr" => Ok(Language::French),
            "es" => Ok(Language::Spanish),
            _ => Err(()),
        }
    }
}

impl Language {
    fn to_lowercase(&self) -> String {
        match self {
            Language::English => "en",
            Language::German => "de",
            Language::French => "fr",
            Language::Spanish => "es",
        }
        .to_lowercase()
    }

    fn to_lowercase_without_quotes(&self) -> String {
        let variant_name = match self {
            Language::English => "en",
            Language::German => "de",
            Language::French => "fr",
            Language::Spanish => "es",
        };
        let lowercase_variant = variant_name.to_lowercase();
        lowercase_variant.replace("\"", "")
    }
}

fn main() {
    let matches = Command::new("MyApp")
        .arg(
            Arg::new("category")
                .short('c')
                .long("category")
                .long_help("Category can only contain either ** programming | misc | dark | pun | spooky | christmas **")
                // .required(true)
                .value_name("Joke category")
                .default_value("Programming")
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("lang")
                .short('l')
                .long_help("Language can only contain either ** en | de | fr | es **")
                // .required(true)
                .default_value("en")
                .action(ArgAction::Set),
        )
        .get_matches(); // builds the instance of ArgMatches

    let category_str = matches.get_one::<String>("category").unwrap();
    let category = match category_str.parse::<JokeCategory>() {
        Ok(category) => category,
        Err(_) => {
            println!("Invalid Category selected. Using default category (Programming).");
            /// Provide a default value in case of error
            JokeCategory::Programming
        }
    };
    let lang_str = matches.get_one::<String>("lang").unwrap();
    let lang = match lang_str.parse::<Language>() {
        Ok(lang) => lang,
        Err(_) => {
            println!("Invalid language selected. Using default language (English).");
            /// Provide a default value in case of error
            Language::English
        }
    };
    println!("Lang: {:?}, Category: {:?}", lang, category);
    let joke = fetchjoke(category, lang);

    #[derive(Debug, Deserialize)]
    struct JokeResponse {
        error: bool,
        category: String,
        #[serde(rename = "type")]
        joke_type: String,
        setup: String,
        delivery: String,
        flags: Flags,
        id: u32,
        safe: bool,
        lang: String,
    }

    #[derive(Debug, Deserialize)]
    struct Flags {
        nsfw: bool,
        religious: bool,
        political: bool,
        racist: bool,
        sexist: bool,
        explicit: bool,
    }

    #[tokio::main] // Enable the async runtime for the async/await syntax
    async fn fetchjoke(
        category: JokeCategory,
        lang: Language,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let base_url = "https://v2.jokeapi.dev/joke/";

        let mut url = reqwest::Url::parse(base_url)?;
        // let lang = "de";

        // Construct the dynamic part of the URL
        let dynamic_part = format!("{:?}?lang={}", category, lang.to_lowercase());

        // Combine the base URL with the dynamic part
        let url = format!("{}{}", base_url, dynamic_part);

        // println!("Response: {}", url);
        // return Ok(());
        // Create a client with default settings
        let client = reqwest::Client::new();

        let response_text = client.get(url).send().await?.text().await?;
        // println!("Response: {}", response_text);

        let response = serde_json::from_str::<JokeResponse>(&response_text)?;

        // Handle the API response (e.g., print the response)
        println!("Setup: {}", response.setup);
        println!("Delivery: {}", response.delivery);

        Ok(())
    }
}
