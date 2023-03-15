fn foo() {
    let sum = add(&[1, 2, 3, 4]);
    println!("sum = {}", sum);
}

special_fn!(add, +, 4, i32, 0);

#[macro_export]
macro_rules! special_fn {
    ($name:ident, $op:tt, $n:literal, $t:ty, $default:expr ) => {
        fn $name(values: &[$t; $n]) -> $t {

            let mut result = $default;
            for i in 0..$n{
                result = result + values[i];
            }
            result
        }
    };
}
