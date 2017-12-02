/*
Part 1

Review a sequence of digits (your puzzle input) and find the sum of all digits that match the next digit in the list. The list is circular, so the digit after the last digit is the first digit in the list.

For example:

    1122 produces a sum of 3 (1 + 2) because the first digit (1) matches the second digit and the third digit (2) matches the fourth digit.
    1111 produces 4 because each digit (all 1) matches the next.
    1234 produces 0 because no digit matches the next.
    91212129 produces 9 because the only digit that matches the next one is the last digit, 9.

Part 2

Now, instead of considering the next digit, it wants you to consider the digit halfway around the circular list. That is, if your list contains 10 items, only include a digit in your sum if the digit 10/2 = 5 steps forward matches it. Fortunately, your list has an even number of elements.

For example:

    1212 produces 6: the list contains 4 items, and all four digits match the digit 2 items ahead.
    1221 produces 0, because every comparison is between a 1 and a 2.
    123425 produces 4, because both 2s match each other, but no other digit has a match.
    123123 produces 12.
    12131415 produces 4.
*/

fn sum_of_matching_digits(bytes: &[u8], offset: usize) -> u32 {
    bytes.iter()
        .zip(
            bytes.iter()
                .cycle()
                .skip(offset)
        )
        .fold(0, |acc: u32, (&a, &b)| {
            if a == b {
                acc + ((a - '0' as u8) as u32)
            } else {
                acc
            }
        })
}

fn run_tests() {
    // Part 1 tests
    assert_eq!(sum_of_matching_digits("1122".as_bytes(), 1), 3);
    assert_eq!(sum_of_matching_digits("1111".as_bytes(), 1), 4);
    assert_eq!(sum_of_matching_digits("1234".as_bytes(), 1), 0);
    assert_eq!(sum_of_matching_digits("91212129".as_bytes(), 1), 9);

    // Part 2 tests
    assert_eq!(sum_of_matching_digits("1212".as_bytes(), 2), 6);
    assert_eq!(sum_of_matching_digits("1221".as_bytes(), 2), 0);
    assert_eq!(sum_of_matching_digits("123425".as_bytes(), 3), 4);
    assert_eq!(sum_of_matching_digits("123123".as_bytes(), 3), 12);
    assert_eq!(sum_of_matching_digits("12131415".as_bytes(), 4), 4);
}

fn main() {
    run_tests();

    let input: &str = "1234";
    let bytes = input.as_bytes();

    // Part 1
    let sum: u32 = sum_of_matching_digits(bytes, 1);
    println!("Part 1:");
    println!("\tString: {}\n\tSum: {}", input, sum);

    // Part 2
    let sum: u32 = sum_of_matching_digits(bytes, bytes.len()/2);
    println!("Part 2:");
    println!("\tString: {}\n\tSum: {}", input, sum);
}
