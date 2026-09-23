pub fn call_traits_abstract() {
    println!("\n\n---------Traits with abstract methods-----------\n\n");
    let news_article = NewsArticle { headline: "Hello World".to_string(), location: "VA".to_string(), author: "Main".to_string(), content: "Hello Word Again!".to_string() };
    print!("{}", news_article.summarize());
}


trait Summary {
    fn summarize(&self) -> String;
}

struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,   
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        return format!("{}, by {} ({})", self.headline, self.author, self.location);
    }
}

struct SocialPost {
    pub username: String,
    pub content: String,
    pub repost: bool,
    pub reply: bool,
}

impl Summary for SocialPost {
    fn summarize(&self) -> String {
        return format!("{}: {}", self.username, self.content);
    }
}