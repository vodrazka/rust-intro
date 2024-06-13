fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (int_param, bool_param) = pair;
    (bool_param, int_param)
}

pub fn main() {
    println!("Tuples code starting here");
    println!("Example on how to reverse a tuple");

    let example_tuple_one = (33, false);
    let reveresd_pair = reverse((example_tuple_one.0, example_tuple_one.1));
    println!("{:?}", reveresd_pair)
}
