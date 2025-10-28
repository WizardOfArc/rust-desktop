use std::env;

use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use thiserror::Error;

const POSTS_FILE_VAR_NAME: &str = "BLOG_POSTS_FILE";

type BlogResult<T> = std::result::Result<T, BlogError>;

#[derive(Debug, Error)]
pub enum BlogError {
    #[error("Unable to Read Posts File")]
    UnableToReadPostsFile,
    #[error("Posts file has unparsable JSON")]
    PostsFileUnParsable,
    #[error("Could not write to or save Posts file")]
    CouldNotWritePostsFile,
    #[error("Could not find env var: {0}")]
    EnvVarNotFound(String),
}

#[derive(Debug)]
pub struct Post {
    pub title: String,
    pub content: Vec<String>,
}

fn time_to_soleilfou(time: DateTime<Local>) -> String {
    time.format("%Y:%m:%d:%H:%M:%S").to_string()
}

#[derive(Serialize, Deserialize, Debug)]
struct PostForJson {
    woa_time: String,
    title: String,
    content: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct FooterLink {
    label: String,
    url: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Page {
    title: String,
    css: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct BlogPostsForJson {
    page: Page,
    footer_links: Vec<FooterLink>,
    posts: Vec<PostForJson>,
}

impl BlogPostsForJson {
    fn from_json_string(json_string: &str) -> BlogResult<BlogPostsForJson> {
        serde_json::from_str(json_string).map_err(|_| BlogError::PostsFileUnParsable)
    }

    fn to_json_string(&self) -> BlogResult<String> {
        serde_json::to_string(self).map_err(|_| BlogError::CouldNotWritePostsFile)
    }

    fn save_to_file(&self, filename: &str) -> BlogResult<()> {
        let json_string = self.to_json_string()?;
        std::fs::write(filename, json_string).map_err(|_| BlogError::CouldNotWritePostsFile)
    }

    fn from_file(filename: &str) -> BlogResult<BlogPostsForJson> {
        let file_contents =
            std::fs::read_to_string(filename).map_err(|_| BlogError::UnableToReadPostsFile)?;
        BlogPostsForJson::from_json_string(&file_contents)
    }

    fn add_post(&mut self, post: Post) {
        self.posts.insert(0, post.for_json());
    }
}

impl Post {
    fn for_json(&self) -> PostForJson {
        let sf_time = time_to_soleilfou(Local::now());
        PostForJson {
            woa_time: sf_time,
            title: self.title.clone(),
            content: self.content.clone(),
        }
    }
}

pub fn publish(post: Post) -> BlogResult<bool> {
    let error_fmt = format!(
        "No blog posts file specified - please set the env var:'{}'",
        POSTS_FILE_VAR_NAME
    );
    let filename = env::var(POSTS_FILE_VAR_NAME)
        .map_err(|e| BlogError::EnvVarNotFound(format!("{}:{}", error_fmt, e.to_string())))?;
    let mut blog_posts =
        BlogPostsForJson::from_file(&filename).unwrap_or_else(|_| BlogPostsForJson {
            posts: vec![],
            footer_links: vec![],
            page: Page {
                title: "".to_string(),
                css: "".to_string(),
            },
        });
    blog_posts.add_post(post);
    blog_posts
        .save_to_file(&filename)
        .map_err(|_e| BlogError::CouldNotWritePostsFile)?;
    Ok(true)
}
