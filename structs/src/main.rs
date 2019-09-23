#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn double(&mut self) {
        self.width *= 2;
        self.height *=2;
    }

    fn can_hold(&self, x: &Rectangle) -> bool {
        if self.width > x.width && self.height > x.height {
            return true
        } 
        else{
            return false
        }
    }

    fn transpose(self) -> Rectangle {
        let trans = Rectangle{
            width: self.height,
            height: self.width
        };
        trans
    }

    // This is an associated function that runs on a Struct name
    fn return_42() -> u32 {
        42
    }

    fn square(size: u32) -> Rectangle {
        Rectangle{ width: size, height: size }
    }
}


fn main() {

    let rect1 = Rectangle{width: 30, height: 50};
    let rect2 = Rectangle{width: 10, height: 40};
    let rect3 = Rectangle{width: 60, height: 45};

    //rect1.double();

    //let rect2 = rect1.transpose();


    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    //println!("rect2 is {:#?}",rect2)
    println!("What is the answer? {}", Rectangle::return_42())
}

