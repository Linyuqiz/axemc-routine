use crate::callC::add;

#[test]
fn test() {
    println!("{} * 2 = {}", 4, unsafe { add(4, 11) });
}
