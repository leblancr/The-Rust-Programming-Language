#[derive(Debug)]

// named types but no values
struct Rectangle {
    width: u32,
    height: u32,
}

// impliment functions in Rectangle struct, like methods in a class
// impl means put these functions (methods) in that struct
// impliment in Rectangle struct these methods
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    fn width(&self) -> bool {
        self.width > 0
    }
    
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    
    // Self is alias for what comes after impl
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    // instantiate the struct by specifying
    // concrete values for each of the fields
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    let sq = Rectangle::square(3);  // returns an object
    
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    // .width() method witdth, .width field width
    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }
    
    println!("sq: {:?}", sq);
    println!("sq.area(): {}", sq.area());
    
}