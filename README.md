# Twilio Prometheus Exporter in Rust

This is a Prometheus exporter that exposes Twilio balance
data for any number of Twilio accounts.

A configuration file is required to configure Twilio account
information.  A [sample file](./twilio-exporter.yml) is available.

This is one of a series of Twilio Prometheus Exporters built
to explore the differences in development experiences across
several languages:

- [Twilio Prometheus Exporter in Python](https://github.com/timfreund/twilio-exporter-python)
- [Twilio Prometheus Exporter in Rust](https://github.com/timfreund/twilio-exporter-rust)


## Development

To work on this code do the following:

Every time:

```
cargo run -- -c twilio-exporter.yml --help
```
