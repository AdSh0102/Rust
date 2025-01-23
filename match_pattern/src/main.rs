use std::io;

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

impl Coin
{
    pub fn value_in_cents(&self) -> u8 {
        match self {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Coin::Penny => "Penny",
            Coin::Nickel => "Nickel",
            Coin::Dime => "Dime",
            Coin::Quarter => "Quarter",
        }
    }
}

impl std::fmt::Display for Coin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({} cents)", self.name(), self.value_in_cents())
    }
}

fn main() {
    let mut inp: String = String::new();
    io::stdin()
        .read_line(&mut inp)
        .expect("Failed to read line\n");
    let coin: Coin;
    match inp.trim().to_lowercase().as_str() {
        "penny" => coin = Coin::Penny,
        "nickel" => coin = Coin::Nickel,
        "dime" => coin = Coin::Dime,
        "quarter" => coin = Coin::Quarter,
        _ => {
            println!("Invalid input!");
            return;
        }
    }
    println!("{}", (coin));
}
