fn command(
    text: &str,
    child_command: &mut Option<std::process::Command>,
    child_process: &mut Option<std::process::Child>,
) -> std::process::Command {
    if let Some(mut command) = child_command.take() {
        command.arg(text);
        command
    } else {
        let mut child_command = std::process::Command::new(text);
        if let Some(mut process) = child_process.take() {
            if let Some(stdout) = process.stdout.take() {
                child_command.stdin(stdout);
            }
        }
        child_command
    }
}

pub fn execute<'a, T>(tokens: &mut std::iter::Peekable<T>) -> Result<(), Box<dyn std::error::Error>>
where
    T: Iterator<Item = Result<crate::parser::Token<'a>, &'static str>>,
{
    let mut child_command: Option<std::process::Command> = None;
    let mut child_process: Option<std::process::Child> = None;
    //let mut previous_token = None;
    for token in tokens {
        let token = token?;
        match token {
            crate::parser::Token::Text(text) => {
                child_command = Some(command(text, &mut child_command, &mut child_process));
            }
            crate::parser::Token::Pipe => {
                if child_process.is_some() {
                    return Err("Unexpected additional pipe".into());
                }
                if let Some(mut child_command) = child_command.take() {
                    child_command.stdout(std::process::Stdio::piped());
                    let process = child_command.spawn()?;
                    child_process = Some(process)
                }
            }
            crate::parser::Token::Redirect => {}

            _ => return Err("Unhandled token".into()),
        }
    }
    if let Some(mut remaining_child) = child_command {
        remaining_child.spawn()?;
    }

    Ok(())
}
