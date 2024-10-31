# Programming a Guessing Game

we will learn about:
- `let` keyword
- `match`
- methods
- associated functions
- external crates
- and more!

## code

```rust
use std::io;
fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    println!("You guessed: {}", guess);
}

```

- `use` keyword to bring a library into scope
- `::` syntax in the `::new` line indicates that new is an associated function of the String type.
- `&` indicates that this argument is a reference, which gives you a way to let multiple parts of your code access one piece of data without needing to copy that data into memory multiple times. References are a complex feature, and one of Rust’s major advantages is how safe and easy it is to use references.
- `mut` makes a variable mutable, and Rust requires variables to be mutable explicitly.
- `expect` method is one of the ways you can handle errors. The `expect` method call will crash your program and display the message that you passed as an argument to `expect`.
Result is an enumeration, often called an enum, which is a type that can be in one of multiple possible states. We call each possible state a variant.
- Result’s variants are Ok and Err. The Ok variant indicates the operation was successful, and inside Ok is the successfully generated value. The Err variant means the operation failed, and Err contains information about how or why the operation failed.
- Values of the Result type, like values of any type, have methods defined on them. An instance of Result has an expect method that you can call. If this instance of Result is an Err value, expect will cause the program to crash and display the message that you passed as an argument to expect.
- If the read_line method returns an Err, it would likely be the result of an error coming from the underlying operating system. If this instance of Result is an Ok value, expect will take the return value that Ok is holding and return just that value to you so you can use it. In this case, that value is the number of bytes in the user’s input.

### cargo.lock

- When you build a project for the first time, Cargo figures out all the versions of the dependencies that fit the criteria and then writes them to the Cargo.lock file. When you build your project in the future, Cargo will see that the Cargo.lock file exists and will use the versions specified there rather than doing all the work of figuring out versions again. This lets you have a reproducible build automatically. In other words, your project will remain at 0.8.5 until you explicitly upgrade, thanks to the Cargo.lock file. Because the Cargo.lock file is important for reproducible builds, it’s often checked into source control with the rest of the code in your project.
- When you do want to update a crate, Cargo provides the command update, which will ignore the Cargo.lock file and figure out all the latest versions that fit your specifications in Cargo.toml. Cargo will then write those versions to the Cargo.lock file. In this case, Cargo will only look for versions greater than 0.8.5 and less than 0.9.0. If the rand crate has released the two new versions 0.8.6 and 0.9.0, you would see the following if you ran cargo update:`
```shell
$ cargo update
    Updating cartes.io index
    Updating rand v0.8.5 -> v0.8.6

```
Cargo ignores the 0.9.0 release. At this point, you would also notice a change in your Cargo.lock file noting that the version of the rand crate you are now using is 0.8.6. To use rand version 0.9.0 or any version in the 0.9.x series, you’d have to update the Cargo.toml file to look like this instead:
```toml
[dependencies]
rand = "0.9"
```

The next time you run cargo build, Cargo will update the registry of crates available and reevaluate your rand requirements according to the new version you have specified.char

## guessing game v3

- `rand` crate
- Ordering enum
- match expression

```rust
use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("Guess the number!");
    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }
        };
        println!("You guessed: {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

```

- use statement, bringing a type called std::cmp::Ordering into scope from the standard library. The Ordering type is another enum and has the variants Less, Greater, and Equal. These are the three outcomes that are possible when you compare two values.
-  We use a match expression to decide what to do next based on which variant of Ordering was returned from the call to cmp with the values in guess and secret_number.
- A match expression is made up of arms. An arm consists of a pattern to match against, and the code that should be run if the value given to match fits that arm’s pattern. 
- 