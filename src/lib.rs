use curl::easy::{Easy, HttpVersion};
use std::fs;
use std::path::PathBuf;

pub fn get_input(year: &str, day: &str) -> String {
    let mut input = String::new();

    // cache input
    let path = PathBuf::from(format!("./.cache/day{day}.input"));

    if path.exists() {
        input = fs::read_to_string(path).unwrap();
    } else {
        let mut easy = Easy::new();
        easy.cookie(&format!(
            "session={}",
            fs::read_to_string(".session")
                .unwrap_or_else(|_| panic!("You must put your session key in '.session'"))
        ))
        .unwrap();
        // Use http/1 not http/2, for some reason it is buggy otherwise
        easy.http_version(HttpVersion::V11).unwrap();

        easy.url(&format!("https://adventofcode.com/{year}/day/{day}/input"))
            .unwrap();

        {
            let mut transfer = easy.transfer();
            transfer
                .write_function(|data| {
                    input.extend(data.iter().map(|b| *b as char));
                    Ok(data.len())
                })
                .unwrap();
            transfer.perform().unwrap();
        }

        fs::write(&path, &input).unwrap();
    }

    input
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        assert_eq!(
            get_input("2023", "1"),
            fs::read_to_string("inputs/day01").unwrap()
        );
    }
}
