fn main() {
    let fahrenheit = 100.0;

    println!("{fahrenheit} in Fahrenheit is {} in Celsius", fahrenheit_to_celsius(fahrenheit));


    println!("Fibonacci series: ");
    for i in 1..10 {
        print!("{} ", fibonacci(i));
    }
    println!("...");

    print_song();
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) / 1.8
}

fn fibonacci(n: i32) -> i64 {
    let mut n1: i64 = 1;
    let mut n2: i64 = 1;

    for _ in 2..n {
        (n1, n2) = (n2, n1 + n2);
    }
    return n2;
}

fn print_song() {
    const DAYS: usize = 12;
    let days: [&str; DAYS] = ["first", "second", "third", "fourth", "fifth", "sixth",
        "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];

    let lines: [&str; DAYS] = [
        "Twelve drummers drumming,",
        "Eleven pipers piping,",
        "Ten lords a-leaping,",
        "Nine ladies dancing,",
        "Eight maids a-milking,",
        "Seven swans a-swimming,",
        "Six geese a-laying,",
        "Five golden rings,",
        "Four calling birds,",
        "Three French hens,",
        "Two turtle doves,",
        "partridge in a pear tree!"
    ];

    for i in 0..DAYS {
        println!("On the {} day of Christmas,", days[i]);
        println!("my true love sent to me");

        for j in DAYS - i - 1..DAYS {
            if j == DAYS - 1 {
                if i > 0 {
                    print!("And a ");
                } else {
                    print!("A ");
                }
            }
            println!("{}", lines[j]);
        }
        println!();
    }
}