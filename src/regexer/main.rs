use std::fmt::format;
use std::fs::read;
use std::path::Path;

use clap::{App, Arg, arg, Parser};
use regex::Regex;

const CRATE_VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");
const CRATE_NAME: Option<&str> = option_env!("CARGO_CRATE_NAME");

#[derive(Debug)]
struct Record {
    line: usize,
    tx: String,
}

fn process_file(p: Box<Path>, re: Regex) -> Result<Vec<Record>, String> {
    let mut res = vec![];

    let bts = read(p).map_err(|_| "could not read string".to_string())?;

    if let Ok(ss) = String::from_utf8(bts) {
        for (i, l) in ss.lines().enumerate() {
            if re.is_match(l) {
                res.push(Record { line: i + 1, tx: l.to_string() });
            }
        }
    }

    Ok(res)
}

fn main() -> Result<(), String> {
    let cp = App::new(CRATE_NAME.unwrap_or("unknown").to_string())
        .author("ScharxiDev")
        .version(&*format!("v{}", CRATE_VERSION.unwrap_or("unknown")))
        .args(&[
            arg!(pattern: <PATTERN> "The Regex pattern to search for"),
            arg!(file: -f --file <FILE> "The file to test")]
        ).get_matches();

    let re = Regex::new(cp.value_of("pattern").unwrap()).map_err(|e| "bad regex")?;
    let path = Path::new(cp.value_of("file").ok_or("No file chosen")?);

    let p = process_file(
        Box::from(path),
        re,
    );

    println!("{:?}", p);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::{CRATE_VERSION};

    #[test]
    fn get_version_from_toml() {
        assert_eq!(
            format!("v{}", CRATE_VERSION.unwrap_or("unknown")),
            "v0.1.0"
        )
    }
}