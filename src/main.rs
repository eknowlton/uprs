#[macro_use]
extern crate serde;
extern crate config;
extern crate futures;
extern crate hyper;
extern crate tokio;

mod https_client;
mod settings;

use settings::Settings;
use std::path::PathBuf;

use futures::{Async, Future, Poll};

use hyper::{rt, Client};

use https_client::HttpsClient;

struct Uprs {
    settings: Settings,
}

impl Uprs {
    fn new(settings: Settings) -> Self {
        Uprs { settings }
    }
}

impl Future for Uprs {
    type Item = ();
    type Error = ();

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        let sites = self.settings.sites.take();

        for (_, site) in sites.unwrap() {
            tokio::spawn({
                let uri: hyper::Uri = site.uri.parse().unwrap();

                let https_client = HttpsClient::new();
                https_client
                    .client
                    .get(uri)
                    .map(|res| {
                        println!("Response: {}", res.status());
                    })
                    .map_err(|err| {
                        eprintln!("Error: {}", err);
                    })
            });
        }

        Ok(Async::Ready(()))
    }
}

fn main() {
    let mut config_dir = PathBuf::new();
    config_dir.push("config");

    let settings = Settings::new(config_dir).unwrap();

    let client = Uprs::new(settings);

    rt::run(client);
}
