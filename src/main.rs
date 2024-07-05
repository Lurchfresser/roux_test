use dotenv::dotenv;
use roux::{
    comment::CommentData, response::BasicThing, submission::SubmissionData, util::FeedOption,
    MaybeReplies, Reddit, Subreddit,
};

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
    let top = subreddit.latest(10, None).await;

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
        println!("-------------------");

        let top_after = subreddit
            .latest(10, Some(FeedOption::new().after(&last_id.clone())))
            .await;

        match &top_after {
            Ok(top_after_ok) => {
                for post in &top_after_ok.data.children {
                    println!("Title: {}", post.data.title);
                    print_comments(post, &subreddit).await;
                }
                if let Some(after) = &top_after_ok.data.after {
                    println!("last_id: {}", last_id);
                    println!("len: {}", top_after_ok.data.children.len());
                    last_id.clone_from(after);
                } else {
                    println!("No more posts, last id: {}", last_id);
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

async fn print_comments(post: &BasicThing<SubmissionData>, subreddit: &Subreddit) {
    let comment_tree = subreddit
        .article_comments(&post.data.id, Some(10), Some(10))
        .await;

    match &comment_tree {
        Ok(comment_tree) => {
            for comment in &comment_tree.data.children {
                recursive_print(&comment.data, 1);
            }
        }
        Err(e) => eprintln!("Error fetching comments: {:?}", e),
    }
}

fn recursive_print(comment: &CommentData, depth: u32) {
    if let Some(body) = &comment.body {
        println!("{}{}{}", depth, " ".repeat(depth as usize), body);
    }
    if let Some(MaybeReplies::Reply(replies)) = &comment.replies {
        for reply in &replies.data.children {
            recursive_print(&reply.data, depth + 1);
        }
    }
}
