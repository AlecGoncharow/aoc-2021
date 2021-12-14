use std::{collections::HashMap, str::FromStr};

#[derive(Debug)]
struct LineEntry {
    patterns: Vec<SignalPattern>,
    output: Vec<String>,
}

impl FromStr for LineEntry {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split("|");

        let patterns = split
            .next()
            .unwrap()
            .trim()
            .split_whitespace()
            .map(SignalPattern::new)
            .collect();
        let output = split
            .next()
            .unwrap()
            .trim()
            .split_whitespace()
            .map(|x| x.to_string())
            .collect();

        Ok(Self { patterns, output })
    }
}

#[derive(Debug, Clone)]
pub struct SignalPattern {
    pub code: String,
    pub bitmask: u8,
}

impl SignalPattern {
    pub fn new(code: &str) -> Self {
        let mut bitmask = 0;
        for ch in code.chars() {
            bitmask |= Self::parse_char(ch);
        }

        Self {
            code: code.to_string(),
            bitmask,
        }
    }

    fn parse_char(ch: char) -> u8 {
        1 << (ch as u32 - 97) as u8
    }

    fn parse_bit_to_char(byte: u8) -> Option<char> {
        Some(match byte {
            0b0000_0001 => 'a',
            0b0000_0010 => 'b',
            0b0000_0100 => 'c',
            0b0000_1000 => 'd',

            0b0001_0000 => 'e',
            0b0010_0000 => 'f',
            0b0100_0000 => 'g',

            _ => return None,
        })
    }

    fn parse_missing_bit_to_char(byte: u8) -> Option<char> {
        let b = !(byte | (1 << 7));
        Some(match b {
            0b0000_0001 => 'a',
            0b0000_0010 => 'b',
            0b0000_0100 => 'c',
            0b0000_1000 => 'd',

            0b0001_0000 => 'e',
            0b0010_0000 => 'f',
            0b0100_0000 => 'g',

            _ => return None,
        })
    }

    fn parse_mask_to_value(byte: u8) -> Option<usize> {
        Some(match byte {
            0b0111_0111 => 0,
            0b0010_0100 => 1,
            0b0101_1101 => 2,
            0b0110_1101 => 3,
            0b0010_1110 => 4,
            0b0110_1011 => 5,
            0b0111_1011 => 6,
            0b0010_0101 => 7,
            0b0111_1111 => 8,
            0b0110_1111 => 9,
            _ => return None,
        })
    }
}

fn main() -> std::io::Result<()> {
    let input: Vec<LineEntry> = common::get_input_vec()?;

    println!("{:?}", input[0]);
    let mut count = 0;
    for line in &input {
        for pattern in &line.output {
            match pattern.len() {
                2 | 3 | 4 | 7 => count += 1,

                _ => {}
            }
        }
    }
    println!("[Part 1] count {:?}", count);

    let mut sum = 0;
    for line in &input {
        let mut pattern_map: HashMap<u8, SignalPattern> = HashMap::new();
        let mut five_set = Vec::new();
        let mut six_set = Vec::new();
        let mut char_map: HashMap<char, char> = HashMap::new();
        for pattern in &line.patterns {
            let _ = match pattern.code.len() {
                2 => pattern_map.insert(1, pattern.clone()),
                3 => pattern_map.insert(7, pattern.clone()),
                4 => pattern_map.insert(4, pattern.clone()),
                5 => {
                    five_set.push(pattern.clone());
                    None
                }
                6 => {
                    six_set.push(pattern.clone());
                    None
                }
                7 => pattern_map.insert(8, pattern.clone()),

                _ => None,
            };
        }

        // XOR 1 and 7 to determine which character is really a
        let one = pattern_map.get(&1).unwrap().clone();
        let four = pattern_map.get(&4).unwrap().clone();
        let seven = pattern_map.get(&7).unwrap();
        let _eight = pattern_map.get(&8).unwrap();
        if let Some(ch) = SignalPattern::parse_bit_to_char(one.bitmask ^ seven.bitmask) {
            char_map.insert(ch, 'a');
            println!("{} -> {}", ch, 'a');
        } else {
            panic!("not good");
        }

        let abd_mask = four.bitmask ^ seven.bitmask;
        let abcdf_mask = four.bitmask | seven.bitmask;
        let mut cdf_mask = 0;
        for pattern in six_set {
            if !((abd_mask & pattern.bitmask) == abd_mask) {
                let ch = SignalPattern::parse_missing_bit_to_char(pattern.bitmask).unwrap();
                char_map.insert(ch, 'd');
                cdf_mask |= SignalPattern::parse_char(ch);
                pattern_map.insert(0, pattern.clone());
                println!("found zero and {} -> {}", ch, 'd');
            } else {
                if let Some(ch) = SignalPattern::parse_bit_to_char(abcdf_mask ^ pattern.bitmask) {
                    pattern_map.insert(9, pattern.clone());
                    char_map.insert(ch, 'g');
                    let e = SignalPattern::parse_missing_bit_to_char(pattern.bitmask).unwrap();
                    char_map.insert(e, 'e');
                    println!("found 9 and {} -> {} | {} -> {}", ch, 'g', e, 'e');
                } else {
                    pattern_map.insert(6, pattern.clone());
                    let c = SignalPattern::parse_missing_bit_to_char(pattern.bitmask).unwrap();
                    let f = SignalPattern::parse_bit_to_char(
                        one.bitmask ^ SignalPattern::parse_char(c),
                    )
                    .unwrap();
                    cdf_mask |= SignalPattern::parse_char(c);
                    cdf_mask |= SignalPattern::parse_char(f);
                    char_map.insert(c, 'c');
                    char_map.insert(f, 'f');
                    println!("found 6 and {} -> {} | {} -> {}", c, 'c', f, 'f');
                }
            }
        }

        println!("four {:b} cdf_mask {:b}", four.bitmask, cdf_mask);
        let b = SignalPattern::parse_bit_to_char(four.bitmask ^ cdf_mask).unwrap();
        char_map.insert(b, 'b');
        println!("{:?}", char_map);

        let _zero = pattern_map.get(&0).unwrap();
        let _six = pattern_map.get(&6).unwrap();
        let _nine = pattern_map.get(&9).unwrap();

        let mut value = 0;
        for (i, code) in line.output.iter().enumerate() {
            let scalar: usize = 10usize.pow(3 - i as u32);

            let mut mask = 0;
            for ch in code.chars() {
                let real_ch = char_map.get(&ch).unwrap();
                mask |= SignalPattern::parse_char(*real_ch);
            }

            let digit = SignalPattern::parse_mask_to_value(mask).unwrap();

            value += digit * scalar;
        }

        println!("Value is {}", value);
        sum += value;
        println!("------");
    }

    println!("[Part 2] count {:?}", sum);
    Ok(())
}
