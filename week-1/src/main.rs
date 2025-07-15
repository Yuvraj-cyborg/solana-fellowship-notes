mod macros;
struct Rect {
    width: u32,
    height: u32,
}
struct Square {
    side: u32,
}

trait Shape {
    fn area(&self) -> u32;
    fn perimeter(&self) -> u32;
}

impl Shape for Rect {
    //self is the instance of the struct
    //&self is a reference to the instance, so we can use it without taking ownership
    //&self is used to call methods on the instance
    //fn area(&self) -> u32 is a method that returns the area of the rectangle
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }
}
impl Shape for Square {
    fn area(&self) -> u32 {
        self.side * self.side
    }
    fn perimeter(&self) -> u32 {
        4 * self.side
    }
}

fn main() {
    //println! macro -> macro expansion -> binary -> run
    println!("Hello, world!");
    let r = Rect {
        width: 30,
        height: 50,
    };
    let c = Square {
        side: 10,
    };
    let (area, perimeter) = get_area_perimeter(&r);
    println!("Rectangle area: {}, perimeter: {}", area, perimeter);
    macros::macro_print();
}

fn get_area_perimeter(s: &impl Shape) -> (u32, u32) {
    (s.area(), s.perimeter())
}