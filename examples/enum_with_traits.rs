trait Shape {
    fn area(&self) -> f64;
}

enum ShapeType {
    Circle(f64),
    Rectangle { w: f64, h: f64 },
}

impl Shape for ShapeType {
    fn area(&self) -> f64 {
        match self {
            ShapeType::Circle(r) => 3.14 * r * r,
            ShapeType::Rectangle { w, h } => w * h,
        }
    }
}

fn main() {
    let s = ShapeType::Circle(5.0);
    println!("{}", s.area());
}