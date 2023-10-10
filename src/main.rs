#[macro_use] extern crate log;
extern crate simplelog;

use clap::{arg, command, Parser};

use metrics::gauge;
use metrics_exporter_prometheus::PrometheusBuilder;

use reqwest::blocking::Client;

use serde::{Serialize, Deserialize};

use simplelog::{CombinedLogger, LevelFilter, SimpleLogger};

use std::fs;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::thread;
use std::time::Duration;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    configuration_file: String,
    #[arg(short, long,  default_value_t = 60)]
    interval: u64,
    #[arg(short, long, default_value_t = 9130)]
    port: u16,
}

#[derive(Debug, Serialize, Deserialize)]
struct TwilioExporterConfiguration {
    accounts: Vec<Account>,
    exporter: Exporter,
}

#[derive(Debug, Serialize, Deserialize)]
struct Account {
    name: String,
    sid: String,
    api_key: String,
    api_secret: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Exporter {
    interval: i64,
    port: i64,
}

#[derive(Serialize, Deserialize, Debug)]
struct TwilioBalanceResponse {
    currency: String,
    balance: String,
    account_sid: String,
}

fn request_balance(sid: String, api_key: String, api_secret: String) -> f64 {
    let mut balance = -1.0;
    let uri = format!("https://api.twilio.com/2010-04-01/Accounts/{sid}/Balance.json");
    let client = Client::new();
    let response = match client.get(uri)
        .basic_auth(api_key, Some(api_secret))
        .send() {
            Ok(r) => r,
            Err(err) => {
                panic!("Request failed: {}", err.to_string());
            },
        };

    // let body = match response.text() {
    //     Ok(r) => r,
    //     Err(err) => {
    //         panic!("Error retrieving response body: {}", err.to_string());
    //     }
    // };
    // println!("{}", body);

    match response.json::<TwilioBalanceResponse>() {
        Ok(tbr) => {
            info!("{} {}", sid, tbr.balance);
            match tbr.balance.parse::<f64>() {
                Ok(bal) => {
                    balance = bal;
                }
                Err(_e) => {
                    error!("Error parsing f64 balance from {}", tbr.balance);
                }
            }
        }
        Err(e) => {
            error!("Response parsing failed: {}", e.to_string());
        }
    };

    // match response.status(){
    //     reqwest::StatusCode::OK => {
    //         println!("{}", response.text());
    //     }
    //     other => {
    //         panic!("request failed");
    //     }
    // }

    return balance;
}


fn main() {
    CombinedLogger::init(
        vec![
            SimpleLogger::new(LevelFilter::Info, simplelog::Config::default()),
            // TermLogger::new(LevelFilter::Debug, simplelog::Config::default(),
            //                 simplelog::TerminalMode::Stdout, simplelog::ColorChoice::Never),
        ]
    ).unwrap();

    let args = Args::parse();
    let config_yml = fs::read_to_string(args.configuration_file).expect("read configuration file");
    let tec: TwilioExporterConfiguration = serde_yaml::from_str::<TwilioExporterConfiguration>(&config_yml).unwrap();

    let prom_builder = PrometheusBuilder::new();
    let prom_socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), args.port);
    match prom_builder.with_http_listener(prom_socket).install() {
        Ok(_) => {}
        Err(e) => {
            panic!("Error installing prometheus listener {}", e.to_string());
        }
    };
    info!("Prometheus listener started on {}", args.port);
    info!("Update interval {}", args.interval);

    let update_interval = Duration::new(args.interval, 0);
    loop {
        info!("updating metrics");
        for acct in tec.accounts.iter(){
            let balance = request_balance(acct.sid.clone(),
                                          acct.api_key.clone(),
                                          acct.api_secret.clone());
            gauge!("twilio_balance", balance,
                   "sid" => acct.sid.clone(),
                   "name" => acct.name.clone());
        }
        thread::sleep(update_interval);
    }
}
