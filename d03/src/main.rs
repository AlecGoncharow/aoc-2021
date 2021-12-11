#![feature(drain_filter)]
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone)]
struct ReportEntry(u16);

impl FromStr for ReportEntry {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match u16::from_str_radix(s, 2) {
            Ok(n) => Ok(ReportEntry { 0: n }),
            Err(_) => Err(Self::Err::new(
                std::io::ErrorKind::Other,
                format!("Oh No! {} is not a u16", s),
            )),
        }
    }
}

fn main() -> std::io::Result<()> {
    let input: Vec<ReportEntry> = common::get_input_vec()?;

    let mut counts = [0usize; 12];

    println!("{:?}", input[0]);

    input.iter().for_each(|entry| {
        let val = entry.0;
        for i in 0..12usize {
            if val & (1 << i) > 0 {
                counts[i] += 1;
            }
        }
    });

    let threshold = input.len() / 2;
    println!("threshold: {}", threshold);
    let mut gamma = 0;
    for i in 0..12usize {
        if counts[i] > threshold {
            println!("count above thresh: {} {}", i, counts[i]);
            gamma |= 1 << i;
        }
    }

    let epsilon = !gamma & 0b0000_1111_1111_1111;

    println!(
        "gamma = {} | {:#012b} \nepsilon = {} | {:#012b}",
        gamma, gamma, epsilon, epsilon
    );

    println!("[Part 1] power consumption: {}", gamma * epsilon);

    let mut count;
    let mut buffer = input.clone();
    let mut bit = 11;
    let oxy_rating = loop {
        count = 0;

        buffer.iter().for_each(|entry| {
            if (entry.0 & (1 << bit)) > 0 {
                count += 1;
            }
        });

        let filter: Box<dyn FnMut(&mut ReportEntry) -> bool> = if count >= (buffer.len() / 2) {
            Box::new(|entry: &mut ReportEntry| (entry.0 & (1 << bit)) > 0)
        } else {
            Box::new(|entry: &mut ReportEntry| (entry.0 & (1 << bit)) == 0)
        };

        buffer = buffer.drain_filter(filter).collect();

        println!(
            "[Bit {}] Count {} Buffer len {} Sample {:012b}",
            bit,
            count,
            buffer.len(),
            buffer[0].0
        );
        if buffer.len() == 1 {
            break buffer[0].clone();
        }

        if bit == 0 {
            panic!("help");
        }
        bit -= 1;
    };
    println!("[Part 2] Oxy Rating: {:?}", oxy_rating);

    let mut count;
    let mut buffer = input.clone();
    let mut bit = 11;
    let scrubber_rating = loop {
        count = 0;

        buffer.iter().for_each(|entry| {
            if (entry.0 & (1 << bit)) > 0 {
                count += 1;
            }
        });

        let filter: Box<dyn FnMut(&mut ReportEntry) -> bool> = if count < (buffer.len() / 2) {
            Box::new(|entry: &mut ReportEntry| entry.0 & (1 << bit) > 0)
        } else {
            Box::new(|entry: &mut ReportEntry| entry.0 & (1 << bit) == 0)
        };

        buffer = buffer.drain_filter(filter).collect();

        println!(
            "[Bit {}] Count {} Buffer len {} Sample {:012b}",
            bit,
            count,
            buffer.len(),
            buffer[0].0
        );
        if buffer.len() == 1 {
            break buffer[0].clone();
        }

        if bit == 0 {
            panic!("help");
        }
        bit -= 1;
    };
    println!("[Part 2] Scrubber Rating: {:?}", scrubber_rating);

    println!(
        "[Part 2] Life Support Rating: {:?}",
        scrubber_rating.0 as usize * oxy_rating.0 as usize
    );

    Ok(())
}
