fn main() {
    part1();
}

fn part2() {
    println!(
        "part2: {}",
        std::io::stdin()
            .lines()
            .map(|l| l.unwrap())
            .map(|l| {
                let mut firstIdx = usize::MAX;
                let mut lastIdx = usize::MIN;
                let mut first = 0;
                let mut last = usize::MAX;
                for (i, num) in [
                    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
                ]
                .iter()
                .enumerate()
                {
                    match l.find(num) {
                        Some(num) => {
                            if firstIdx > num {
                                firstIdx = num;
                                first = i + 1;
                            } else if lastIdx < num {
                                lastIdx = num;
                                last = i + 1;
                            }
                        }
                        None => {}
                    }
                }

                let mut iterator = l.chars().enumerate().filter(|(_, c)| c.is_ascii_digit());
                let firstTuple = iterator.next().unwrap();
                if firstTuple.0 < firstIdx {
                    first = firstTuple.1 as usize - '0' as usize
                }
                last = match iterator.last() {
                    Some((i, c)) => i,
                    None => first,
                };
                let mut digit = String::from(first);
                digit.push(last);
                digit.parse::<u32>().unwrap()
            })
            .sum::<u32>()
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
