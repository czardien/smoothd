mod models;
use models::ScrapeTarget;

use hyper::body::HttpBody as _;
use hyper::Client;
use tokio::io::{stdout, AsyncWriteExt as _};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let scrape_targets: Vec<ScrapeTarget> = vec![ScrapeTarget {
        uri: "http://localhost:9100/metrics".parse().unwrap(),
        scrape_interval: 5,
    }];

    let client = Client::new();

    for scrape_target in scrape_targets {
        let mut resp = client.get(scrape_target.uri).await?;

        while let Some(chunk) = resp.body_mut().data().await {
            stdout().write_all(&chunk?).await?;
        }
    }
    Ok(())
}
