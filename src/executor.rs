pub fn execute<'a, T>(tokens: &mut std::iter::Peekable<T>) -> Result<(), Box<dyn std::error::Error>>
where
    T: Iterator<Item = Result<crate::parser::Token<'a>, &'static str>>,
{
    let mut child: Option<std::process::Command> = None;
    let mut child_stdout = None;
    for token in tokens {
        let token = token?;
        match token {
            crate::parser::Token::Text(text) => {
                child = if let Some(mut child_command) = child.take() {
                    child_command.arg(text);
                    Some(child_command)
                } else {
                    let mut child_command = std::process::Command::new(text);
                    if let Some(stdout) = child_stdout.take() {
                        child_command.stdin(stdout);
                    }
                    Some(child_command)
                }
            }
            crate::parser::Token::Pipe => {
                if child_stdout.is_some() {
                    return Err("Unexpected additional pipe".into());
                }
                if let Some(mut child_command) = child.take() {
                    child_command.stdout(std::process::Stdio::piped());
                    let child_process = child_command.spawn()?;
                    child_stdout = child_process.stdout
                }
            }
            _ => return Err("Unhandled token".into()),
        }
    }
    if let Some(mut remaining_child) = child {
        remaining_child.spawn()?;
    }

    Ok(())
}
