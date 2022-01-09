fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    let rect1 = (30, 50);

    println!(
        "The area of the tuple rectangle is {} square pixels.",
        area_tuple(rect1)
    );

    let rectangle1 = Rectangle {
        width: 30, 
        height: 50
    };


    println!(
        "The area of the struct rectangle is {} square pixels.",
        area_struct(&rectangle1)
    );

    println!("----------------------");
    println!("rectangle1 is {:?}", rectangle1);

    println!("----------------------");
    println!("rectangle1 is {:#?}", rectangle1);

    let rect3 = Rect {
        width: 30, 
        height: 50,
    };

    println!("The area of the Rect is {} square pixels", rect3.area());

    println!("----------------------");

    let rect4 = Rect {
        width: 20,
        height: 40,
    };

    let rect5 = Rect {
        width: 40,
        height: 40,
    };

    println!("Can rect3 hold rect4? {}", rect3.can_hold(&rect4));
    println!("Can rect3 hold rect5? {}", rect3.can_hold(&rect5));
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

// the rectangle is borrowed from main (by reference) so that ownership doesn't change
// if ownership changed to this function then when this function exits the rectangle would be destroyed
fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32,
}

impl Rect {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other_rectangle: &Rect) -> bool {
        self.width > other_rectangle.width && self.height > other_rectangle.height

    }

    fn square(side: u32) -> Rect {
        Rect {
            width: side,
            height: side,
        }
    }
}

