mod shapes;

use crate::shapes::{rect::Rect, circle::Circle, area::Area};

fn main(){
    let rect = Rect::default();
    let circle = Circle {
        x: 0.0,
        y: 0.0,
        radius: 10.0,
    };
    println!("{}", circle.area());
    println!("{}", rect);
    println!("{}", rect.area());
}
