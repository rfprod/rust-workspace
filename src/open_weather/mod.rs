//! Open weather module.

use colored::Colorize;
use http_body_util::{BodyExt, Empty};
use hyper::{body::Bytes, Request, Uri};
use hyper_util::rt::TokioIo;
use std::{
    env::args,
    io::{self, Write},
};
use tokio::net::TcpStream;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

/// The entry point of the program.
pub fn main() {
    OpenWeather::new();
}

/// Input arguments of the program.
struct InuputArguments {
    city: Option<String>,
    api_key: Option<String>,
}

struct OpenWeather;

impl OpenWeather {
    /// Program constructor.
    fn new() -> OpenWeather {
        let mut program = OpenWeather;
        program.init();
        program
    }

    /// Initializes the program.
    fn init(&mut self) {
        println!("\n{}", "Open weather initialized.".blue().bold());

        let args = self.args();

        self.weather(args.city, args.api_key);
    }

    /// Parses arguments passed to the program.
    fn args(&mut self) -> InuputArguments {
        let arguments: Vec<String> = args().collect();

        println!("\n{}:\n{:?}", "Arguments".cyan().bold(), arguments);

        InuputArguments {
            city: arguments.get(2).cloned(),
            api_key: arguments.get(3).cloned(),
        }
    }

    /// Processes the input arguments and send a request to get weather data.
    fn weather(&mut self, city_arg: Option<String>, api_key_arg: Option<String>) {
        println!(
            "\n{}",
            "Current weather by city name using OpenWeather API.".cyan()
        );

        let city_is_some = city_arg.is_some();
        let mut city_arg_input = if city_is_some {
            match city_arg.unwrap().trim().parse::<String>() {
                Ok(value) => value,
                Err(_) => String::new(),
            }
        } else {
            String::new()
        };

        let api_key_is_some = api_key_arg.is_some();
        let mut api_key_arg_input = if api_key_is_some {
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
                println!("\n{}", "Please input a city:".yellow().bold());

                io::stdin()
                    .read_line(&mut city_input)
                    .expect("Failed to read line");
            } else if city_input.trim().is_empty() {
                city_input = city_arg_input.to_string();
            }

            let mut city = "";

            if api_key_arg_input.trim().is_empty() && api_key_input.trim().is_empty() {
                println!("\n{}", "Please input an API key (to get one for free, sign up here -> https:///openweathermap.org/home/sign_up):".yellow());

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
                    let result = self.weather_request(city, api_key).await;
                    match result {
                        Ok(data) => data,
                        Err(error) => println!("\n{}: {:?}", "There was an error".red(), error),
                    };
                });
                break;
            }
        }
    }

    /// Weather data request logic.
    async fn weather_request(&mut self, city: &str, api_key: &str) -> Result<()> {
        let mut uri_with_params = String::from("http://api.openweathermap.org/data/2.5/weather");
        uri_with_params.push_str("?q=");
        uri_with_params.push_str(city);
        uri_with_params.push_str("&appid=");
        uri_with_params.push_str(api_key);

        println!("\nUri, {}", uri_with_params);

        let uri = uri_with_params.as_str().parse::<Uri>()?;

        let host = uri.host().expect("uri has no host");
        let port = uri.port_u16().unwrap_or(80);
        let addr = format!("{}:{}", host, port);
        let stream = TcpStream::connect(addr).await?;
        let io = TokioIo::new(stream);

        let (mut sender, conn) = hyper::client::conn::http1::handshake(io).await?;
        tokio::task::spawn(async move {
            if let Err(err) = conn.await {
                println!("Connection failed: {:?}", err);
            }
        });

        let authority = uri.authority().unwrap().clone();

        let path = uri.path();
        let req = Request::builder()
            .uri(path)
            .header(hyper::header::HOST, authority.as_str())
            .body(Empty::<Bytes>::new())?;

        let mut res = sender.send_request(req).await?;

        println!("Response: {}", res.status());
        println!("Headers: {:#?}\n", res.headers());

        // Stream the body, writing each chunk to stdout as we get it
        // (instead of buffering and printing at the end).
        while let Some(next) = res.frame().await {
            let frame = next?;
            if let Some(chunk) = frame.data_ref() {
                io::stdout().write_all(chunk)?;
            }
        }

        println!("\n\n{}", "Done!".green().bold());

        Ok(())
    }
}
