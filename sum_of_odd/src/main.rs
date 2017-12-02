fn row_sum_odd_numbers(n:i64) -> i64 {
    let mut y = 0;
    for i in 1..n {
        y += i;
    }

    let v = vec![0; y];

    for i in 1..y {
        if i%2 != 0 {
            println!("{}", i);
        }
    }

    return 5;
}

fn main() {
    println!("Hello, world!");
    row_sum_odd_numbers(10);
}
