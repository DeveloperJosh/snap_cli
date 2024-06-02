use snap_cli::{app::App, arg::Arg, command::Command};

fn main() {
    let app = App::new("cli")
        .version("1.0.0")
        .author("Blue")
        .about("A simple CLI app")
        .arg(Arg::new("verbose").about("Enable verbose mode").is_flag(true))
        .command(
            Command::new("hello")
                .about("Prints hello world")
                .execute(|matches| {
                    if matches.is_present("verbose") {
                        println!("Hello, world!");
                    } else {
                        println!("Hello!");
                    }
                }),
        )
        .command(
            Command::new("bye")
                .about("Prints goodbye world")
                .execute(|matches| {
                    if matches.is_present("verbose") {
                        println!("Goodbye, world!");
                    } else {
                        println!("Goodbye!");
                    }
                }),
        );

    // Usage 
    // cli hello --verbose
    // output: Hello, world!
    // cli bye
    // output: Goodbye!

    let _matches = app.get_matches();
}