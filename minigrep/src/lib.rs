use std::error::Error;
use std::fs;

pub struct Config {
  pub query: String,
  pub filename: String,
}

impl Config {
  pub fn new(args: &[String]) -> Result<Config, &'static str> {
      if args.len() < 3 {
          return Err("no enough arguments")
      }
      let query = args[1].clone();
      let filename = args[2].clone();
      Ok(Config {
          query,
          filename,
      })
  }
}

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
  let content = fs::read_to_string(&config.filename)?;
  print!("Search result: {:?}", search(&config.query, content.as_str()));
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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_search() {
    assert_eq!(search("test", "This is test text\n, Just test,\n End line.").len(), 2);
  }
}