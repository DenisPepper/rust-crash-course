#![deny(clippy::all)]

fn main() {
    // переменные нужно задавать в snake_case
    // почти всегда rust определит тип данных переменной на основе её значения
    // let - не позволит переопределить значение идентификатора, кроме случаев затенения
    // let mut - значение - можно переопределить, тип - нельзя
    let name = "Denis";
    println!("My name is {}", name);

    let mut _age: u8 = 30;
    //let mut _age = 30u8; - тип можно задать после значения
    let _population = 61_000_000;
    let _rgb = 0xFF0000;

    let size1 = 5.2f32;
    let size2 = 9.1f32;
    let _total_size = size1 + size2;
}
