use crate::collector::EcstasyCollector;
use crate::item::EcstasyItem;
use serde::{Deserialize, Serialize};

use crate::error::EcstasyError;
use log::{debug, info};

#[derive(Clone, Debug, Default)]
pub struct RealbooruCollector;

impl RealbooruCollector {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn boxed() -> Box<dyn EcstasyCollector> {
        Box::new(Self::new())
    }
}

impl EcstasyCollector for RealbooruCollector {
    fn id(&self) -> &'static str {
        "realbooru"
    }

    fn name(&self) -> &'static str {
        "Realbooru"
    }

    fn api_base(&self) -> &'static str {
        "https://realbooru.com/index.php?page=dapi&s=post&q=index"
    }

    fn site_base(&self) -> &'static str {
        "https://realbooru.com"
    }

    fn tags_argument(&self) -> &'static str {
        "tags"
    }

    fn page_argument(&self) -> &'static str {
        "pid"
    }

    fn collect(&self, tags: Vec<String>, pagelimit: &u64) -> Result<Vec<EcstasyItem>, EcstasyError> {
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
            let posts: RealbooruPosts = match serde_xml_rs::from_str(&body) {
                Ok(posts) => posts,
                Err(why) => {
                    debug!(
                        "Failed getting page {} of {}, gracefully ending collection: {}",
                        page,
                        self.name(),
                        why
                    );
                    RealbooruPosts { posts: Vec::new() }
                }
            };
            info!(
                "Found {} {} on page {} of {}...",
                posts.posts.len(),
                if posts.posts.len() == 1 {
                    "post"
                } else {
                    "posts"
                },
                page,
                self.name()
            );
            if posts.posts.is_empty() {
                finished = true;
                info!("Page {} is empty, stopping collection.", &page);
            } else {
                for post in posts.posts {
                    items.push(EcstasyItem::new(
                        post.file_url,
                        tags.clone(),
                        self.id().to_owned(),
                    ));
                }

                if &page >= pagelimit {
                    finished = true;
                    info!("Pagelimit hit at {}, stopping collection.", &page);
                }

                page += 1;
            }
        }
        Ok(items)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RealbooruPosts {
    #[serde(rename = "post")]
    pub posts: Vec<RealbooruPost>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RealbooruPost {
    pub file_url: String,
    pub tags: String,
    pub md5: String,
}
