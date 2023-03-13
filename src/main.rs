fn main() {
    let sum = add(&[1, 2, 3, 4]);
    let diff = diff(&[1, 2, 3, 4]);
    let product = mult(&[4, 2]);
    println!("sum = {}, diff = {}, product = {}", sum, diff, product);
}

// Generate a function call add using out custom macro.
special_fn!(add, +, 4, i32, 0);

// Generate a function call diff using out custom macro.
special_fn!(diff, -, 4, i32, 0);

// Generate a function call mult using out custom macro.
special_fn!(mult, *, 2, i32, 0);


// Definging out macro.
#[macro_export] // This makes our macro importable into other modules.
macro_rules! special_fn {

    // Define the patterns.
    ($name:ident, $op:tt, $n:literal, $t:ty, $default:expr ) => {
        fn $name(values: &[$t; $n]) -> $t {

            // Define the generated code here.
            if(values.len() < 1){
                return $default;
            }
            let mut result = values[0];
            for i in 1..$n {
                result = result $op values[i];
                println!("i = {}, result = {}, val = {}",i,result,values[i]);
            }
            result
        }
    };
}
