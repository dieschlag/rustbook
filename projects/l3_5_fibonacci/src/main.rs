use std::io;

fn main() {

    
    let current_term = 0;
    let next_term = 1;
    let mut tup: (u64, u64) = (current_term, next_term);

    println!("Type which term of the fibnacci suite you want to get");
    
    let index: u64 = {
        
        let mut input = String::new();

        io::stdin()
        .read_line(&mut input)
        .expect("Faled to read line");
        input.trim().parse().expect("Type a positive integer")};

    println!("The index you chose is {index}");

    for _ in 0..index {
        tup = (tup.1, tup.0 + tup.1)
    }

    let result: u64 = tup.0;

    println!("The element of index {index} in the fibonacci suite is {result}");
}


