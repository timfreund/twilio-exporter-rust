use metrics::gauge;
use metrics_exporter_prometheus::PrometheusBuilder;

use rand::Rng;

use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::thread;
use std::time::Duration;

fn main() {
    println!("Hello, world!");

    let prom_builder = PrometheusBuilder::new();
    let prom_socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 9130);
    // prom_builder.with_http_listener("0.0.0.0:9130".parse());
    prom_builder.with_http_listener(prom_socket).install();
    //prom_builder.install();


    let update_interval = Duration::new(5, 0);
    let mut rng = rand::thread_rng();
    loop {
        println!("updating metrics");
        gauge!("twilio_balance", rng.gen::<f64>(), "sid" => "1234", "name" => "asdf");
        gauge!("twilio_balance", rng.gen::<f64>(), "sid" => "5678", "name" => "qwerty");

        thread::sleep(update_interval);
    }
}
