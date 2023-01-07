pub fn execute<'a, T>(tokens: &mut std::iter::Peekable<T>) -> Result<(), Box<dyn std::error::Error>>
where
    T: Iterator<Item = Result<crate::parser::Token<'a>, &'static str>>,
{
    let mut child: Option<std::process::Command> = None;
    for token in tokens {
        let token = token?;
        match token {
            crate::parser::Token::Text(text) => {
                child = if let Some(mut child_command) = child.take() {
                    child_command.arg(text);
                    Some(child_command)
                } else {
                    Some(std::process::Command::new(text))
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
