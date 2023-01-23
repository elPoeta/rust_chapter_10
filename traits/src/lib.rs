#![allow(unused)]

use std::fmt::Display;
pub trait Summary {
    fn summarize(&self) -> String;
}

/* Default Implementations
  pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}
*/

/*
  pub trait Summary {
       fn summarize_author(&self) -> String;

      fn summarize(&self) -> String {
          format!("(Read more from {}...)", self.summarize_author())
      }
}
*/
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
    /*
       fn summarize_author(&self) -> String {
        format!("@{}", self.username)
      }
    */
}

// Traits as Parameters
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

/*
  Trait Bound Syntax => same and more verbose that Traits as Parameters
  pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
*/

//Specifying Multiple Trait Bounds with the + Syntax
pub fn multi_notify(item: &(impl Summary + Display)) {}

//The + syntax is also valid with trait bounds on generic types:

//pub fn multi_notify<T: Summary + Display>(item: &T) {}

//Returning Types that Implement Traits
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
