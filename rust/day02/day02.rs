/*
Part 1
Calculate the spreadsheet's checksum. For each row, determine the difference between the largest value and the smallest value; the checksum is the sum of all of these differences.

For example, given the following spreadsheet:

5 1 9 5
7 5 3
2 4 6 8

    The first row's largest and smallest values are 9 and 1, and their difference is 8.
    The second row's largest and smallest values are 7 and 3, and their difference is 4.
    The third row's difference is 6.

In this example, the spreadsheet's checksum would be 8 + 4 + 6 = 18.
 */

fn checksum_of_spreadsheet(input: &str) -> u32 {
    input.lines()
        .map(|l| {
            let (min, max) = l.split_whitespace()
                .map(|v| v.parse::<u32>().unwrap())
                .fold((u32::max_value(), 0), |(mut min, mut max), i| {
                    if i < min {
                        min = i;
                    }
                    if i > max {
                        max = i;
                    }
                    (min, max)
                });
            max - min
        }).sum()
}

fn run_tests() {
    let input =
"5 1 9 5
7 5 3
2 4 6 8";

    assert_eq!(checksum_of_spreadsheet(input), 18);
}

fn main() {
    run_tests();

    let input: &str = "5 1 9 5
7 5 3
2 4 6 8";

    let sum = checksum_of_spreadsheet(input);

    println!("Sum: {}", sum);
}
