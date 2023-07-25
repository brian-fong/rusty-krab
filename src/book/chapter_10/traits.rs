pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        return format!(
            "(Read more from {}...)", self.summarize_author()
        );
    }
}

#[derive(Debug)]
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        return format!("the author, {}", self.author);
    }
}

// impl Summary for NewsArticle {
//     fn summarize(&self) -> String {
//         return format!("\"{}\", by {} ({})",
//             self.headline,
//             self.author,
//             self.location
//         );
//     }
// }

#[derive(Debug)]
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        return format!("@{}", self.username);
    }
    
    // fn summarize(&self) -> String {
    //     return format!("{}: {}",
    //         self.username,
    //         self.content,
    //     );
    // }
}

pub fn notify(item: &impl Summary) {
    println!("Quick summary: {}", item.summarize());
}

pub fn main() {
    println!();
    println!("+------------+");
    println!("|   Traits   |");
    println!("+------------+");
    let tweet = Tweet {
        username: String::from("0xfrian"),
        content: String::from("Learning about traits in Rust!"),
        reply: false,
        retweet: false,
    };
    println!("tweet: ");
    println!("{:#?}", tweet);
    println!();

    println!("tweet summary: ");
    println!("{}", tweet.summarize());
    println!();

    println!("tweet notif");
    notify(&tweet);
    println!();

    let article = NewsArticle {
        headline: String::from("Generate mechs on Midjourney"),
        location: String::from("Los Angeles, CA, USA"),
        author: String::from("0xfrian"),
        content: String::from(
            "I'm addicted to generating portraits of mecha robot suits
            using Midjourney",
        ),
    };
    println!("article: ");
    println!("{:#?}", article);
    println!();

    println!("article summary: ");
    println!("{}", article.summarize());
    println!();

    println!("article notif: ");
    notify(&article);
    println!();

    println!();
}
