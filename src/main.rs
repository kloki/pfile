use std::{env, time::Duration};

use indicatif::ProgressBar;
use reqwest::{blocking::Client, header::HeaderMap};
use serde::{Deserialize, Serialize};

const INSTRUCTIONS: &str = "* Your are a helpfull assistent helping a developer.
* You will be ask to generate new file or snippet in different programming languages.
* You ONLY respond with the actual content of the file.
* Don't wrap your response in markdown syntax.
* Add explaination using comment blocks
";

#[derive(Serialize, Deserialize, Debug)]
struct Message<'a> {
    role: &'a str,
    content: &'a str,
}

#[derive(Serialize, Deserialize, Debug)]
struct PromptRequest<'a> {
    model: &'a str,
    messages: Vec<Message<'a>>,
}

impl PromptRequest<'_> {
    fn new(prompt: &str) -> PromptRequest<'_> {
        PromptRequest {
            model: "gpt-3.5-turbo",
            messages: vec![
                Message {
                    role: "system",
                    content: INSTRUCTIONS,
                },
                Message {
                    role: "user",
                    content: prompt,
                },
            ],
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct RespMessage {
    content: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct Entry {
    message: RespMessage,
}

#[derive(Serialize, Deserialize, Debug)]
struct Payload {
    choices: Vec<Entry>,
}

fn main() {
    let key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set");
    let prompt: String = env::args().collect::<Vec<_>>()[1..].to_vec().join(" ");

    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    headers.insert(
        "Authorization",
        ["Bearer ", &key].concat().parse().expect("Invalid api key"),
    );
    let client = Client::builder().default_headers(headers).build().unwrap();

    let bar = ProgressBar::new_spinner();
    bar.enable_steady_tick(Duration::from_millis(100));

    let res = client
        .post("https://api.openai.com/v1/chat/completions")
        .json(&PromptRequest::new(&prompt))
        .send();
    let payload: Payload = res.unwrap().json().expect("Failed response from openai");

    bar.finish();
    println!("{}", payload.choices[0].message.content);
}
