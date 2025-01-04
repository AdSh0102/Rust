struct Rectangle{
    width: u32,
    height: u32,
}

impl Rectangle{
    fn area(&self) -> u32 {
        return self.height * self.width;
    }
    fn can_hold(&self, rect1: &Rectangle) -> bool {
        self.height > rect1.height && self.width > rect1.width
    }
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
    // let rect2 = Rectangle {
    //     width: 10,
    //     height: 40,
    // };
    // let rect3 = Rectangle {
    //     width: 60,
    //     height: 45,
    // };

    // println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    
    let sq = Rectangle::square(50);
    
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&sq));

}