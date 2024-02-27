#[derive(Debug)]
enum UsState { 
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {

    let coin = Coin::Penny;
    let quarter = value_in_cents(coin);

    
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    // Example of an enum with other
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => reroll()
        //_ => reroll(), //if we want a catch-all without using the element as an argument to the function
    }
}

// Simple match example
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        } // comma at the end is optional
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

//Example of a match using Option :

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}


// The match arms must cover all possibilities :

//fn plus_one(x: Option<i32>) -> Option<i32> {
//    match x {
//         Some(i) => Some(i + 1),                  //Error when executing the function
//     }
// }

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}
fn reroll(){}
