mod utils;

fn main() {
    // The statements here will be executed when the compiled binary is called

    // Print text to the console
    println!("Hello Dot!");

    utils::logging::trace("Blah2 invoked")
}