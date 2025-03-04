fn count_chars(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            format!("\"{}\"", line.replace(r"\", r"\\").replace(r#"""#, r#"\""#)).len() - line.len()
        })
        .sum()
}

fn main() {
    println!("{}", count_chars(include_str!("../input.txt")));
}
