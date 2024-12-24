fn main() {
    let width1 = 30;
    let height1 = 50;
    println!(
        "The area of the rectangle is {} square pixels.",
        area(height1, width1)
    );
}

fn area(height: u32, width: u32) -> u32 {
    height * width
}