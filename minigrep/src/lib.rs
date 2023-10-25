use std::error::Error;
use std::fs;
use std::env;

pub struct Config {
  pub query: String,
  pub filename: String,
  pub case_sensitive: bool,
}

impl Config {
  pub fn new(args: &[String]) -> Result<Config, &'static str> {
      if args.len() < 3 {
          return Err("no enough arguments")
      }
      let query = args[1].clone();
      let filename = args[2].clone();
      let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
      Ok(Config {
          query,
          filename,
          case_sensitive,
      })
  }
}

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
  let content = fs::read_to_string(&config.filename)?;
  let result = if config.case_sensitive {
    search(&config.query, content.as_str())
  } else {
    search_case_insensitive(&config.query, content.as_str())
  };
  print!("Search result: {:?}", result);
  Ok(())
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
  let mut result =  Vec::new();
  for line in content.lines() {
    if line.contains(query) {
      result.push(line);
    }
  }
  result
}

pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
  let mut result =  Vec::new();
  for line in content.lines() {
    if line.to_lowercase().contains(&query.to_lowercase()) {
      result.push(line);
    }
  }
  result
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_search() {
    assert_eq!(search("test", "This is test text\n, Just test,\n End line.").len(), 2);
  }
}