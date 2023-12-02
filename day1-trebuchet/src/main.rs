fn main() {
    part2();
}

fn part2() {
    println!(
        "part2: {}",
        std::io::stdin()
            .lines()
            .map(|l| l.unwrap())
            .map(|l| {
                let mut l = l.clone();
                for (i, num) in [
                    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
                ]
                .into_iter()
                .enumerate()
                {
                    l = l.replace(num, format!("{}{}{}", num, i + 1, num).as_str())
                }

                let mut iterator = l.chars().filter(|c| c.is_ascii_digit());
                let first = iterator.next().unwrap();
                let last = match iterator.last() {
                    Some(l) => l,
                    None => first,
                };
                let mut digit = String::from(first);
                digit.push(last);
                println!("{}\n{}", l, digit);
                digit.parse::<usize>().unwrap()
            })
            .sum::<usize>()
    )
}

fn part1() {
    println!(
        "part1: {}",
        std::io::stdin()
            .lines()
            .map(|l| {
                let binding = l.unwrap();
                let mut iterator = binding.chars().filter(|c| c.is_ascii_digit());
                let first = iterator.next().unwrap();
                let last = match iterator.last() {
                    Some(l) => l,
                    None => first,
                };
                let mut digit = String::from(first);
                digit.push(last);
                digit.parse::<u32>().unwrap()
            })
            .sum::<u32>()
    );
}
