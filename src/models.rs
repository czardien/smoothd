use hyper::Uri;

pub struct ScrapeTarget {
    pub uri: Uri,
    pub scrape_interval: u64,
}
