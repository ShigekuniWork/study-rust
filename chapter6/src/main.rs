#[derive(Debug)]
enum UsState {
    Alabama,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

fn valu_in_cents(coin:Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

fn main() {
    let dime = Coin::Dime;
    let nickel = Coin::Nickel;
    let penny = Coin::Penny;
    let quarter = Coin::Quarter;
    
    let mut coin = valu_in_cents(dime);
    println!("{coin}");
    coin = valu_in_cents(nickel);
    println!("{coin}");
    coin = valu_in_cents(penny);
    println!("{coin}");
    coin = valu_in_cents(quarter(UsState::Alabama));
    println!("{coin}");
    
    let sample_optional = Some("option");
    match sample_optional {
        Some(s) => println!("{}", s),
        None=> println!("None"),
    }
}
