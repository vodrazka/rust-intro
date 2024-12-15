pub fn main() {
    demonstrate_mutability();
    demonstrate_shadowing();
    demonstrate_mutability_in_functions();
}

fn demonstrate_mutability_in_functions() {
    println!("You can pass variables by value");
    let a = 5;
    let b = increment_by_value(a); // passing a by value by default
    println!("a = {}, b = {}", a, b);

    println!("You can pass variables by value");
    let mut c = 5;
    println!("c = {}", c);
    let mut d = increment_by_reference(&mut c); // passing a by reference
    println!("c = {}, d = {}", c, d);
    d = increment_by_value(d);
    println!("c = {}, d = {}", c, d);
}

fn increment_by_value(mut num: i32) -> i32 {
    num += 1;
    num
}

fn increment_by_reference(num: &mut i32) -> i32 {
    *num += 1;
    *num
}

fn demonstrate_mutability() {
    println!("Module \"binding_and_mutability\" loaded");

    println!("By default variables are immuatable");
    let a = 5;
    // a = 6; // This will cause a compilation error
    println!("a = {}", a);

    println!("But you can make them mutable by adding the 'mut' keyword");
    let mut b = 5;
    println!("b = {}", b);
    b = 6;
    println!("b = {}", b);
}

fn demonstrate_shadowing() {
    println!("You can shadow a variable by using the same name");
    let a = 5;
    println!("a = {}", a);
    let a = a + 1;
    println!("a = {}", a);

    println!("You can also shadow a variable by using the same name in a different scope");
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
    println!("scope 0 a = {}", a);
}
