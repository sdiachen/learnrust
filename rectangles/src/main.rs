fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "Area of the rectangle is {}",
        area(width1, height1)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}