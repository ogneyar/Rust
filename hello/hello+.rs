
fn main() {
    print!("January has ");
    // `{}` это заполнители для аргументов, которые будут строками
    println!("{} days", 31);
    // Позиционные аргументы могут быть повторно использованы по шаблону
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
    // Аргументы можно называть
    println!("{subject} {verb} {predicate}",
             predicate="over the lazy dog",
             subject="the quick brown fox",
             verb="jumps");
}
