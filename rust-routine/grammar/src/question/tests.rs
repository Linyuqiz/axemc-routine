use crate::question::{calculate_area, sum, Square, TrafficLight};

#[test]
fn test() {
    let light = TrafficLight::Green;
    println!("The {:?} light: {}s", light, light.time());

    let array = [11, 12, 3, 4, 5, 6, 7];
    if let Some(value) = sum(&array) {
        println!("The value is {}", value)
    } else {
        println!("The value is too big!")
    }

    let shape = Square { a: 11 };
    let value = calculate_area(shape);
    println!("The shape's area is {}", value)
}
