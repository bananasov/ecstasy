use crate::collector::EcstasyCollector;
use crate::error::EcstasyError;
use crate::item::EcstasyItem;
use log::{debug, info};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default)]
pub struct BleachbooruCollector;

impl BleachbooruCollector {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn boxed() -> Box<dyn EcstasyCollector> {
        Box::new(Self::new())
    }
}

impl EcstasyCollector for BleachbooruCollector {
    fn id(&self) -> &'static str {
        "bleachbooru"
    }

    fn name(&self) -> &'static str {
        "bleachbooru"
    }

    fn api_base(&self) -> &'static str {
        "https://bleachbooru.org/post.json?limit=100"
    }

    fn site_base(&self) -> &'static str {
        "https://bleachbooru.org/"
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
        let mut page = 1u64; // starts at 1
        let mut finished = false;
        while !finished {
            debug!("Grabbing page with Reqwest GET...");
            let joined_tags = tags.clone().join("+");
            let mut resp = reqwest::get(&self.api_by_page(joined_tags, page))?;
            debug!("Reading the page body as text...");
            let body = resp.text()?;
            debug!("Deserializing posts...");
            let posts: Vec<BleachbooruPost> = match serde_json::from_str(&body) {
                Ok(posts) => posts,
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
                    items.push(EcstasyItem::new(
                        post.file_url,
                        tags.clone(),
                        self.id().to_owned(),
                    ));
                }
                page += 1;
            }
        }
        Ok(items)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BleachbooruPost {
    pub file_url: String,
    pub tags: String,
    pub md5: String,
}
