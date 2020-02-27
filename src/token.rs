use {
    crate::function::Function,
    regex::Regex,
    std::fmt::{self, Debug, Display, Formatter},
};

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct Ident(String);
impl Ident {
    pub fn new(s: String) -> Option<Self> {
        if Regex::new(r#"^((\s)+)?(\D\w*)(\d)?"#).unwrap().is_match(&s) {
            Some(Self(s))
        } else {
            None
        }
    }

    pub fn internal_cloned(&self) -> String {
        self.0.clone()
    }
}
impl Display for Ident {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    USub,
    Pow,
    Assign,
    LParen,
    RParen,
}
impl Operator {
    fn new(s: String) -> Option<Self> {
        match s.as_str() {
            "+" => Some(Self::Add),
            "-" => Some(Self::Sub),
            "*" => Some(Self::Mul),
            "/" => Some(Self::Div),
            "^" => Some(Self::Pow),
            "=" => Some(Self::Assign),
            "(" => Some(Self::LParen),
            ")" => Some(Self::RParen),
            _ => None,
        }
    }

    pub fn operate(&self, a: f64, b: f64) -> Result<f64, String> {
        match self {
            Self::Add => Ok(a + b),
            Self::Sub => Ok(a - b),
            Self::Mul => Ok(a * b),
            Self::Div => Ok(b / a),
            Self::Pow => Ok(a.powf(b)),
            Self::USub => Ok(-a),
            _ => Err(format!("operate should not be called on {}", self)),
        }
    }

    pub fn associativity(&self) -> OperatorAssociativity {
        match self {
            Self::Pow => OperatorAssociativity::Right,
            _ => OperatorAssociativity::Left,
        }
    }

    pub fn precedence(&self) -> u8 {
        match self {
            Self::USub => 7,
            Self::Pow => 6,
            Self::Mul | Self::Div => 5,
            Self::Add | Self::Sub => 4,
            _ => 0,
        }
    }
}
impl Display for Operator {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Add => "+",
                Self::Sub => "-",
                Self::Mul => "*",
                Self::Div => "/",
                Self::Pow => "^",
                Self::USub => "u",
                Self::Assign => "=",
                Self::LParen => "(",
                Self::RParen => ")",
            }
        )
    }
}

pub enum OperatorAssociativity {
    Left,
    Right,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Keyword {
    Function,
    Variable,
}
impl Keyword {
    pub fn new(s: String) -> Option<Self> {
        match s.as_str() {
            "function" => Some(Self::Function),
            "var" => Some(Self::Variable),
            _ => None,
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum Token {
    Value(f64),
    Operator(Operator),
    Function(Function),
    Identifier(Ident),
    Keyword(Keyword),
}
impl Token {
    pub fn new(s: String) -> Option<Self> {
        if let Some(kw) = Keyword::new(s.clone()) {
            Some(Self::Keyword(kw))
        } else if let Some(op) = Operator::new(s.clone()) {
            Some(Self::Operator(op))
        } else if let Ok(v) = s.parse::<f64>() {
            Some(Self::Value(v))
        } else if let Some(ident) = Ident::new(s.clone()) {
            Some(Self::Identifier(ident))
        } else {
            None
        }
    }
}
impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Value(v) => format!("{}", v),
                Self::Operator(op) => format!("{}", op),
                Self::Function(func) => format!("{}", func),
                Self::Identifier(ident) => format!("{}", ident.internal_cloned()),
                Self::Keyword(kw) => match kw {
                    Keyword::Function => "function".to_owned(),
                    Keyword::Variable => "var".to_owned(),
                },
            }
        )
    }
}

pub fn tokenize(s: String) -> Result<Vec<Token>, String> {
    let mut nbuffer = String::new();
    let mut idbuffer = String::new();
    let mut idx = 0;
    let mut tokens = Vec::new();

    while let Some(c) = s.chars().nth(idx) {
        // check for unary operators (will always be first or directly following another operator (thanks greg!))
        // unwrap or will make this evalute true if it's the first item in the expression
        if c == '-' && nbuffer.is_empty() {
            if let Some(&Token::Operator(_)) = tokens.last() {
                tokens.push(Token::Operator(Operator::USub));
                idx += 1;
                continue;
            }
        }
        // put numbers into the buffer if idbuffer is empty
        else if "1234567890.".contains(c) && idbuffer.is_empty() {
            if c == '.' && nbuffer.contains('.') {
                return Err(format!(
                    "Encountered unexpected '.' when parsing {}",
                    nbuffer
                ));
            } else {
                nbuffer.push(c);
            }
        }
        // make a token from the buffer, add it, clear the buffer
        else if !nbuffer.is_empty() {
            if let Some(tok) = Token::new(nbuffer) {
                tokens.push(tok)
            } else {
                unreachable!()
            }
            nbuffer = String::new();
            idx -= 1;
        }
        // check for operators, parens
        else if let Some(op) = Operator::new(format!("{}", c)) {
            tokens.push(Token::Operator(op));
        }
        // put characters in the identifier buffer
        else if c.is_ascii_alphabetic() {
            idbuffer.push(c);
        }
        // Check for keywords and identifiers
        else if !idbuffer.is_empty() {
            if let Some(tok) = Token::new(idbuffer.clone()) {
                tokens.push(tok);
                idbuffer = String::new();
            } else {
                return Err(format!(
                    "Something went wrong! {} is not a valid identifier",
                    idbuffer
                ));
            }
            idx -= 1;
        }
        // increment the pointer
        idx += 1;
    }

    // check the buffers for missed tokens
    if !idbuffer.is_empty() {
        if let Some(tok) = Token::new(idbuffer.clone()) {
            tokens.push(tok);
        } else {
            return Err(format!(
                "Something went wrong! {} is not a valid identifier",
                idbuffer
            ));
        }
    }

    if !nbuffer.is_empty() {
        if let Some(tok) = Token::new(nbuffer) {
            tokens.push(tok)
        } else {
            unreachable!()
        }
    }

    Ok(tokens)
}

#[test]
fn test_broad_token_new() {
    assert_eq!(
        Token::new("function".to_owned()).unwrap(),
        Token::Keyword(Keyword::Function)
    );
    assert_eq!(
        Token::new("foo".to_owned()).unwrap(),
        Token::Identifier(Ident::new("foo".to_owned()).unwrap())
    );
    assert_eq!(
        Token::new("a".to_owned()).unwrap(),
        Token::Identifier(Ident::new("a".to_owned()).unwrap())
    );
    assert_eq!(
        Token::new("b".to_owned()).unwrap(),
        Token::Identifier(Ident::new("b".to_owned()).unwrap())
    );
    assert_eq!(
        Token::new("=".to_owned()).unwrap(),
        Token::Operator(Operator::Assign)
    );
    assert_eq!(Token::new("2".to_owned()).unwrap(), Token::Value(2.0));
    assert_eq!(
        Token::new("*".to_owned()).unwrap(),
        Token::Operator(Operator::Mul)
    );
    assert_eq!(
        Token::new("a".to_owned()).unwrap(),
        Token::Identifier(Ident::new("a".to_owned()).unwrap())
    );
    assert_eq!(
        Token::new("+".to_owned()).unwrap(),
        Token::Operator(Operator::Add)
    );
    assert_eq!(
        Token::new("b".to_owned()).unwrap(),
        Token::Identifier(Ident::new("b".to_owned()).unwrap())
    );
}

#[test]
fn test_tokenize() {
    let expected = vec![
        Token::new("function".to_owned()).unwrap(),
        Token::new("foo".to_owned()).unwrap(),
        Token::new("a".to_owned()).unwrap(),
        Token::new("b".to_owned()).unwrap(),
        Token::new("=".to_owned()).unwrap(),
        Token::new("2".to_owned()).unwrap(),
        Token::new("*".to_owned()).unwrap(),
        Token::new("a".to_owned()).unwrap(),
        Token::new("+".to_owned()).unwrap(),
        Token::new("b".to_owned()).unwrap(),
    ];
    assert_eq!(
        tokenize("function foo a b = 2 * a + b".to_owned()).unwrap(),
        expected
    );

    let expected = vec![
        Token::new("var".to_owned()).unwrap(),
        Token::new("foo".to_owned()).unwrap(),
        Token::new("=".to_owned()).unwrap(),
        Token::new("14".to_owned()).unwrap(),
        Token::new("^".to_owned()).unwrap(),
        Token::new("2".to_owned()).unwrap(),
    ];
    assert_eq!(tokenize("var foo = 14 ^ 2".to_owned()).unwrap(), expected);

    let expected = vec![
        Token::new("10".to_owned()).unwrap(),
        Token::new("+".to_owned()).unwrap(),
        Token::new("11".to_owned()).unwrap(),
        Token::new("/".to_owned()).unwrap(),
        Token::new("2".to_owned()).unwrap(),
        Token::new("*".to_owned()).unwrap(),
        Token::new("(".to_owned()).unwrap(),
        Token::new("2".to_owned()).unwrap(),
        Token::new("^".to_owned()).unwrap(),
        Token::new("4".to_owned()).unwrap(),
        Token::new(")".to_owned()).unwrap(),
        Token::new("/".to_owned()).unwrap(),
        Token::new("4".to_owned()).unwrap(),
    ];
    assert_eq!(
        tokenize("10 + 11/2 * (2 ^ 4) /    4".to_owned()).unwrap(),
        expected
    );
}
