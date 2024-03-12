//! # My Crate
//!
//! `my_crate` is a collection of utilities to make performing certain
//! calculations more convenient.

//定义 trait
pub trait Summary {
  // 以下方法 实现 size 具体逻辑
  fn size(&self) -> String;
  
  // 默认字符串
  fn normal(&self) -> String {
    String::from("(Read more...)")
  }
  
  fn summary_author(&self) -> String;

  fn summary_fn(&self) -> String {
    format!("(Read more form {} ...)", self.summary_author())
  }
}

pub struct NewsArticle {
  pub headline: String,
  pub localhost: String,
  pub author: String,
  pub content:String,
}

impl Summary for NewsArticle {
    fn size(&self) -> String {
      format!("{}, by {} {}", self.headline, self.author, self.localhost)
    }
    fn summary_author(&self) -> String {
      format!("{}", self.author)
    }
}

pub struct Tweet {
  pub username: String,
  pub content: String,
  pub reply:bool,
  pub retweet:bool,
}

impl Summary for Tweet {
  fn size(&self) -> String {
    format!("{}:{}", self.username, self.content)
  }

  fn summary_author(&self) -> String {
    format!("{}", self.username)
  }
}

/// adds one to the number given
/// 
/// # Examples
/// 
/// ```
/// let arg = 5;
/// let answer = rust_study::add_one(arg);
/// 
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
  x + 1
}