use std::io;
use std::sync::Arc;

use hyper::client::connect::{Connect, Connected, Destination};
use hyper::client::HttpConnector;
use hyper::Client;

use tokio::net::TcpStream;
use tokio_tls::{TlsConnector, TlsStream};

use futures::future::{err, Future};

pub struct HttpsClient {
    pub client: hyper::Client<HttpsConnector>,
}

impl HttpsClient {
    pub fn new() -> Self {
        let tls_cx = native_tls::TlsConnector::builder().build().unwrap();
        let mut connector = HttpsConnector::new(tls_cx.into(), HttpConnector::new(2));
        connector.http.enforce_http(false);

        HttpsClient {
            client: Client::builder().build(connector),
        }
    }
}

pub struct HttpsConnector {
    tls: Arc<TlsConnector>,
    http: HttpConnector,
}

impl HttpsConnector {
    fn new(tls: TlsConnector, http: HttpConnector) -> Self {
        Self {
            tls: Arc::new(tls),
            http,
        }
    }
}

impl Connect for HttpsConnector {
    type Transport = TlsStream<TcpStream>;
    type Error = io::Error;
    type Future = Box<Future<Item = (Self::Transport, Connected), Error = Self::Error> + Send>;

    fn connect(&self, dst: Destination) -> Self::Future {
        if dst.scheme() != "https" {
            return Box::new(err(io::Error::new(
                io::ErrorKind::Other,
                "only works with https",
            )));
        }

        let host = format!(
            "{}{}",
            dst.host(),
            dst.port().map(|p| format!(":{}", p)).unwrap_or("".into())
        );

        let tls_cx = self.tls.clone();

        Box::new(self.http.connect(dst).and_then(move |(tcp, connected)| {
            tls_cx
                .connect(&host, tcp)
                .map(|s| (s, connected))
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
        }))
    }
}
#[cfg(test)]
mod tests {

    use super::HttpsClient;
    use futures::future::Future;
    use hyper::rt;

    #[test]
    fn test_https_connector_fails_for_non_https() {
        let uri: hyper::Uri = "http://google.com".parse().unwrap();

        rt::run(rt::lazy(|| {
            HttpsClient::new()
                .client
                .get(uri)
                .map(|_| {
                    panic!(); // should receive a error
                })
                .map_err(|result| match result.into_cause() {
                    Some(_) => assert!(true),
                    _ => panic!(),
                })
        }));
    }
}
