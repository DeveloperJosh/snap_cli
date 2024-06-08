use crate::arg::Arg;
use crate::command::{Command, Matches};
use crate::app::AppError;

/// Parses command line arguments and returns a `Matches` instance containing the parsed arguments.
///
/// # Arguments
///
/// * `app` - The `App` instance containing command definitions.
///
/// # Returns
///
/// * An instance of `Matches` containing the parsed arguments or an `AppError`.
pub fn parse(app: &crate::app::App) -> Result<Matches, AppError> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() <= 1 {
        app.print_help();
        return Err(AppError::NoArguments);
    }

    let mut matches = Matches::new();

    // Parse the command and its arguments
    if let Some(command_name) = args.get(1) {
        if let Some(command) = app.commands.get(command_name.as_str()) {
            let command_args = args[2..].to_vec();
            parse_args(&command.args, &command_args, &mut matches)?;
            if let Some(execute) = &command.execute {
                execute(&matches);
            }
            Ok(matches)
        } else {
            Err(AppError::UnknownCommand(command_name.clone()))
        }
    } else {
        Err(AppError::NoArguments)
    }
}

/// Parses command line arguments for the specified command.
///
/// # Arguments
///
/// * `arg_definitions` - The list of argument definitions.
/// * `input_args` - The list of command line arguments.
/// * `matches` - The `Matches` instance to store parsed arguments.
///
/// # Returns
///
/// * `Ok(())` if parsing is successful, otherwise an `AppError`.
fn parse_args(arg_definitions: &[crate::arg::Arg], input_args: &[String], matches: &mut Matches) -> Result<(), AppError> {
    let mut i = 0;
    let mut positional_index = 0;
    let positional_args: Vec<&Arg> = arg_definitions.iter().filter(|a| a.is_positional).collect();

    while i < input_args.len() {
        let input = &input_args[i];

        if input.starts_with("--") {
            // Handle long flag or option
            let key = &input[2..];
            if let Some(arg_def) = arg_definitions.iter().find(|a| a.name == key) {
                if arg_def.is_flag {
                    matches.insert(key, "true".to_string());
                } else if let Some(value) = input_args.get(i + 1) {
                    matches.insert(key, value.clone());
                    i += 1;
                } else if let Some(default) = arg_def.default.clone() {
                    matches.insert(key, default.to_owned());
                } else {
                    return Err(AppError::MissingValue(key.to_string()));
                }
            } else {
                return Err(AppError::UnknownArgument(key.to_string()));
            }
        } else if input.starts_with('-') {
            // Handle short flag or option
            let short_key = &input[1..];
            if let Some(arg_def) = arg_definitions.iter().find(|a| a.short == Some(short_key)) {
                if arg_def.is_flag {
                    matches.insert(arg_def.name, "true".to_string());
                } else if let Some(value) = input_args.get(i + 1) {
                    matches.insert(arg_def.name, value.clone());
                    i += 1;
                } else if let Some(default) = arg_def.default.clone() {
                    matches.insert(arg_def.name, default.to_owned());
                } else {
                    return Err(AppError::MissingValue(arg_def.name.to_string()));
                }
            } else {
                return Err(AppError::UnknownArgument(short_key.to_string()));
            }
        } else {
            // Handle positional arguments
            if positional_index < positional_args.len() {
                let arg_def = positional_args[positional_index];
                matches.insert(arg_def.name, input.clone());
                positional_index += 1;
            } else {
                return Err(AppError::UnexpectedArgument(input.clone()));
            }
        }
        i += 1;
    }

    // Handle missing default values for positional arguments
    for arg_def in positional_args.iter() {
        if !matches.is_present(arg_def.name) {
            if let Some(default) = arg_def.default.clone() {
                matches.insert(arg_def.name, default.to_owned());
            }
        }
    }

    Ok(())
}
