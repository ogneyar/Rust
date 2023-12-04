fn main() {
    let mut _integer: i8 = 32; // i8, i16, i32, i64, i128, isize 
    let _float: f32 = 3.14; // f32, f64
    _integer = 31;
    let string1 = String::from("Alice");
    let string2: &str = "Bob";

    // Вывести текст в консоль
    println!("Hello World!");
    // ещё
    print!("В январе ");
    // `{}` это заполнители для аргументов, которые будут строками
    println!("{} день", _integer);
    // Позиционные аргументы могут быть повторно использованы по шаблону
    println!("{0}, this is {1}. {1}, this is {0}", string1, string2);
    // Аргументы можно называть
    println!("{subject} {verb} {predicate}",
             predicate="over the lazy dog",
             subject="the quick brown fox",
             verb="jumps");
}