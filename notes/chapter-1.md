## Anatomy of a Rust Program

```rust
fn main() {

}
```
- `main` it is always the first code that runs in every Rust executable Rust program.
- `main` that has no parameters and returns nothing.
- If there were parameters,they would go inside the parentheses `()`
- Rust requires curly brackets around all function bodies. It’s good style to place the opening curly bracket on the same line as the function declaration, adding one space in between.

```rust
    println!("Hello world!");
```
- First, Rust style is to indent with four spaces, not a tab.
- println! calls a Rust macro. If it had called a function instead, it would be entered as println (without the !). (we will discuss Rust macros in more detail in Chapter 19)
- At now just remember `!` means that you’re calling a macro instead of a normal function.
- `;` indicates that this expression is over and the next one is ready to begin.

## Compiling and Running a Rust Program

### rustc

```rust
rustc main.rs
```

- Rust not like Python or Ruby, it’s a compiled language, which means you need to compile your code before you can run it.

### cargo
Cargo is Rust’s build system and package manager. Most Rustaceans use Cargo to manage their Rust projects because Cargo handles a lot of tasks for you, such as building your code, downloading the libraries your code depends on, and building those libraries.

Cargo expects your source files to live inside the src directory. The top-level project directory is just for README files, license information, configuration files, and anything else not related to your code. Using Cargo helps you organize your projects. There’s a place for everything, and everything is in its place.
Cargo.lock. This file keeps track of the exact versions of dependencies in your project.
Cargo figured out that the files hadn’t changed, so it didn’t rebuild but just ran the binary. 
- `cargo check` - checks your code to make sure it compiles but doesn’t produce an executable
- cargo check is much faster than cargo build because it skips the step of producing an executable. 

#### summary

- We can create a project using cargo new.
- We can build a project using cargo build.
- We can build and run a project in one step using cargo run.
- We can build a project without producing a binary to check for errors using cargo check.
- Instead of saving the result of the build in the same directory as our code, Cargo stores it in the target/debug directory.

#### build for release

```rust
cargo build --release
```
This command will create an executable in target/release instead of target/debug. The optimizations make your Rust code run faster, but turning them on lengthens the time it takes for your program to compile.

