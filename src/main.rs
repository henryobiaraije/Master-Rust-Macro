fn main() {
    let sizes = Vec::from([1, 2, 3, 4, 5]);
    let sizes2 = [1, 2, 3, 4, 5].to_vec();
    let sizes3 = vec![1, 2, 3, 4, 5];

    let sizes4 = my_vec!(1, 2, 3, 4, 5);
    let sizes42 = my_vec!(1.0, 2.0, 3.0, 4.0, 5.0);
    let sizes5: Vec<i32> = my_vec!();

    let mut sizes6 = Vec::new();
    sizes6.push(1);
    sizes6.push(2);
    sizes6.push(3);
    sizes6.push(4);
    sizes6.push(5);

}



#[macro_export]
macro_rules! my_vec  {
    ($($x:expr),*) => {
        {
            let mut new_vec = Vec::new();
            $(
                new_vec.push($x);
            )*
            new_vec
        }
    }
}
