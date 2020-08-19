use std::str::FromStr;

type LexerResult<T> = Result<T, String>;

pub struct Lexer<'a> {
    str: &'a str,
    pos: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(str: &str) -> Lexer {
        Lexer { str, pos: 0 }
    }

    pub fn read_number<T: FromStr>(&mut self) -> LexerResult<T> {
        let str = self.read_while(|c| c.is_ascii_digit());
        str.parse::<T>().map_err(|_| format!("not a number '{}'", str))
    }

    pub fn expect(&mut self, str: &str) -> LexerResult<()> {
        if self.optional(str) {
            Ok(())
        } else {
            Err(format!("Expected '{}', but did not see", str))
        }
    }

    pub fn optional(&mut self, str: &str) -> bool {
        if self.str[self.pos..].starts_with(str) {
            self.pos += str.len();
            true
        } else {
            false
        }
    }

    pub fn read_while<P: Fn(char) -> bool>(&mut self, predicate: P) -> &'a str {
        let start = self.pos;

        while let Some(c) = self.str[self.pos..].chars().next() {
            if predicate(c) {
                self.pos += c.len_utf8();
            } else {
                break;
            }
        }

        &self.str[start..self.pos]
    }
}
