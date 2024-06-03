use snap_cli::{app::App, arg::Arg, command::Command};

fn main() {
    let app = App::new("Git CLI")
        .version("1.0")
        .author("Blue <Blue@blue-dev.xyz>")
        .about("A simple Git CLI tool");

    let git_command = Command::new("git")
        .about("A version control system")
        .subcommand(
            Command::new("init")
                .about("Create an empty Git repository or reinitialize an existing one")
                .execute(|_matches| {
                    println!("Initialized empty Git repository in /path/to/repo");
                })
        );
    let _matches = app.command(git_command).get_matches();
}
