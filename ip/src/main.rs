enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

impl Coin {
    fn val_in_cents(&self) -> u32 {
        match self {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }
}

fn main() {
    let q: Coin = Coin::Quarter;
    let p = Coin::Penny;
    let evil = Coin::Nickel;
    
    println!("{}", q.val_in_cents());
    println!("{}", evil.val_in_cents());
    println!("{}", p.val_in_cents());
}



