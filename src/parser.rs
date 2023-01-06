#[derive(Debug)]
pub enum Token<'a> {
    Text(&'a str),
    Semicolon,
}

pub struct Tokens<'a> {
    text: &'a str,
}

enum State {
    Unquoted,
    SingleQuotes,
    DoubleQuotes,
}

impl<'a> Iterator for Tokens<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.text = self.text.trim_start();
        if self.text.is_empty() {
            return None;
        }
        let mut length = 0;
        let mut state = State::Unquoted;
        for c in self.text.chars() {
            match state {
                State::Unquoted => match c {
                    '\'' => {
                        state = State::SingleQuotes;
                        self.text = &self.text[1..];
                    }
                    '"' => {
                        state = State::DoubleQuotes;
                        self.text = &self.text[1..];
                    }
                    ' ' => {
                        break;
                    }
                    ';' => {
                        if length == 0 {
                            length = 1;
                        }
                        break;
                    }
                    _ => length += 1,
                },
                State::SingleQuotes => {
                    if c == '\'' {
                        break;
                    } else {
                        length += 1;
                    }
                }
                State::DoubleQuotes => {
                    if c == '"' {
                        break;
                    } else {
                        length += 1;
                    }
                }
            }
        }
        dbg!(length);
        let res = &self.text[..length];
        self.text = &self.text[length..];
        match state {
            State::SingleQuotes | State::DoubleQuotes => self.text = &self.text[1..],
            _ => {}
        }
        match res {
            ";" => Some(Token::Semicolon),
            _ => Some(Token::Text(res)),
        }
    }
}

pub trait TokenParser {
    fn split_tokens(&self) -> Tokens;
}

impl TokenParser for str {
    fn split_tokens(&self) -> Tokens {
        Tokens { text: self }
    }
}
