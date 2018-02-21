macro_rules! create_function {
    // This macro takes an argument of desinator `ident` and
    // creates a function name `$func_name`
    // The `ident` desinator is used for variable/function names
    ($func_name:ident) => (
        fn $func_name() {
            println!("You called {:?}()", stringify!($func_name));
        }
    )
}

create_function!(foo);
create_function!(bar);

macro_rules! print_result {
    // This macro takes an expression of type `expr` and prints
    // it as a string along with its result.
    // The `expr` designator is used for expression.
    ($expression:expr) => (
        println!("{:?} = {:?}", stringify!($expression), $expression);
    )
}

fn main() {
    foo();
    bar();

    print_result!(1u32 + 1);

    //Recall that blocks are expressions too!
    print_result!({
        let x = 1u32;
        x * x + 2 * x - 1
    });
}
