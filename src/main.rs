mod first_time;

use json::JsonValue;

fn main() {
    let config: JsonValue;
    if std::path::Path::new("config.json").exists() {
        let file = std::fs::read_to_string("config.json")
            .expect("Cannot read config");
        config = json::parse(file.as_str())
            .expect("Config is corrupted and should be deleted");
    } else {
        config = first_time::setup();
        let stringified = json::stringify_pretty(config.clone(), 4);
        std::fs::write("config.json", stringified).expect("Failed to write config.json");
    }

    if !config["client_id"].is_null() {
        let mut rpc = discord_rpc_client::Client::new(config["client_id"].as_u64()
            .expect("Failed to get client_id"));

        rpc.start();
        rpc.clear_activity().expect("Failed to clear rpc");

        let img_large_key = match config["image"]["large"]["key"].is_null() {
            true => "",
            false => config["image"]["large"]["key"].as_str().unwrap()
        };

        let img_large_text = match config["image"]["large"]["text"].is_null() {
            true => "",
            false => config["image"]["large"]["text"].as_str().unwrap()
        };

        let img_small_key = match config["image"]["small"]["key"].is_null() {
            true => "",
            false => config["image"]["small"]["key"].as_str().unwrap()
        };

        let img_small_text = match config["image"]["small"]["text"].is_null() {
            true => "",
            false => config["image"]["small"]["text"].as_str().unwrap()
        };

        let state = match config["state"].is_null() {
            true => "",
            false => config["state"].as_str().unwrap()
        };

        let details = match config["details"].is_null() {
            true => "",
            false => config["details"].as_str().unwrap()
        };

        rpc.set_activity(|act|
            act.assets(|asset|
                asset.large_image(img_large_key)
                    .large_text(img_large_text)
                    .small_image(img_small_key)
                    .small_text(img_small_text)
            )
                .state(state)
                .details(details)
        ).expect("Failed to set rpc");

        println!("rpc started");
        loop {
            // program doesn't need to do anything anymore so can enter a infinite sleep
            std::thread::sleep(std::time::Duration::from_secs(9999999));
        }
    } else {
        eprintln!("Client id is not valid delete config.json and restart");
    }
}
