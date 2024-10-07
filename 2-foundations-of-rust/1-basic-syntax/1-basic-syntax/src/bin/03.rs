fn main() {
    let input = [23, 82, 16, 45, 21, 94, 12, 34];

    let mut smallest = 999999;
    let mut biggest = -1;
    for item in input {
        if item < smallest {
            smallest = item;
        }
        if item > biggest {
            biggest = item;
        }
    }

    println!("{} is largest and {} is smallest", biggest, smallest);
}
