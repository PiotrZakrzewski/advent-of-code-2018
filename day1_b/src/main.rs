use std::fs;
use std::collections::HashSet;

fn main() {
    let content = fs::read_to_string("input.txt")
        .expect("error");
    let mut seen_frequencies:HashSet<i32> = vec![].into_iter().collect();
    let mut sum = 0;
    let mut seen: bool = false;
    loop {
        for frequency in content.split("\n") {
            if frequency == "" {
                continue;
            }
            let int_freq: i32 = frequency.parse().unwrap();
            sum += int_freq;
            if !seen_frequencies.insert(sum) {
                seen = true;
                break;
            }
        }
        if seen {
            break;
        }
    }
    println!("{}", sum);
}
