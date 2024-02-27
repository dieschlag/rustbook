enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState { 
    Alabama,
    Alaska,
}

fn main() {
    // Example of if let usage : 

    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
    //Using else :

    let origin = UsState::Alaska;
    let coin = Coin::Quarter(origin);
    let mut count = 0;
    
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}
