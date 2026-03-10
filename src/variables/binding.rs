#![allow(dead_code)]
fn main() {
    let mut x = 1; // Додаємо mut, щоб x можна було змінити
    x += 2;

    assert_eq!(x, 3);
    println!("Розділ 3 (Mutability) пройдено успішно!");
}