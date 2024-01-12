use rand::Rng;

#[derive(Debug)]

// named types but no values
struct Rectangle {
    width: u32,
    height: u32,
}

// implement functions in Rectangle struct, like methods in a class
// impl means put these functions (methods) in that struct
// implement in Rectangle struct these methods
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
    // Rectangle factory
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let num_rectangles = 25;
    let mut rng = rand::thread_rng();

    // Create list of Rectangles with random width/height
    let mut rectangles: Vec<Rectangle> = Vec::new();

    for _i in 0..num_rectangles {
        let random_width: u32 = rng.gen_range(1..=100);
        let random_height: u32 = rng.gen_range(1..=100);

        let rectangle_instance = Rectangle {
            width: random_width,
            height: random_height,
        };

        // Add the instance to the vector
        rectangles.push(rectangle_instance);
    };

    println!("rectangles: {:?}", rectangles);

    for rectangle in &rectangles {
        println!("rectangle: {:?}", rectangle);
        println!("rectangle area: {:?}", rectangle.area());

        // See how many rectangles this one can hold
        for rectangle2 in &rectangles {
            println!("Can rect1 hold rect2? {}", rectangle.can_hold(&rectangle2));
        }
    }

    let rect1 = Rectangle {
        width: 20,
        height: 30,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    let sq = Rectangle::square(3);  // returns an object with equal width/height
    
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    // .width() method width, .width field width
    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }
    
    println!("sq: {:?}", sq);
    println!("sq.area(): {}", sq.area());



}
