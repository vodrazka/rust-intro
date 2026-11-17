pub fn main() {
    demonstrate_mutability();
    demonstrate_shadowing();
    demonstrate_mutability_in_functions();
}

// Shows the difference between passing by value and by mutable reference
// PL: Pokazuje różnicę między przekazywaniem przez wartość a przez referencję mutowalną
fn demonstrate_mutability_in_functions() {
    println!("You can pass variables by value");
    let a = 5;
    let b = increment_by_value(a); // passing a by value by default / PL: domyślnie przekazujemy a przez wartość
    // `a` is unchanged - the function received a copy
    // PL: `a` pozostaje bez zmian - funkcja otrzymała kopię
    println!("a = {}, b = {}", a, b);

    println!("You can pass variables by value");
    let mut c = 5;
    println!("c = {}", c);
    let mut d = increment_by_reference(&mut c); // passing a by reference / PL: przekazujemy przez referencję
    // `c` was modified inside the function through the reference
    // PL: `c` zostało zmodyfikowane wewnątrz funkcji poprzez referencję
    println!("c = {}, d = {}", c, d);
    d = increment_by_value(d);
    println!("c = {}, d = {}", c, d);
}

// `mut num` makes the local copy mutable - the caller's variable is not affected
// PL: `mut num` czyni lokalną kopię mutowalną - zmienna wywołującego nie jest zmieniana
fn increment_by_value(mut num: i32) -> i32 {
    num += 1;
    num
}

// `&mut i32` is a mutable reference - `*num` dereferences it to change the original value
// PL: `&mut i32` to referencja mutowalna - `*num` ją dereferencjonuje, aby zmienić oryginalną wartość
fn increment_by_reference(num: &mut i32) -> i32 {
    *num += 1;
    *num
}

fn demonstrate_mutability() {
    println!("Module \"binding_and_mutability\" loaded");

    println!("By default variables are immuatable");
    let a = 5;
    // a = 6; // This will cause a compilation error / PL: To spowoduje błąd kompilacji
    println!("a = {}", a);

    println!("But you can make them mutable by adding the 'mut' keyword");
    // `mut` allows the value to be changed later
    // PL: `mut` pozwala później zmienić wartość
    let mut b = 5;
    println!("b = {}", b);
    b = 6;
    println!("b = {}", b);
}

// Shadowing: a new `let` with the same name creates a new variable that hides the old one
// PL: Przesłanianie (shadowing): nowe `let` z tą samą nazwą tworzy nową zmienną, która zasłania poprzednią
fn demonstrate_shadowing() {
    println!("You can shadow a variable by using the same name");
    let a = 5;
    println!("a = {}", a);
    let a = a + 1;
    println!("a = {}", a);

    println!("You can also shadow a variable by using the same name in a different scope");
    // Shadowing inside a block lasts only until the end of that block
    // PL: Przesłanianie wewnątrz bloku obowiązuje tylko do końca tego bloku
    {
        let a = a + 1;
        println!("scope 1 a = {}", a);
        {
            if false {
                let a = a + 1;
            }
            println!("scope 2 a = {}", a);
        }
        println!("scope 1 a = {}", a);
    }
    // Back in the outer scope, the original `a` is visible again
    // PL: Po powrocie do zewnętrznego zakresu ponownie widoczne jest oryginalne `a`
    println!("scope 0 a = {}", a);
}
