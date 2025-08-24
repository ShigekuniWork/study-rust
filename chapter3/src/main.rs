const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    println!("{THREE_HOURS_IN_SECONDS}");

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    let big_number = 1_000_000;

    println!("{big_number}");

    let sample = num_sample();
    println!("{sample}");

    let result = sample_flow(sample);
    println!("{result}");

    let state = true;
    let number = if state { 5 } else { 6 };
    println!("{number}");
}

fn num_sample() -> i32 {
    4
}

fn sample_flow(num: i32) -> bool {
    if num < 5 {
        false
    } else if num == 3 {
        false
    } else {
        true
    }
}
