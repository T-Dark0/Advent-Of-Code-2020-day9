use std::{
    cmp::Ordering,
    collections::{HashSet, VecDeque},
    fs,
};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let input = input
        .lines()
        .map(|n| n.parse().unwrap())
        .collect::<Vec<_>>();

    part1(&input);
    part2(&input);
}

fn part1(input: &[i64]) {
    println!("First invalid number: {}", first_invalid_number(input));
}

fn part2(input: &[i64]) {
    let mut subsequence_start = 0;
    let mut subsequence_end = 0;
    let mut current_sum = 0;
    let target_sum = first_invalid_number(input);
    loop {
        match current_sum.cmp(&target_sum) {
            Ordering::Less => {
                current_sum += input[subsequence_end];
                subsequence_end += 1;
            }
            Ordering::Equal => break,
            Ordering::Greater => {
                current_sum -= input[subsequence_start];
                subsequence_start += 1;
            }
        }
    }

    let subsequence = &input[subsequence_start..=subsequence_end];
    let min = subsequence.iter().min().unwrap();
    let max = subsequence.iter().max().unwrap();

    println!("Encryption weakness: {}", min + max);
}

fn first_invalid_number(input: &[i64]) -> i64 {
    let (preamble, rest) = input.split_at(25);
    let mut recent_order = preamble.iter().copied().collect::<VecDeque<_>>();
    let mut recent_nums = preamble.iter().copied().collect::<HashSet<_>>();
    *rest
        .into_iter()
        .find(|&&current_num| {
            let is_current_valid = recent_nums
                .iter()
                .find(|&&recent_num| recent_nums.contains(&(current_num - recent_num)))
                .is_some();

            let oldest_num = recent_order.pop_front().unwrap();
            recent_nums.remove(&oldest_num);
            recent_order.push_back(current_num);
            recent_nums.insert(current_num);

            !is_current_valid
        })
        .unwrap()
}
