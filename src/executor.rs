pub fn execute<'a, T>(tokens: &mut std::iter::Peekable<T>) -> Result<(), Box<dyn std::error::Error>>
where
    T: Iterator<Item = Result<crate::parser::Token<'a>, &'static str>>,
{
    let mut child_command: Option<std::process::Command> = None;
    let mut child_process: Option<std::process::Child> = None;
    for token in tokens {
        let token = token?;
        match token {
            crate::parser::Token::Text(text) => {
                child_command = if let Some(mut command) = child_command.take() {
                    command.arg(text);
                    Some(command)
                } else {
                    let mut child_command = std::process::Command::new(text);
                    if let Some(mut process) = child_process.take() {
                        if let Some(stdout) = process.stdout.take() {
                            child_command.stdin(stdout);
                        }
                    }
                    Some(child_command)
                }
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
            _ => return Err("Unhandled token".into()),
        }
    }
    if let Some(mut remaining_child) = child_command {
        remaining_child.spawn()?;
    }

    Ok(())
}
