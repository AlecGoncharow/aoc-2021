fn error_score(ch: char) -> usize {
    match ch {
        ')' => 00003,
        ']' => 00057,
        '}' => 01197,
        '>' => 25137,
        _ => unreachable!(),
    }
}

fn completion_score(ch: char) -> usize {
    match ch {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => unreachable!(),
    }
}

fn compare_open_close(open: char, close: char) -> bool {
    match open {
        '(' => close == ')',
        '[' => close == ']',
        '{' => close == '}',
        '<' => close == '>',
        _ => unreachable!(),
    }
}

fn main() -> std::io::Result<()> {
    let input: Vec<String> = common::get_input_vec()?;

    println!("{}", input[0]);

    let mut sum = 0;
    let mut stack: Vec<char> = Vec::new();
    let mut incomplete_line_stacks = Vec::with_capacity(input.len());
    for line in input {
        stack.clear();

        let mut corrupted = false;
        for ch in line.chars() {
            match ch {
                '(' | '[' | '{' | '<' => stack.push(ch),
                _ => {
                    if let Some(open) = stack.pop() {
                        if !compare_open_close(open, ch) {
                            sum += error_score(ch);
                            corrupted = true;
                            break;
                        }
                    } else {
                        sum += error_score(ch);
                        corrupted = true;
                        break;
                    }
                }
            }
        }
        if !corrupted {
            incomplete_line_stacks.push(stack.clone());
        }
    }

    println!("[Part 1] sum {}", sum);

    let mut completion_scores = Vec::new();
    for stack in incomplete_line_stacks.iter_mut() {
        let mut score = 0;
        while let Some(open) = stack.pop() {
            score *= 5;
            score += completion_score(open);
        }
        completion_scores.push(score);
    }

    completion_scores.sort();
    println!(
        "[Part 2] middle {}",
        completion_scores[(completion_scores.len() / 2)]
    );

    Ok(())
}
