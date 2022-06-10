// https://e926.net/posts.json

use crate::collector::EcstasyCollector;
use crate::error::EcstasyError;
use crate::item::EcstasyItem;
use log::{debug, info};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default)]
pub struct E926Collector;

impl E926Collector {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn boxed() -> Box<dyn EcstasyCollector> {
        Box::new(Self::new())
    }
}

impl EcstasyCollector for E926Collector {
    fn id(&self) -> &'static str {
        "e926"
    }

    fn name(&self) -> &'static str {
        "e926"
    }

    fn api_base(&self) -> &'static str {
        "https://e926.net/posts.json"
    }

    fn site_base(&self) -> &'static str {
        "https://e926.net"
    }

    fn tags_argument(&self) -> &'static str {
        "tags"
    }

    fn page_argument(&self) -> &'static str {
        "page"
    }

    fn collect(&self, tags: Vec<String>) -> Result<Vec<EcstasyItem>, EcstasyError> {
        info!("Starting {} collector...", &self.name());
        let mut items = Vec::new();
        let mut page = 0u64;
        let mut finished = false;
        while !finished {
            debug!("Grabbing page with Reqwest GET...");
            let joined_tags = tags.clone().join("+");
            let mut resp = reqwest::get(&self.api_by_page(joined_tags, page))?;
            debug!("Reading the page body as text...");
            let body = resp.text()?;
            debug!("Deserializing posts...");
            let posts: Vec<E926Post> = match serde_json::from_str::<E926Response>(&body) {
                Ok(resp) => resp.posts,
                Err(why) => {
                    debug!(
                        "Failed getting page {} of {}, gracefully ending collection: {}",
                        page,
                        self.name(),
                        why
                    );
                    Vec::new()
                }
            };
            info!(
                "Found {} {} on page {} of {}...",
                posts.len(),
                if posts.len() == 1 { "post" } else { "posts" },
                page,
                self.name()
            );
            if posts.is_empty() {
                finished = true;
                info!("Page {} is empty, stopping collection.", &page);
            } else {
                for post in posts {
                    let url = post.file.url.unwrap_or_else(|| "".to_owned());
                    if !url.is_empty() {
                        items.push(EcstasyItem::new(url, tags.clone(), self.id().to_owned()));
                    }
                }
                page += 1;
            }
        }
        Ok(items)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct E926Response {
    pub posts: Vec<E926Post>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct E926Post {
    pub file: E926PostFile,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct E926PostFile {
    pub md5: String,
    pub url: Option<String>,
}
