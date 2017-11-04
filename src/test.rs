use super::*;
use std::time::Duration;
use std::thread;

fn init_reddit() -> App {
    use std::env;
    let get_env = |var| -> String {
        match env::var(var) {
            Ok(item) => item,
            _ => panic!("{} must be set", var),
        }
    };

    let username = get_env("REDDIT_USERNAME");
    let password = get_env("REDDIT_PASSWORD");
    let id = get_env("REDDIT_APP_ID");
    let secret = get_env("REDDIT_APP_SECRET");

    let mut reddit = App::new("OrcaLibTest", "v0.0.2", "/u/IntrepidPig/").unwrap();

    reddit.conn.auth = Some(
        reddit
            .authorize(username, password, net::auth::OauthApp::Script(id, secret))
            .unwrap(),
    );

    reddit
}

#[test(posts)]
fn get_posts() {
    init_reddit()
        .get_posts("unixporn".to_string(), Sort::Top(SortTime::All))
        .unwrap();
}

#[test(sort)]
fn post_sort() {
    assert_eq!(
        Sort::Top(SortTime::All).param(),
        &[("sort", "top"), ("t", "all")]
    )
}

#[test(auth)]
fn test_auth() {
    init_reddit().get_self().unwrap();
}

#[test(selfuser)]
fn self_info() {
    let reddit = init_reddit();

    let user = reddit.get_self().unwrap();
    println!("Me:\n{}", json::to_string_pretty(&user).unwrap());
}

#[test(otheruser)]
fn other_info() {
    let reddit = init_reddit();

    let otherguy = reddit.get_user("DO_U_EVN_SPAGHETTI").unwrap();
    println!(
        "That one guy:\n{}",
        json::to_string_pretty(&otherguy).unwrap()
    );
}

#[test(stream)]
fn comment_stream() {
    let reddit = init_reddit();
    let comments = reddit.get_comments("all".to_string());

    let mut count = 0;

    for comment in comments {
        count += 1;
        match comment {
            Comment::Loaded(data) => {
                println!("Got comment #{} by {}", count, data.author);
            }
            _ => panic!("This was not supposed to happen"),
        }

        if count > 30 {
            break;
        };
    }
}

#[test(tree)]
fn comment_tree() {
    let reddit = init_reddit();
    let tree = reddit.get_comment_tree("2np694".to_string());

    fn print_tree(listing: Listing<Comment>, level: i32) {
        for comment in listing {
            match comment {
                Comment::Loaded(data) => {
                    for _ in 0..level {
                        print!("\t");
                    }
                    println!("Comment by {}", data.author);
                    print_tree(data.replies, level + 1);
                }
                _ => {}
            }
        }
    };

    print_tree(tree, 0);
}

//#[test(submit)]
/*
fn test_post() {
	println!("{}", init_reddit().submit_self("pigasusland".to_string(), "Test Post".to_string(), "The time is dank-o-clock".to_string(), true).unwrap());
}*/
