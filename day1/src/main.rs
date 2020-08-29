use std::fs;

fn main() {
    let content = fs::read_to_string("input.txt")
        .expect("error");
    let mut sum = 0;
    for frequency in content.split("\n") {
        if frequency == "" {
            continue;
        }
        let int_freq: i32 = frequency.parse().unwrap();
        sum += int_freq;
    }
    println!("{}", sum);
}
