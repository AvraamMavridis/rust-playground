use std::io;

fn main() {
    println!("Enter Fahrenheit value");
    let mut fahr = String::new();
    io::stdin().read_line(&mut fahr).expect("Failed to read line");
    let fahr: f32 = match fahr.trim().parse() {
        Ok(num) => num,
        Err(e) => {
            println!("error parsing header: {:?}", e);
            -1.0
        }
    };

    const PARAGON: f32 = 0.555555555555;
    let res: f32 = (fahr - 32.0) * PARAGON;
    println!("Fahrenheit: {}, Celsius: {}", fahr, res);
}
