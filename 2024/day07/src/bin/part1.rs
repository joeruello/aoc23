use common::Itertools;

fn main() {
    let input: String = common::AocInput::fetch(2024, 7).unwrap().into();
    println!("Output: {}", process(&input));
}

fn process(input: &str) -> usize {
    input
        .lines()
        .map(|l| {
            let (res, operands) = l.split_once(": ").expect("parsed");
            (
                res.parse::<usize>().unwrap(),
                operands
                    .split_whitespace()
                    .map(|n| n.parse::<usize>().unwrap())
                    .collect_vec(),
            )
        })
        .filter(|(res, operands)| guess_operator(operands, *res))
        .map(|(res, _)| res)
        .sum()
}

fn guess_operator(nums: &[usize], result: usize) -> bool {
    let add = nums[0] + nums[1];
    let mul = nums[0] * nums[1];

    match nums.len() {
        2 => add == result || mul == result,
        _ => {
            guess_operator(&[&[add], &nums[2..]].concat(), result)
                || guess_operator(&[&[mul], &nums[2..]].concat(), result)
        }
    }
}
