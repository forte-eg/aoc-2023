use std::{env, fs};

pub fn read_input() -> Result<String, ()> {
    let mut pwd = env::current_dir()
        .map_err(|err| eprintln!("{err}"))?;

    pwd.push("input");

    let contents = fs::read_to_string(pwd.as_path())
        .map_err(|err| eprintln!("{err}"))?;

    return Ok(contents);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let contents = read_input().unwrap();
        assert!(contents.eq("hello world"));
    }
}
