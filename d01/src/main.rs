fn main() -> std::io::Result<()> {
    let input: Vec<usize> = common::get_input_vec()?;

    let mut increases = 0;
    for (i, val) in input[1..].iter().enumerate() {
        let last = input[i];

        if *val > last {
            increases += 1;
        }
    }

    println!("[P1] increases {:?}", increases);

    increases = 0;
    let mut window = input[0] + input[1] + input[2];
    for (i, val) in input[3..].iter().enumerate() {
        let prev_win = window;
        window += val;
        window -= input[i];

        if window > prev_win {
            increases += 1;
        }
    }

    println!("[P2] window increases {:?}", increases);

    Ok(())
}
