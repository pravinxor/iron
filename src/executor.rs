fn command<S>(
    text: S,
    child_command: &mut Option<std::process::Command>,
    child_process: &mut Option<std::process::Child>,
) -> std::process::Command
where
    S: AsRef<std::ffi::OsStr>,
{
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

fn pipe(child_command: &mut std::process::Command) -> Result<std::process::Child, std::io::Error> {
    child_command.stdout(std::process::Stdio::piped());
    child_command.spawn()
}

fn redirect<P>(
    path: P,
    command: &mut std::process::Command,
) -> Result<std::process::Child, std::io::Error>
where
    P: AsRef<std::path::Path>,
{
    let file = std::fs::File::create(path).unwrap();
    command.stdout(file);
    command.spawn()
}

pub fn execute<'a, T>(tokens: &mut std::iter::Peekable<T>) -> Result<(), Box<dyn std::error::Error>>
where
    T: Iterator<Item = Result<crate::parser::Token<'a>, &'static str>>,
{
    let mut child_command: Option<std::process::Command> = None;
    let mut child_process: Option<std::process::Child> = None;
    let mut previous_token = None;
    for token in tokens {
        let token = token?;
        match token {
            crate::parser::Token::Text(text) => {
                if let Some(previous_token) = previous_token.take() {
                    match previous_token {
                        crate::parser::Token::Redirect => {
                            if let Some(mut command) = child_command.take() {
                                child_process = Some(redirect(text, &mut command)?);
                            } else {
                                return Err("No output to redirect".into());
                            }
                        }
                        _ => {}
                    }
                } else {
                    child_command = Some(command(text, &mut child_command, &mut child_process));
                }
            }
            crate::parser::Token::Pipe => {
                if child_process.is_some() {
                    return Err("Unexpected additional pipe".into());
                } else if let Some(mut command) = child_command.take() {
                    child_process = Some(pipe(&mut command)?);
                } else {
                    return Err("Cannot pipe a nonexistent process".into());
                }
            }
            crate::parser::Token::Background => {
                if let Some(mut command) = child_command.take() {
                    command.spawn()?;
                } else {
                    return Err("& specified but no process to fork".into());
                }
            }
            crate::parser::Token::Redirect => {
                previous_token = Some(token);
            }
            crate::parser::Token::Semicolon => {
                if let Some(mut remaining_child) = child_command.take() {
                    remaining_child.spawn()?.wait_with_output()?;
                }
            }

            _ => return Err("Unhandled token".into()),
        }
    }
    if let Some(mut remaining_child) = child_command {
        remaining_child.spawn()?.wait_with_output()?;
    }

    Ok(())
}
