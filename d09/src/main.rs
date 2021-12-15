#![feature(string_remove_matches)]
struct FloodFillContext {
    height_map: Vec<u32>,
    explored_map: Vec<bool>,
    rows: usize,
    cols: usize,
    current_basin: usize,
}

fn main() -> std::io::Result<()> {
    let input: Vec<String> = common::get_input_vec()?;
    let cols = input[0].len();
    let rows = input.len();

    let height_map: Vec<u32> = input
        .join("")
        .chars()
        .map(|x| x.to_digit(10).unwrap())
        .collect();
    let count = height_map.len();

    let mut sum = 0;
    for (i, height) in height_map.iter().enumerate() {
        let row = i / cols;
        let col = i % cols;
        let mut check_idx;
        // NESW

        if row != 0 {
            check_idx = i - cols;
            if height_map[check_idx] <= *height {
                println!("removing north");
                continue;
            }
        }

        if col != cols - 1 {
            check_idx = i + 1;
            if height_map[check_idx] <= *height {
                println!("removing east");
                continue;
            }
        }

        if row != rows - 1 {
            check_idx = i + cols;
            if height_map[check_idx] <= *height {
                println!("removing south");
                continue;
            }
        }

        if col != 0 {
            check_idx = i - 1;
            if height_map[check_idx] <= *height {
                println!("removing west");
                continue;
            }
        }

        sum += 1 + height;
    }

    println!("[Part 1] sum {}", sum);

    let mut basins: Vec<usize> = Vec::new();
    let explored_map = vec![false; count];
    let mut ff_ctx = FloodFillContext {
        height_map,
        explored_map,
        rows,
        cols,
        current_basin: 0,
    };

    fn flood_fill(ff_ctx: &mut FloodFillContext, idx: usize) {
        if ff_ctx.height_map[idx] == 9 || ff_ctx.explored_map[idx] {
            return;
        }
        ff_ctx.explored_map[idx] = true;
        ff_ctx.current_basin += 1;
        let row = idx / ff_ctx.cols;
        let col = idx % ff_ctx.cols;
        let mut next_idx;
        if row != 0 {
            next_idx = idx - ff_ctx.cols;
            flood_fill(ff_ctx, next_idx);
        }

        if col != ff_ctx.cols - 1 {
            next_idx = idx + 1;
            flood_fill(ff_ctx, next_idx);
        }

        if row != ff_ctx.rows - 1 {
            next_idx = idx + ff_ctx.cols;
            flood_fill(ff_ctx, next_idx);
        }

        if col != 0 {
            next_idx = idx - 1;
            flood_fill(ff_ctx, next_idx);
        }
    }

    for i in 0..count {
        ff_ctx.current_basin = 0;
        flood_fill(&mut ff_ctx, i);

        if ff_ctx.current_basin != 0 {
            basins.push(ff_ctx.current_basin);
        }
    }

    basins.sort();
    basins.reverse();
    println!("{:?}", basins);
    println!("[part 2] {:?}", basins[0] * basins[1] * basins[2]);

    Ok(())
}
