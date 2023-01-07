use peeking_take_while::PeekableExt;

//fn extract_token(tokens: &mut std::iter::Peekable<T>) ->

pub fn execute<'a, T>(tokens: &mut std::iter::Peekable<T>) -> Result<(), Box<dyn std::error::Error>>
where
    T: Iterator<Item = Result<crate::parser::Token<'a>, &'static str>>,
{
    let mut child_stdout: Option<std::process::ChildStdout> = None;
    loop {
        let mut arguments = tokens
            .by_ref()
            .peeking_take_while(|t| matches!(t, Ok(crate::parser::Token::Text(_))))
            .flatten()
            .map(|tok| match tok {
                crate::parser::Token::Text(text) => text,
                _ => panic!(),
            });
        if let Some(program) = arguments.next() {
            let mut process = std::process::Command::new(program);
            process.args(arguments);
            let stdin = if let Some(child_stdout) = child_stdout {
                child_stdout.into()
            } else {
                std::process::Stdio::inherit()
            };
            let stdout = if matches!(tokens.peek(), Some(Ok(crate::parser::Token::Pipe))) {
                tokens.next().unwrap()?;
                std::process::Stdio::piped()
            } else {
                std::process::Stdio::inherit()
            };
            let spawned = process.stdin(stdin).stdout(stdout).spawn();
            child_stdout = spawned?.stdout.take();
        } else {
            break;
        }
    }

    Ok(())
}
