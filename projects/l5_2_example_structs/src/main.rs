//Goal : make a programm that calculates the area of a rectangle


//From idea 2 :
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {

    //First version :

    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.\n",
        area_first_version(width1, height1)
    );

    //Issue : we can't know that width and height are related
    //Idea 1 : using tuples : only one argument but elements inside a tuple are not named, calculation is less obvious, more risk to make errors
    //Idea 2 : use structs :

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("Printing in debug without # : rect1 is {:?}\n", rect1);
    println!("Printing in debug with # : rect1 is {:#?}\n", rect1);

    //Using dbg! :

    dbg!(&rect1);

    println!("The area of the rectangle is : {}", area_with_structs(&rect1));

}


fn area_first_version(width: u32, height: u32) -> u32 {
    width * height
}
fn area_with_structs(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}