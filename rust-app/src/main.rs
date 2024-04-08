#![deny(clippy::all)]

const DEFAUL_PORT: u16 = 3000;

fn greet(name: &String) {
    // &String - ссылка на значение. Значение НЕ МОЖЕТ быть изменено
    println!("Hello, {}!", name);
}

fn clear_string(value: &mut String) {
    // &mut String - ссылка на значение. Значение по ссылке МОЖЕТ быть изменено.
    value.clear();
}

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

    // Ownership
    // https://doc.rust-lang.ru/book/ch04-01-what-is-ownership.html
    // Владение — это набор правил, определяющих, как программа на языке Rust управляет памятью.
    // Основная цель владения — управление данными в heap.
    // Правила :
    //   1. У каждого значения в Rust есть владелец,
    //   2. У значения может быть только один владелец в один момент времени,
    //   3. Когда владелец покидает область видимости, значение удаляется.
    let s1 = String::from("Hello");
    let s2 = &s1;
    greet(&s1);
    greet(s2);

    let mut name1 = String::from("John");
    clear_string(&mut name1);
}
