use std::fmt::Display;

fn print_debug_generic<T: std::fmt::Debug>(input: T) {
    println!("{:?}", input);
}

fn print_debug_where<T>(input: T)
where
    T: std::fmt::Debug,
{
    println!("{:?}", input);
}

fn print_debug_impl(input: impl std::fmt::Debug) {
    println!("{:?}", input);
}

trait Formatter {
    fn format<T: Display>(&self, value: T) -> String;
}

struct SimpleFormatter;

impl Formatter for SimpleFormatter {
    fn format<T: Display>(&self, value: T) -> String {
        format!("Value {}", value)
    }
}

fn apply_format<F>(formatter: F) -> impl for<'a> Fn(&'a str) -> String
where
    F: Formatter,
{
    move |s| formatter.format(s)
}

#[cfg(test)]
mod tests {
    use crate::learn_rust::higher_ranked_trait_bounds::{
        apply_format, print_debug_generic, print_debug_where, SimpleFormatter,
    };

    #[test]
    fn trait_bounds() {
        print_debug_generic("hello generic");
        print_debug_where("hello where");
        print_debug_where("hello impl");
    }

    #[test]
    fn formatter() {
        let formatter = SimpleFormatter;
        let format_fn = apply_format(formatter);
        let s1 = "Hello";
        let s2 = String::from("World");

        println!("{}", format_fn(s1));
        println!("{}", format_fn(&s2));

        {
            let s3: String = String::from("Lifetime");
            println!("{}", format_fn(&s3));
        }
    }
}
