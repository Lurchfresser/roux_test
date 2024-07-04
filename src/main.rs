use dotenv::dotenv;
use roux::{util::FeedOption, Reddit};

#[tokio::main]
async fn main() {
    dotenv().ok();

    //println!("REDDIT_CLIENT_ID: {}",std::env::var("REDDIT_CLIENT_ID").unwrap());
    //println!("REDDIT_CLIENT_SECRET: {}",std::env::var("REDDIT_CLIENT_SECRET").unwrap());
    //println!("REDDIT_USERNAME: {}",std::env::var("REDDIT_USERNAME").unwrap());
    //println!("REDDIT_PASSWORD: {}",std::env::var("REDDIT_PASSWORD").unwrap());

    let reddit = Reddit::new(
        "PostmanRuntime/7.39.0",
        std::env::var("REDDIT_CLIENT_ID").unwrap().as_str(),
        std::env::var("REDDIT_CLIENT_SECRET").unwrap().as_str(),
    )
    .username(std::env::var("REDDIT_USERNAME").unwrap().as_str())
    .password(std::env::var("REDDIT_PASSWORD").unwrap().as_str())
    .login()
    .await;

    let me = reddit.unwrap();

    use roux::Subreddit;

    let subreddit = Subreddit::new_oauth("CryptoCurrency", &me.client);
    // Now you are able to:

    // Get top posts with limit = 10.
    let top = subreddit.top(10, None).await;

    match &top {
        Ok(top) => {
            for post in &top.data.children {
                println!("Title: {}", post.data.title);
            }
        }
        Err(e) => eprintln!("Error fetching top posts: {:?}", e),
    }

    let mut last_id = top.unwrap().data.after.unwrap();

    loop {
        let top_after = subreddit
            .top(
                10,
                Some(FeedOption::new().after(&last_id.clone())),
            )
            .await;

        match &top_after {
            Ok(top_after_ok) => {
                for post in &top_after_ok.data.children {
                    println!("Title: {}", post.data.title);
                }
                if let Some(after) = &top_after_ok.data.after {
                    last_id = after.clone();
                } else {
                    break;
                }
            }
            Err(e) => {
                eprintln!("Error fetching top posts: {:?}", e);
                break;
            }
        }
    }
}
