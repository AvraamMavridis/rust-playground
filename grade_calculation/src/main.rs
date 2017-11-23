// Your program should ask the user for the four assignment scores, the midterm, final and section grades. Then, the final score is calculated and printed. To do the calculations, you average the four assignment scores together and then multiply it by 0.4 (40%). You then multiply the midterm score by 0.15, the final by 0.35 and the participation grade by 0.1. Then you add all the results of these multiplications together.

use std::io;

fn read_score(msg: &str, vector: &mut Vec<u32>){
    println!("{}",msg);
    let mut grade = String::new();
    io::stdin().read_line(&mut grade).expect("Cannot read line");
    let val: u32 = grade
        .trim()
        .parse()
        .expect("Expected a number");

    vector.push(val);
}

fn main() {
    let mut vector: Vec<u32> = Vec::new();
    for number in 1..5 {
        let msg = format!("Enter the score of the {} assignment", number);
        read_score(&msg, &mut vector);
    }

    let msg = "Enter the score for the midterm.";
    read_score(&msg, &mut vector);

    let msg = "Enter the score for the final.";
    read_score(&msg, &mut vector);

    let msg = "Enter the score for the section grade.";
    read_score(&msg, &mut vector);

    println!("{}", vector[0]);

    let sum :u32 = vector[0..4].iter().sum();
    let sum = sum as f32;
    let mut avg :f32 = (sum/4.0) * 0.4;
    avg += (vector[5] as f32) * 0.15;
    avg += (vector[6] as f32) * 0.35;
    avg += (vector[6] as f32) * 0.1;
    println!("Final Grade: {}", avg);
}
