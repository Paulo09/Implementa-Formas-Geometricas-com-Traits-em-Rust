use std::fmt;

trait Shape {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
    fn draw(&self) -> String;
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }

    fn draw(&self) -> String {
        format!("Rectangle: width = {}, height = {}", self.width, self.height)
    }
}

struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius.powf(2.0)
    }

    fn perimeter(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius
    }

    fn draw(&self) -> String {
        format!("Circle: radius = {}", self.radius)
    }
}

fn main() {
    let rect = Rectangle { width: 5.0, height: 3.0 };
    let circle = Circle { radius: 2.0 };

    println!("Área do retângulo: {}", rect.area());
    println!("Perímetro do retângulo: {}", rect.perimeter());
    println!("{}", rect.draw());

    println!("Área do círculo: {}", circle.area());
    println!("Perímetro do círculo: {}", circle.perimeter());
    println!("{}", circle.draw());
}
