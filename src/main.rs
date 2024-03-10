#[derive(Debug)]
enum TokenKind {
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
}

fn main() {
    for char in "+-*/=!@'\"?<>{}().".chars() {
        let kind = match char {
            '+' => TokenKind::Plus,
            '-' => TokenKind::Minus,
            '*' => TokenKind::Asterisk,
            '/' => TokenKind::Slash,
            '=' => TokenKind::Equals,
            '!' => TokenKind::ExclamationMark,
            '@' => TokenKind::At,
            '\'' => TokenKind::SingleQuote,
            '"' => TokenKind::DoubleQuote,
            '?' => TokenKind::QuestionMark,
            '<' => TokenKind::LessThan,
            '>' => TokenKind::GreaterThan,
            '{' => TokenKind::LeftBrace,
            '}' => TokenKind::RightBrace,
            '(' => TokenKind::LeftParan,
            ')' => TokenKind::RightParan,
            '[' => TokenKind::LeftBracket,
            ']' => TokenKind::RightBracket,
            '.' => TokenKind::Dot,
            _ => panic!("unknown character: '{}'", char)
        };

        println!("{:?}", kind)
    }
}
