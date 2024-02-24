#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // We can give a methode the same name as on of the struct's field

    fn width(&self) -> bool {
        self.width > 0
    }

    // A method can take several parameters :

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// It is possible to have multiplr impl in the same code :

impl Rectangle {
    // Example of associated function :

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    //Calling an associated function:

    let sq = Rectangle::square(3);
}