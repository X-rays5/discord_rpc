use json::JsonValue;
use std::io::Write;

fn read_line(text: &str) -> String {
    print!("{}: ", text);
    std::io::stdout().flush()
        .expect("Failed to flush stdout");
    let mut line = String::new();
    std::io::stdin().read_line(&mut line)
        .expect("Failed to read line");
    let mut input = line.trim();
    if input.is_empty() {
        input = "‌‌ ‌‌";
    }

    input.parse().expect("Failed to parse &str to String")
}

pub fn setup() -> JsonValue {
    println!("config could not be found generating a new config");

    let client_id: u64 = read_line("Client id").parse()
        .expect("Client id is not a number");

    println!("\nPress enter on options you want to be disabled");
    let large_image_key = read_line("Large image key");
    let large_image_text = read_line("Large image text");
    let small_image_key = read_line("Small image key");
    let small_image_text = read_line("Small image text");
    let state_text = read_line("State text");
    let details_text = read_line("Details text");

    let config = json::object!{
        client_id: client_id,
        image: {
            large: {
                key: large_image_key,
                text: large_image_text,
            },
            small: {
                key: small_image_key,
                text: small_image_text,
            }
        },
        state: state_text,
        details: details_text
    };

    config
}