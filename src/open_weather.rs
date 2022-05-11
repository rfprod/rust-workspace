use colored::Colorize;
use hyper::{body::Buf, Client, Uri};
use std::{
    env::{args, Args},
    io::{self, Write},
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

// The open weather program entry point.
pub fn main() {
    let mut args: Args = args();

    println!("\n{}:\n{:?}", "Arguments".cyan(), args);

    println!(
        "\n{}",
        "Current weather by city name using OpenWeather API.".cyan()
    );

    let city_arg = args.nth(2);

    let api_key_arg = args.nth(3);

    get_weather(city_arg, api_key_arg);
}

// Process arguments and send a request for weather data.
fn get_weather(city_arg: Option<String>, api_key_arg: Option<String>) {
    let mut city_arg_input = if let Some(..) = city_arg {
        match city_arg.unwrap().trim().parse::<String>() {
            Ok(value) => value,
            Err(_) => String::new(),
        }
    } else {
        String::new()
    };

    let mut api_key_arg_input = if let Some(..) = api_key_arg {
        match api_key_arg.unwrap().trim().parse::<String>() {
            Ok(value) => value,
            Err(_) => String::new(),
        }
    } else {
        String::new()
    };

    let mut city_input = String::new();

    let mut api_key_input = String::new();

    loop {
        if city_arg_input.trim().is_empty() && city_input.trim().is_empty() {
            println!("\n{}", "Please input a city:".yellow());

            io::stdin()
                .read_line(&mut city_input)
                .expect("Failed to read line");
        } else if city_input.trim().is_empty() {
            city_input = city_arg_input.to_string();
        }

        let mut city = "";

        if api_key_arg_input.trim().is_empty() && api_key_input.trim().is_empty() {
            println!("\n{}", "Please input an API key (to get one for free, sign up here -> https://openweathermap.org/home/sign_up):".yellow());

            io::stdin()
                .read_line(&mut api_key_input)
                .expect("Failed to read line");
        } else if api_key_input.trim().is_empty() {
            api_key_input = api_key_arg_input.to_string();
        }

        let mut api_key = "";

        if city_input.trim().is_empty() {
            city_arg_input = String::new();
        } else {
            city = city_input.as_str().trim();
        }

        if api_key_input.trim().is_empty() {
            api_key_arg_input = String::new();
        } else {
            api_key = api_key_input.as_str().trim();
        }

        if !city_input.trim().is_empty() && !api_key_input.trim().is_empty() {
            let runtime = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap();

            runtime.block_on(async {
                let result = send_weather_request(city, api_key).await;
                match result {
                    Ok(data) => data,
                    Err(error) => println!("\n{}: {:?}", "There was an error".red(), error),
                };
            });
            break;
        }
    }
}

// Weather data request logic.
async fn send_weather_request(city: &str, api_key: &str) -> Result<()> {
    let client = Client::new();

    let mut uri_with_params = String::from("http://api.openweathermap.org/data/2.5/weather");
    uri_with_params.push_str("?q=");
    uri_with_params.push_str(city);
    uri_with_params.push_str("&appid=");
    uri_with_params.push_str(api_key);

    println!("\nUri, {}", uri_with_params);

    let uri = uri_with_params.as_str().parse::<Uri>()?;

    let res = client.get(uri).await?;

    println!("{}: {}", "Response".green(), res.status());
    println!("{}: {:#?}\n", "Headers".green(), res.headers());

    let body = hyper::body::aggregate(res).await?;
    io::stdout().write_all(body.chunk())?;

    println!("\n\n{}", "Done!".green());

    Ok(())
}
