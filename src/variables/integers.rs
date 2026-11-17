pub fn main() {
    // Raw string `r#"..."#` lets us use quotes without escaping
    // PL: Surowy łańcuch `r#"..."#` pozwala używać cudzysłowów bez escapowania
    println!(r#"Module "integers" loaded"#);
    // Integer addition
    // PL: Dodawanie liczb całkowitych
    println!("1 + 2 = {}", 1u32 + 2);

    // Integer subtraction
    // PL: Odejmowanie liczb całkowitych
    println!("1 - 2 = {}", 1i32 - 2);
    // TODO ^ Try changing `1i32` to `1u32` to see why the type is important
    // PL: TODO ^ Spróbuj zmienić `1i32` na `1u32`, aby zobaczyć, dlaczego typ jest ważny

    // Scientific notation
    // PL: Notacja naukowa
    println!("1e4 is {}, -2.5e-3 is {}", 1e4, -2.5e-3);

    // Short-circuiting boolean logic
    // PL: Logika boolowska ze skróconym wartościowaniem
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    // Bitwise operations
    // PL: Operacje bitowe
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    // Use underscores to improve readability!
    // PL: Używaj podkreśleń, aby poprawić czytelność!
    println!("One million is written as {}", 1_000_000u32);
}
