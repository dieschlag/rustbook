// Defining an enum

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// We can implement methods on enums

impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

enum IpAddrKindNoType {
    V4,
    V6,
}

// Using struct to be able to store IP adress data :

struct IpAddr {
    kind: IpAddrKindNoType,
    address: String,
}

//We can add the type directly in the enum definition

enum IpAddrWithType {
    V4((u8, u8, u8, u8)),
    V6(String),
}

fn main() {
    let home = IpAddr {
        kind: IpAddrKindNoType::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKindNoType::V6,
        address: String::from("::1"),
    };

    //With the new way to define enums :

    let home = IpAddrWithType::V4((127, 0, 0, 0));

    let loopback = IpAddrWithType::V6(String::from("::1"));

    // Calling an enum implementation method :

    let m = Message::Write(String::from("hello"));
    m.call();

    // Use of the Option enum :

    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;

    // Compatibility between types and Option types

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    //let sum = x + y; Error

    // Convert Option<T> to T :

    let x = Some("value");
    assert_eq!(x.expect("the value of x should be a str"), "value");

}
