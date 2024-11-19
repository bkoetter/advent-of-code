use regex::Regex;

fn main() -> std::io::Result<()> {
    let lines = std::fs::read_to_string("input.txt")?.lines()
        .map(String::from).collect::<Vec<String>>();

    let default = &".".repeat(lines[0].len());
    let re = Regex::new(r"\d+").unwrap();

    let mut sum: usize = 0;

    for (n, line) in lines.iter().enumerate() {
        println!("{line}");
        for num in re.find_iter(line) {
            'outer: for mut idx in num.start()..num.end() + 2 {
                if idx == 0 {
                    continue;
                }
                idx -= 1;
                for relative_line in 0..=2 {
                    if n == 0 && relative_line == 2 {
                        continue;
                    }
                    let line = &lines.get(n + 1 - relative_line).unwrap_or(default);
                    let char = line.chars().nth(idx).unwrap_or('.');
                    if char.is_ascii_digit() || char == '.' {
                        continue;
                    }
                    println!("{}", num.as_str());
                    sum += num.as_str().parse::<usize>().unwrap();
                    break 'outer;
                }
            }
        }
    }
    println!("{}", sum);
    Ok(())
}
