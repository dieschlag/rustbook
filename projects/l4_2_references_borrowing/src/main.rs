fn main() {

    //Usage of references

    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    //Unable to modify reference

    //let s = String::from("hello");

    //change(&s);

    //Mutable References

    let mut s = String::from("hello");

    change_mut(&mut s);

    println!("{s}");

    //Can have only one reference to a mutable reference
    
    let r1 = &mut s;
    //let r2 = &mut s; //This line generates an error

    //println!("{},{}", r1, r2);

    //Combining mutable and immutable references

    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    //let r3 = &mut s; // BIG PROBLEM

    //println!("{}, {}, and {}", r1, r2, r3);

    // Right way to deal with reference scope : begins first time it is used and ends last time it is used

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);

    // Dandling pointer :

    //let reference_to_nothing = dangle();

    //Solution : return the string directly

    let reference=no_dangle();
    
    println!("{reference}")

}


fn calculate_length(s: &String) -> usize {
    s.len()
}// Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, it is not dropped.


  fn change(some_string: &String) {
    //some_string.push_str(", world");
}

fn change_mut(some_string: &mut String) {
    some_string.push_str(", world");
}

//fn dangle() -> &String { // dangle returns a reference to a String
    //let s = String::from("hello");// s is a new String

    //&s // we return a reference to the String, s
//} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}