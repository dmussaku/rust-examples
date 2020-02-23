#[derive(Debug)]

struct Rectangle {
	length: u32,
	width: u32,
}

impl Rectangle {
	fn area(&self) -> u32 {
		self.length * self.width
	}
}

fn main() {
    let rectangle = Rectangle{length: 4, width: 6};

    println!("Area of rectangle is {}", rectangle.area());
}

