
mod demo;


use demo::Demo;
use gc2d::gc2d::Gc2d;

fn main() {
    Gc2d::new().run(
        Demo::default()
    ).unwrap();
    
}
