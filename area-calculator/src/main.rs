#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, another_rect: &Rectangle) -> bool {
        self.width > another_rect.width && self.height > another_rect.height
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height:43 };
    let rect2 = Rectangle { width: 10, height:83 };
    let rect3 = Rectangle { width: 15, height:23 };
    println!("rect1 is {:#?}", &rect1);
    println!("rect2 is {:#?}", &rect2);
    println!("rect3 is {:#?}", &rect3);

    println!(
        "The area1 of the rectangle is {} square pixels.",
        rect1.area()
    );
    println!(
        "The area2 of the rectangle is {} square pixels.",
        rect2.area()
    );
    println!(
        "The area3 of the rectangle is {} square pixels.",
        rect3.area()
    );

    println!(
        "Can rect1 hold rect2? {}",
        rect1.can_hold(&rect2)
    );
    println!(
        "Can rect1 hold rect3? {}",
        rect1.can_hold(&rect3)
    );
}

