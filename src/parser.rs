enum Token<'a> {
    Command(&'a str),
    Argument(&'a str),
}

struct Tokens<'a> {
    text: &'a str,
}

impl<'a> Iterator for Tokens<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
