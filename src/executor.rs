use peeking_take_while::PeekableExt;

//fn extract_token(tokens: &mut std::iter::Peekable<T>) ->

pub fn execute<'a, T>(tokens: &mut std::iter::Peekable<T>) -> Result<(), Box<dyn std::error::Error>>
where
    T: Iterator<Item = Result<crate::parser::Token<'a>, &'static str>>,
{
    let mut child = None;
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
            let res = std::process::Command::new(program).args(arguments).spawn();
            child = Some(res?);
        } else {
            break;
        }
    }

    Ok(())
}
