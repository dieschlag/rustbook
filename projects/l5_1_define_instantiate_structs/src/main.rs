struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    // Defining a struct

    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    //If user1 is mut, we can change a value using the dot notation :

    user1.email = String::from("anotheremail@example.com");

    //Creating a new user without the update synthax

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };

    //Better way to do it with the .. synthax

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    //Example of tuple structs :

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
}

//Constructor :

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

// But no need to repeat the fields, we can use the field init shortand :

fn build_user_no_repeat(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

