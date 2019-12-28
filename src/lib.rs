use std::fs;
use std::error::Error;
use std::env;

pub struct Config<'a>
{
	pub query: &'a String,
	pub filename: &'a String,
	pub case_sesnitive: bool,
}

impl<'a> Config<'a>
{
	pub fn new(args: &[String]) -> Result<Config, &'static str>
	{
		if args.len() < 3
		{
			return Err("Not enough arguments passed.")
		}
		let query = &args[1];
		let filename = &args[2];

		let case_sesnitive = env::var("CASE_INSENSITIVE").is_err();

		Ok(Config { query, filename, case_sesnitive })
	}
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>>
{
	let contents = fs::read_to_string(config.filename)?;

	let results = if config.case_sesnitive
	{
		search(&config.query, &contents)
	}
	else {
		search_case_insensitive(&config.query, &contents)
	};

	for line in results
	{
		println!("{}", line);
	}

	Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str>
{
	let mut results = Vec::new();

	for line in contents.lines()
	{
		if line.contains(query)
		{
			results.push(line);
		}
	}

	results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str>
{
	let query = query.to_lowercase();
	let mut results = Vec::new();

	for line in contents.lines()
	{
		if line.to_lowercase().contains(&query)
		{
			results.push(line);
		}
	}

	results
}

#[cfg(test)]
mod tests
{
	use super::*;

	#[test]
	fn one_result()
	{
		let query = "duct";
		let contents = "\
Rust:
safe, fast, productive.
Pick three.";

		assert_eq!(
			vec!["safe, fast, productive."],
			search(query, contents)
		);
	}
}