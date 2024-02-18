
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    println!("The constant can be used everywhere in this program and its value is {THREE_HOURS_IN_SECONDS}");

    second();

    let spaces = "   ";
    println!("{spaces}");
    let spaces = spaces.len();
    println!("{spaces}");
}

fn second() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x out of that scope is: {x}");
}

