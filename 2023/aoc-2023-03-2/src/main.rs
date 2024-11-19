fn main() -> std::io::Result<()> {
    let re = regex::Regex::new(r"\d+").unwrap();
    let file: Vec<String> = std::fs::read_to_string("input.txt")?
        .lines()
        .map(|s| s.to_string())
        .collect();

    file.iter().enumerate().map(
        |(n, line)| line.chars().enumerate().filter_map(|(i, c)| match c {
            '*' => { [0, 1, 2].iter().map(|j| {
                let line = &file.get(n+1-j)?;
                re.find_iter(line).find(|m| {
                    let start = if m.start() > 0 { m.start() - 1 } else { m.start() };
                    let end = if m.end() < file[n+1-j].len() { m.end() + 1 } else { m.end() };
                    (start..end).contains(&i)
                }).map(|m| m.as_str().parse::<usize>().unwrap())
            }).collect::<Option<Vec<usize>>>() },
            _ => None,
        }).collect::<Vec<Vec<usize>>>()
    )
    .filter(|v| !v.is_empty())
        .for_each(|line| {
        println!("{:?}", line);
    });
    Ok(())
}
