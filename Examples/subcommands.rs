use snap_cli::{app::App, arg::Arg, command::Command};

/// Entry point of the CLI application.
fn main() {
    // Create a new instance of the application with its name, version, author, and description.
    let app = App::new("cli")
        .version("1.0.0")
        .author("Blue")
        .about("A simple CLI app")
        // Add a global argument for enabling verbose mode.
        .arg(Arg::new("verbose").about("Enable verbose mode").is_flag(true))
        // Add a command 'echo' which prints the provided text.
        .command(
            Command::new("echo")
                .about("Prints what you say")
                .execute(|matches| {
                    // Retrieve the value of the 'text' argument or use the default message.
                    let text = matches.value_of_str("text", "No text provided");
                    println!("{}", text);
                }),
        );

    // Define the 'main' command with a subcommand 'sub'.
    let main_command = Command::new("main")
        .about("The main command")
        // Add an argument 'text' to the 'main' command with a default value.
        .arg(Arg::new("text").about("The text to print").default("Hello, world!"))
        // Add the 'sub' command with arguments 'a' and 'b'.
        .subcommand(
            Command::new("sub")
                .about("A subcommand")
                .arg(Arg::new("a").about("add a").default("0"))
                .arg(Arg::new("b").about("add b").default("0"))
                .execute(|matches| {
                    // Retrieve the values of 'a' and 'b', parse them as integers, and print their sum.
                    let a = matches.value_of_int("a", 0);
                    let b = matches.value_of_int("b", 0);
                    println!("{}", a + b);
                }),
        );

    // Add the 'main' command to the app and parse the command line arguments.
    let _app = app.command(main_command).get_matches();
}
