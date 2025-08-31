#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

pub trait Summary {
    fn summarize(&self) -> String;
}

#[derive(Debug)]
struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!(
            "{}, content: {} by {} ({})",
            self.headline, self.content, self.author, self.location
        )
    }
}

#[derive(Debug)]
struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}


fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}


fn main() {
    let point = Point { x: 5, y: 10.4 };
    let point2 = Point { x: "Hello", y: 'c' };

    let point3 = point.mixup(point2);

    println!("{:?}, {:?}", point3.x, point3.y);

    let news = NewsArticle {
        headline: String::from("headline"),
        location: String::from("location"),
        author: String::from("author"),
        content: String::from("countent"),
    };

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course"),
        reply: false,
        retweet: false,
    };

    let result = longest("test", "f");
    println!("{:?}", result);

    println!("{:?}", news.summarize());

    println!(
        "1 new tweet: {} :reply? {} : retweet? {}",
        tweet.summarize(),
        tweet.reply,
        tweet.retweet
    );
}

