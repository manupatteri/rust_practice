struct Rectangle {
	width: u32,
	height: u32,
}
impl Rectangle {
	fn area(&self) -> u32 {
		self.width * self.height
	}
	
}
impl Rectangle {
	fn can_hold(&self, other: &Rectangle ) -> bool {
		if self.width > other.width && self.height > other.height {
			true
		} else {
			false
		}
	}
	fn createRectangle(width:u32, height:u32) -> Self {
		Self {
			width:width,
			height:height,
		}	
	}
}
fn main() {
    println!("Hello, world!");
    let rect1 = Rectangle{width:50, height:100};
    let rect2 = Rectangle{width:40, height:90};
    println!("Area {} ", rect1.area());
    println!("rect1.can_hold(rect2) {} ", rect1.can_hold(&rect2));
    println!("is_senior, {} ", test_function("harry".to_string(), 64));
    let rect3 = Rectangle::createRectangle(30, 60);
    println!(" {} ", rect3.width);
}
fn test_function(name: String, age: u32) -> bool {
	if  age > 60 {
		return true;
	} else {
		return false;
	}
}
