// Tuples can be used as function arguments and return values
// PL: Krotki (tuple) mogą być argumentami funkcji i wartościami zwracanymi
fn reverse(pair: (i32, bool)) -> (bool, i32) {
    // `let` can be used to destructure a tuple into variables
    // PL: `let` pozwala rozpakować (destrukturyzować) krotkę na zmienne
    let (int_param, bool_param) = pair;
    // The last expression without `;` is the return value
    // PL: Ostatnie wyrażenie bez `;` jest wartością zwracaną
    (bool_param, int_param)
}

pub fn main() {
    println!("Tuples code starting here");
    println!("Example on how to reverse a tuple");

    let example_tuple_one = (33, false);
    // Tuple elements are accessed by index: `.0`, `.1`, ...
    // PL: Do elementów krotki odwołujemy się przez indeks: `.0`, `.1`, ...
    let reveresd_pair = reverse((example_tuple_one.0, example_tuple_one.1));
    // `{:?}` uses the Debug formatter, needed to print tuples
    // PL: `{:?}` używa formatowania Debug, potrzebnego do wypisania krotek
    println!("{:?}", reveresd_pair)
}
