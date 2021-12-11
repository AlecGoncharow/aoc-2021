use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;

pub fn get_input() -> std::io::Result<String> {
    let mut file = File::open("input/input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn get_input_vec<T>() -> std::io::Result<Vec<T>>
where
    T: FromStr,
{
    Ok(get_input()?
        .trim()
        .split("\n")
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
