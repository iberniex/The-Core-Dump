use std::{env, error::Error, fs, println, process};

pub struct Config {
    pub query: String,
    pub filepath: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("The value provided does not evaluate to String"),
        };
        let filepath = match args.next() {
            Some(arg) => arg,
            None => return Err("The value provided is not a valid filepath"),
        };
        let case_sensitive = env::var("CASE_INSENSITIVE").is_ok();

        Ok(Config {
            query,
            filepath,
            case_sensitive,
        })
    }
}

pub fn worker() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem passing arguments: {}", err);
        process::exit(1)
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filepath)?;

    let results = if config.case_sensitive {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{}", line)
    }

    Ok(())
}

pub fn search_case_insensitive<'a>(
    query: &'a str,
    contents: &'a str,
) -> Box<dyn Iterator<Item = &'a str> + 'a> {
    // let query = query.to_lowercase();
    // let mut result = Vec::new();
    //
    // for line in contents.lines() {
    //     if line.to_lowercase().contains(&query) {
    //         result.push(line)
    //     }
    // }
    //
    // result
    Box::new(
        contents
            .lines()
            .filter(move |line| line.to_lowercase().contains(&query.to_lowercase())),
    )
}
pub fn search<'a>(query: &'a str, contents: &'a str) -> Box<dyn Iterator<Item = &'a str> + 'a> {
    Box::new(contents.lines().filter(move |line| line.contains(query)))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape";

        assert_eq!(
            Some("safe, fast, productive."),
            search(query, contents).next()
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        let mut result = search_case_insensitive(query, contents);

        assert_eq!(Some("Rust:"), result.next());
        assert_eq!(Some("Trust me."), result.next());
    }
}
