pub struct Tokens<'a> {
    text: &'a str,
}

enum State {
    Unquoted,
    SingleQuotes,
    DoubleQuotes,
}

impl<'a> Iterator for Tokens<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        let mut end = 0;
        let mut state = State::Unquoted;

        let mut chars = self.text.chars();
        while let Some(c) = chars.next() {
            match c {
                ' ' => {
                    if end == 0 {
                        self.text = &self.text[1..];
                    } else {
                        break;
                    }
                }
                '\'' => match state {
                    State::SingleQuotes => break,
                    _ => state = State::SingleQuotes,
                },
                '"' => match state {
                    State::DoubleQuotes => break,
                    _ => state = State::DoubleQuotes,
                },

                _ => end += 1,
            }
        }
        if end == 0 {
            None
        } else {
            let res = Some(&self.text[..end]);
            self.text = &self.text[end..];
            res
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
