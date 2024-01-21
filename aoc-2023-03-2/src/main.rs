fn main() -> std::io::Result<()> {
    let re = regex::Regex::new(r"\d+").unwrap();
    let file = std::fs::read_to_string("input.txt")?;
    file.lines().enumerate().map(
        |(_, line)| line.chars().enumerate().filter_map(|(i, c)| match c {
            '*' => {
                re.find_iter(line).find(|m| {
                    let start = if m.start() > 0 { m.start() - 1 } else { m.start() };
                    let end = if m.end() < line.len() { m.end() + 1 } else { m.end() };
                    (start..end).contains(&i)
                }).map(|m| m.as_str().parse::<usize>().unwrap())
            }
            _ => None,
        }).collect::<Vec<usize>>()
    )
    .filter(|v| !v.is_empty())
        .for_each(|line| {
        println!("{:?}", line);
    });
    Ok(())
}
