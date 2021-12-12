fn main() -> std::io::Result<()> {
    let mut fish_buffer: Vec<u8> = common::get_input_vec_split(",")?;

    println!("[Part 1] Fish Count start {}", fish_buffer.len());
    for _day in 0..80 {
        let mut new_fish = 0;
        for fish in fish_buffer.iter_mut() {
            if *fish == 0 {
                new_fish += 1;
                *fish = 6;
            } else {
                *fish -= 1;
            }
        }
        let mut fish_slice = vec![8u8; new_fish];
        fish_buffer.append(&mut fish_slice);
    }
    println!("[Part 1] Fish Count End {}", fish_buffer.len());

    let fish_buffer: Vec<u8> = common::get_input_vec_split(",")?;
    let mut fish_buckets = [0usize; 9];
    for fish in fish_buffer {
        fish_buckets[fish as usize] += 1;
    }

    for _day in 0..256 {
        let cache_buckets = fish_buckets.clone();

        for i in 0..8 {
            fish_buckets[7 - i] = cache_buckets[8 - i];
        }

        fish_buckets[8] = cache_buckets[0];
        fish_buckets[6] += cache_buckets[0];
    }
    println!(
        "[Part 2] Fish Bucket Count End {}",
        fish_buckets.iter().sum::<usize>()
    );

    Ok(())
}
