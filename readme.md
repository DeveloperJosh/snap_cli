# Snap

![Crates.io](https://img.shields.io/crates/v/snap_cli.svg)
![License](https://img.shields.io/crates/l/snap_cli.svg)

Snap is a lightweight and efficient command-line argument handler, inspired by `clap`.

## Features

- **Ease of Use**: Simple and intuitive API for defining command-line arguments.
- **Performance**: Optimized for fast argument parsing.
- **Customizability**: Flexible configuration options to tailor argument parsing to your needs.
- **Error Handling**: Comprehensive and user-friendly error messages.
- **Help Message Generation**: Automatic generation of help and usage messages.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
snap = "0.1.3"
```
Usage
Here's a simple example of how to use Snap:

```rust
use snap::App;

fn main() {
    let matches = App::new("myapp")
        .version("1.0")
        .author("Your Name <your@email.com>")
        .about("Does awesome things")
        .arg("<input> 'Sets the input file to use'")
        .arg("-v, --verbose 'Sets the level of verbosity'")
        .get_matches();

    if let Some(input) = matches.value_of("input") {
        println!("Using input file: {}", input);
    }

    if matches.is_present("verbose") {
        println!("Verbose mode is on");
    }
}
```
# Examples
More detailed examples can be found in the [examples](https://github.com/DeveloperJosh/snap_cli/tree/main/Examples) directory.

```rust
use snap::App;

fn main() {
    let matches = App::new("myapp")
        .version("1.0")
        .author("Your Name <your@email.com>")
        .about("Does awesome things")
        .arg("<input> 'Sets the input file to use'")
        .arg("-v, --verbose 'Sets the level of verbosity'")
        .get_matches();

    if let Some(input) = matches.value_of("input") {
        println!("Using input file: {}", input);
    }

    if matches.is_present("verbose") {
        println!("Verbose mode is on");
    }
}
```


# Contributing
Contributions are welcome! Feel free to open issues or submit pull requests.

# License
This project is licensed under the MIT License.