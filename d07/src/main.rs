#![feature(int_abs_diff)]

fn main() -> std::io::Result<()> {
    let mut input: Vec<usize> = common::get_input_vec_split(",")?;

    input.sort();

    let median_index = input.len() / 2;

    println!(
        "len {}, median_index {}, window {:?}",
        input.len(),
        median_index,
        &input[median_index - 2..median_index + 2]
    );

    let median = input[median_index];

    let mut cost = 0;
    for value in &input {
        cost += median.abs_diff(*value);
    }

    println!("[Part 1] cost {}", cost);

    let average: usize = (input.iter().sum::<usize>() as f64 / input.len() as f64).round() as usize;

    println!("avg {}", average);

    /*
     * Not useful but an interesting artifact of my thought process
    let avg_lo = average.floor() as usize;
    let avg_hi = average.ceil() as usize;

    let avg_lo_idx = input.iter().position(|val| *val == avg_lo);
    let avg_hi_idx = input.iter().position(|val| *val == avg_hi);

    println!("avg_lo_idx {:?} avg_hi_idx {:?}", avg_lo_idx, avg_hi_idx);
    */

    cost = 0;
    for value in &input {
        // https://math.stackexchange.com/questions/593318/factorial-but-with-addition/593323
        // https://en.wikipedia.org/wiki/Triangular_number
        let diff = average.abs_diff(*value);
        let triangle_number = (diff.pow(2) + diff) / 2;
        cost += triangle_number;
    }

    println!("[Part 2] cost {}", cost);

    let mut cost_low = usize::MAX;
    let mut location = 0;
    for i in 0usize..2000 {
        cost = 0;
        for value in &input {
            // https://math.stackexchange.com/questions/593318/factorial-but-with-addition/593323
            // https://en.wikipedia.org/wiki/Triangular_number
            let diff = i.abs_diff(*value);
            let triangle_number = (diff.pow(2) + diff) / 2;
            cost += triangle_number;
        }

        if i == 459 {
            println!("459 cost {}", cost);
        }

        if cost < cost_low {
            cost_low = cost;
            location = i;
        }
    }
    println!("[Part 2] cost {} location {}", cost_low, location);

    Ok(())
}
