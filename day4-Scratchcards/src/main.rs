fn main() {
    part2();
}

fn part2() {
    let mut repeats = vec![1];
    let mut num_lines = 0;
    std::io::stdin()
        .lines()
        .flatten()
        .enumerate()
        .for_each(|(i, l)| {
            let l = String::from(l.trim_start_matches(|c| c != ':').trim_start_matches(": "));
            let mut l = l.split(" | ");
            let left = l
                .next()
                .unwrap()
                .split_whitespace()
                .map(|s| s.parse::<usize>())
                .flatten()
                .collect::<Vec<usize>>();
            let mut matches = 0;
            for result in l
                .next()
                .unwrap()
                .split_whitespace()
                .map(|l| l.parse::<usize>())
                .flatten()
            {
                left.contains(&result).then(|| matches += 1);
            }
            while i + matches + 1 > repeats.len() {
                repeats.push(1)
            }
            for j in (i + 1)..(i + matches + 1) {
                println!(
                    "Adding {} to {}, Card: {}",
                    repeats[i] * matches,
                    repeats[j],
                    j + 1
                );
                repeats[j] += repeats[i];
            }
            num_lines = i + 1;
        });
    println!(
        "{:?}\nPart 2: {}",
        repeats,
        repeats.iter().take(num_lines).sum::<usize>()
    );
}

fn part1() {
    println!(
        "{:?}",
        std::io::stdin()
            .lines()
            .flatten()
            .map(|l| {
                let l = String::from(l.trim_start_matches(|c| c != ':').trim_start_matches(": "));
                let mut l = l.split(" | ");
                let left = l
                    .next()
                    .unwrap()
                    .split_whitespace()
                    .map(|s| s.parse::<usize>())
                    .flatten()
                    .collect::<Vec<usize>>();
                let mut pow = 0;
                for result in l
                    .next()
                    .unwrap()
                    .split_whitespace()
                    .map(|l| l.parse::<usize>())
                    .flatten()
                {
                    left.contains(&result).then(|| pow += 1);
                }
                if pow == 0 {
                    0
                } else {
                    1 << (pow - 1)
                }
            })
            .sum::<usize>()
    );
}
