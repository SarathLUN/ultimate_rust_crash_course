// Silence some warnings that could distract from the exercise
#![allow(unused)]

fn main() {
    // 1. Write code that will print out the the value of maybe_fruit's wrapped string if it is a
    // `Some` variant, and does nothing if it is a `None` variant. Use an `if` expression with the
    // Option type's `is_some` and `unwrap` methods to implement the logic. Then run the code.

    let maybe_fruit: Option<&str> = Some("apple");
    // if ...

    // 2. Write a function that accepts an `Option<&str>` as an argument and prints out
    // "You passed in a {}" with the value of the string in the Some variant or does nothing if the
    // value is a None variant. Use an `if let` expression to get the value wrapped by the `Some`
    // variant.
    //
    // Call the function once for each of `maybe_plant` and `maybe_food`.
    //
    // Then run the code.

    let maybe_plant: Option<&str> = None;
    let maybe_food: Option<&str> = Some("cake");
    // your_function(...);
    // your_function(...);

    // 2.  Write a loop that passes each number in the `numbers` vector to the `do_math` function
    // and then checks the result using a `match` expression.
    //
    // The `do_math` function (see bottom of this file) takes an i32 as input and returns a
    // `Result` whose `Ok` variant wraps a new i32, or an `Err` variant that wraps a `String`
    // which is a message about the error which occurred.
    //
    // - If the result is an `Ok`, then print out "The result was {}", with the value of the
    //   wrapped number
    // - If the result is an `Err`, then print out the wrapped error message string

    let numbers = vec![0, 1, 2, 3];
    // for ...
    for number in numbers {
        match do_math(number) {
            Ok(x) => println!("Ok, the result was {x}"),
            Err(msg) => println!("{msg}"),
        }
    }

    // 3. Define an enum named `Snack` with the following variants:
    //
    // - Apple - which contains no data
    // - Cookies - which contains an unnamed tuple with a single u8
    // - Sandwich - which contains an unnamed struct with fields `lettuce` and `cheese`, both of
    //   type `bool`
    //
    // Then uncomment and run the code below

    let healthy_snack = Snack::Apple;
    let sugary_snack = Snack::Cookies(18);
    let lunch = Snack::Sandwich {
        lettuce: false,
        cheese: true,
    };
    if let Snack::Apple = healthy_snack {
        println!("The healthy snack is an apple.");
    }
    if let Snack::Cookies(num_cookies) = sugary_snack {
        println!("The sugary snack is {} cookies", num_cookies);
    }
    if let Snack::Sandwich { lettuce, cheese } = lunch {
        let lettuce_msg = if lettuce { "does" } else { "does not" };
        let cheese_msg = if cheese { "does" } else { "does not" };
        println!(
            "The sandwich {} have lettuce and {} have cheese.",
            lettuce_msg, cheese_msg
        );
    }

    // 4. Create an `impl` block for Snack and implement a method named `price` which takes
    // ownership of a snack and returns a u8 representing the price of the snack according to the
    // following rules:
    //
    // - An apple's price is 5
    // - A cookie's price is 2 times the number of cookies
    // - A sandwich's price starts at 10, plus 1 if lettuce is true, plus 2 if cheese is true
    //
    // Hint: The signature of the method is `fn price(self) -> u8`
    //
    // Then uncomment and run the code below. You should see three lines ending with the costs of
    // $5, $36, and $13.

    println!("An apple costs ${}", healthy_snack.price());
    if let Snack::Cookies(number) = sugary_snack {
        println!("{} cookies costs ${}", number, sugary_snack.price());
    }
    if let Snack::Sandwich { lettuce, cheese } = lunch {
        let lettuce_message = if lettuce { " with lettuce" } else { "" };
        let cheese_message = if cheese { " with cheese" } else { "" };
        println!(
            "A sandwich{}{} costs ${}",
            lettuce_message,
            cheese_message,
            lunch.price()
        );
    }

    // Challenge 1: Implement an `is_apple` method for Snack that return a bool. Return `true` if
    // the value is an `Apple` variant, and `false` otherwise. Then uncomment and run the code
    // below.

    let snacks = vec![Snack::Apple, Snack::Cookies(5), Snack::Apple];
    for (index, snack) in snacks.iter().enumerate() {
        if snack.is_apple() {
            println!("Snack {} is an apple.", index)
        } else {
            println!("Snack {} is NOT an apple.", index)
        }
    }
}

enum Snack {
    Apple,
    Cookies(u8),
    Sandwich { lettuce: bool, cheese: bool },
}

impl Snack {
    fn price(self) -> u8 {
        match self {
            Snack::Apple => 5,
            Snack::Cookies(number) => 2 * number,
            Snack::Sandwich { lettuce, cheese } => {
                let mut price = 10;
                if lettuce {
                    price += 1;
                }
                if cheese {
                    price += 2;
                }
                price
            }
        }
    }
    fn is_apple(&self) -> bool {
        if let Snack::Apple = self {
            true
        } else {
            false
        }
    }
}

fn do_math(x: i32) -> Result<i32, String> {
    if x == 1 {
        return Ok(100);
    }
    return Err(format!("I wanted the number 1 and you gave me a {} 🥺", x));
}