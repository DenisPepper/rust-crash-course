#![deny(clippy::all)]

const DEFAUL_PORT: u16 = 3000;

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
    let port = DEFAUL_PORT;

    // tuples
    // https://doc.rust-lang.org/std/primitive.tuple.html
    let personal_data = (22, "John");
    let (my_age, my_name) = personal_data;
    let some_age = personal_data.0;
    let some_name = personal_data.1;
}
