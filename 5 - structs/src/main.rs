use rand::Rng;

// named types but no values
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
    rects_that_fit: Vec<usize>, // Store indices instead of references
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
            rects_that_fit: vec![],
        }
    }
}

fn main() {
    let num_rectangles = 250;
    let mut rng = rand::thread_rng();

    // Create list of Rectangles with random width/height
    let mut rectangles: Vec<Rectangle> = Vec::new();

    for _i in 0..num_rectangles {
        let random_width: u32 = rng.gen_range(1..=100);
        let random_height: u32 = rng.gen_range(1..=100);

        let rectangle_instance = Rectangle {
            width: random_width,
            height: random_height,
            rects_that_fit: vec![],
        };

        // Add the instance to the vector
        rectangles.push(rectangle_instance);
    };

    println!("rectangles: {:?}", rectangles);

    // Take each rectangle one by one and go through the whole list counting how many other
    // rectangles fit into this one.
    for i in 0..rectangles.len() {
        println!("rectangle {i}: {:#?}", rectangles[i]);
        println!("rectangle {i} area: {:?} pxls.", rectangles[i].area());
        dbg!(rectangles[i].area());

        // See how many rectangles this one can hold
        for j in 0..rectangles.len() {
            println!("Can rect {i} {}x{} {} pxls. hold rect {j} {}x{} {} pxls.? {}",
                     rectangles[i].width,
                     rectangles[i].height,
                     rectangles[i].area(),
                     rectangles[j].width,
                     rectangles[j].height,
                     rectangles[j].area(),
                     rectangles[i].can_hold(&rectangles[j]));

            if rectangles[i].can_hold(&rectangles[j]) {
                // rectangles[i].fits += 1;
                rectangles[i].rects_that_fit.push(j);
            }
        }
        println!("rect {i} {}x{} {} can hold {} other rectangle{}",
                 rectangles[i].width,
                 rectangles[i].height,
                 rectangles[i].area(),
                 rectangles[i].rects_that_fit.len(),
                 if rectangles[i].rects_that_fit.is_empty() || rectangles[i].rects_that_fit.len() > 1 {"s".to_string()} else {"".to_string()});

        println!("rectangles that rect {i} can hold:");
        for j in &rectangles[i].rects_that_fit {
            // println!("{:?}:", rectangles[*j]);
            println!("rect {j} {}x{} {}",
                     rectangles[*j].width,
                     rectangles[*j].height,
                     rectangles[*j].area());
        }
    }

    let rect1 = Rectangle {
        width: 20,
        height: 30,
        rects_that_fit: vec![],
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
        rects_that_fit: vec![],
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
        rects_that_fit: vec![],
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
