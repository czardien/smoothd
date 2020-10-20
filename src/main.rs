mod error;
use error::SmoothdError;

use hyper::{Client, Uri};

struct Target {
    endpoint: Uri,
    scrape_interval: u64,
}

#[tokio::main]
async fn main() {
    let client = Client::new();

    let url: Uri = "http://httpbin.org/response-headers?foo=bar"
        .parse()
        .unwrap();
    assert_eq!(url.query(), Some("foo=bar"));

    match client.get(url).await {
        Ok(res) => println!("Response: {}", res.status()),
        Err(err) => println!("Error: {}", err),
    }
}

use tokio::time;

type Result<T, E = SmoothdError> = std::result::Result<T, E>;

pub async fn scrape(target: Target) -> Option<SmoothdError> {
    let mut count = 0;
    loop {
        let res = reqwest::get(target.endpoint).await?;
        let body = res.text().await?;
        println!("Count:\n{}", count);
        // println!("Body:\n{}", body);

        count = count + 1;

        let mut interval = time::interval(time::Duration::from_secs(target.scrape_interval));
        interval.tick().await;
    }
}

#[tokio::main]
pub async fn main() -> Result<()> {
    // TODO: Gets configuration from environment file
    let targets: Vec<Target> = vec![Target {
        endpoint: "http://localhost:9100/metrics".parse()?,
        scrape_interval: 5,
    }];

    for target in &targets {
        let res = reqwest::get(target.endpoint).await?;
        let body = res.text().await?;
        println!("Body:\n{}", body);
    }
    Ok(())
}
