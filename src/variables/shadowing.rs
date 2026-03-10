fn main() {
    let x: i32 = 5;
    {
        let x = 12; // Це затінення (shadowing): нова x ховає стару
        assert_eq!(x, 12);
    }
    assert_eq!(x, 5); // Тут знову бачимо першу x

    let x = 42;
    println!("Shadowing Success: x = {}", x);
}
