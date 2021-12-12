use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;
use std::str::Split;

pub fn get_input() -> std::io::Result<String> {
    let mut file = File::open("input/input")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn get_input_vec<T>() -> std::io::Result<Vec<T>>
where
    T: FromStr,
{
    get_input_vec_split("\n")
}

pub fn get_input_vec_split<T>(pattern: &str) -> std::io::Result<Vec<T>>
where
    T: FromStr,
{
    parse_split(get_input()?.trim().split(pattern))
}

pub fn parse_split<T>(input: Split<&str>) -> std::io::Result<Vec<T>>
where
    T: FromStr,
{
    Ok(input
        .map(|s| match s.parse::<T>() {
            Ok(inner) => inner,
            Err(_) => panic!("oh, no! {} isn't the type you thought it was!", s),
        })
        .collect())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
