
#[derive(Debug, PartialEq)]
enum Token {
    Plus,
    Minus,
    Asterisk,
    Slash,
    Equals,
    ExclamationMark,
    At,
    SingleQuote,
    DoubleQuote,
    QuestionMark,
    LessThan,
    GreaterThan,
    LeftBrace,
    RightBrace,
    LeftParan,
    RightParan,
    LeftBracket,
    RightBracket,
    Dot,
    Integer(i32),
    Illegal,
    EOF,
}

struct Lexer<'a> {
    input: &'a str,
    position: usize,
}

impl<'a> Lexer<'a> {
    fn new(input: &'a str) -> Self {
        Lexer { input, position: 0 }
    }

    fn next_char(&self) -> Option<char> {
        self.input.chars().nth(self.position)
    }

    fn advance(&mut self) {
        self.position += 1;
    }

    fn parse_num(&mut self) -> Token {
        let mut digits = String::new();

        loop {
            let char = self.input
                .chars()
                .nth(self.position);
            
            if let None = char {
                break;
            }

            let c = char.unwrap();
            
            if !c.is_digit(10) {
                break;
            }

            digits.push(c);
            self.advance();
        }

        Token::Integer(digits.parse::<i32>().unwrap())
    }

    fn next_token(&mut self) -> Token {
        let char = self.input.chars().nth(self.position);

        if let Some(c) = char {
            return match c {
                '+' => {
                    self.advance();
                    Token::Plus
                },
                '-' => {
                    self.advance();
                    Token::Minus
                },
                '*' => {
                    self.advance();
                    Token::Asterisk
                },
                '/' => {
                    self.advance();
                    Token::Slash
                },
                '=' => {
                    self.advance();
                    Token::Equals
                },
                '!' => {
                    self.advance();
                    Token::ExclamationMark
                },
                '@' => {
                    self.advance();
                    Token::At
                },
                '\'' => {
                    self.advance();
                    Token::SingleQuote
                },
                '"' => {
                    self.advance();
                    Token::DoubleQuote
                },
                '?' => {
                    self.advance();
                    Token::QuestionMark
                },
                '<' => {
                    self.advance();
                    Token::LessThan
                },
                '>' => {
                    self.advance();
                    Token::GreaterThan
                },
                '{' => {
                    self.advance();
                    Token::LeftBrace
                },
                '}' => {
                    self.advance();
                    Token::RightBrace
                },
                '(' => {
                    self.advance();
                    Token::LeftParan
                },
                ')' => {
                    self.advance();
                    Token::RightParan
                },
                '[' => {
                    self.advance();
                    Token::LeftBracket
                },
                ']' => {
                    self.advance();
                    Token::RightBracket
                },
                '.' => {
                    self.advance();
                    Token::Dot
                },
                '0'..='9' => {
                    self.parse_num()
                },
                // _ => panic!("unknown character: '{}'", char)
                _ => {
                    self.advance();
                    Token::Illegal
                },
            };
        }
        
        Token::EOF
    }
}

fn main() {
    let input = "123+123=246";
    let mut lexer = Lexer::new(input);

    loop {
        let token = lexer.next_token();
        println!("{:?}", token);

        if token == Token::EOF {
            break;
        }
    }
}

