#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn rectangle_to_area(&self, rectangle: &Rectangle) -> u32 {
        rectangle.width * rectangle.height
    }

    fn bigger_area_than(&self, rectangle: &Rectangle) -> bool {
        self.area() < self.rectangle_to_area(&rectangle)
    }
}

fn main() {
    let room_rectangle = Rectangle {
        width: 30,
        height: 50,
    };

    let home_rectangle = Rectangle {
        width: 250,
        height: 300,
    };

    println!("{}", room_rectangle.bigger_area_than(&home_rectangle));
}
