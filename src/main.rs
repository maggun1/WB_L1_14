use std::any::type_name;

fn print_type_of<T>(value: &T)
where T: std::fmt::Debug + ?Sized {
    println!("{:?} is {}", value, type_name::<T>());
}

fn main() {
    print_type_of(&0);
    print_type_of(&0.1);
    print_type_of("str");
    print_type_of(&"String".to_string());
}
