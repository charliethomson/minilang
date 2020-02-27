
use {
    std::{
        collections::HashMap,
        fmt::{ Display, Debug, Formatter, Result as fmt_Result },
        ops::Index,
    },
    crate::{
        token::{
            Ident,
            Token,
            Keyword,
            Operator,
            tokenize,
        },
        interpreter::{
            Context,
            evaluate,
        },
    }
};

#[derive(PartialEq, Clone, Debug)]
pub struct Function {
    ident: Ident,
    args: Vec<Ident>,
    code: Vec<Token>,
} impl Function {
    pub fn new(tokens: &Vec<Token>) -> Result<Self, String> {
        if let Some(Token::Keyword(Keyword::Function)) = tokens.first() {
            if let Some(Token::Identifier(ident)) = tokens.get(1) {
                let mut found_assn = false;
                let mut args = Vec::new();
                let mut idx = 2;
                while let Some(tok) = tokens.get(idx) {
                    idx += 1;
                    match tok {
                        Token::Operator(Operator::Assign) => break,
                        Token::Identifier(ident) => args.push(ident.clone()),
                        _ => return Err(format!("Expected identifier in declaration of {}, got: {:?}", ident, tok))
                    }
                }

                let code = tokens.get(idx..).unwrap().iter().cloned().collect::<Vec<Token>>();
                
                if code.is_empty() {
                    return Err("Function declaration with no body".to_owned())
                }

                return Ok(Function {
                    ident: ident.clone(),
                    args,
                    code
                })
            } else {
                return Err("`function` keyword not followed by an identifier".to_owned())
            }
        } else {
            unreachable!()
        }
    }

    pub fn call(&self, args: &Vec<Token>, ctx: &Context) -> Result<f64, String> {
        let mut code = Vec::new();

        for tok in &self.code {
            match tok {
                Token::Identifier(ident) => {
                    if self.args.contains(&ident) {
                        let idx = self.args.iter().position(|id| id == ident).unwrap();
                        if let Some(arg) = args.get(idx) {
                            code.push(arg.clone());
                        } else {
                            return Err(format!("Did not recieve the right amount of arguments. Expected {}, got {}", self.args.len(), args.len()));
                        }
                    } else if let Some(val) = ctx.variables.get(&ident) {
                        code.push(Token::Value(*val));
                    } else {
                        return Err(format!("Unknown identifier {}", ident))
                    }
                },
                _ => code.push(tok.clone())
            }
        }

        evaluate(&code)
    }

} impl Display for Function {
    fn fmt(&self, f: &mut Formatter) -> fmt_Result {
        write!(f, "function {} {} = {}",
            self.ident,
            self.args.iter().map(|i| i.internal_cloned()).collect::<Vec<String>>().join(" "),
            self.code.iter().map(|tok| format!("{}", tok)).collect::<Vec<String>>().join(" ")
        )
    }
}

#[test]
fn test_function_new() {

    assert!(Function::new(&tokenize("function = 10 + 2".to_owned()).unwrap()).is_err());
    assert!(Function::new(&tokenize("function foo a b = ".to_owned()).unwrap()).is_err());
    assert!(Function::new(&tokenize("function foo 10 + b = 0.0 / 2".to_owned()).unwrap()).is_err());

    let expected = Function {
        ident: Ident::new("foo".to_owned()).unwrap(),
        args: vec![
            Ident::new("a".to_owned()).unwrap(),
            Ident::new("b".to_owned()).unwrap(),
        ],
        code: vec![
            Token::new("2".to_owned()).unwrap(),
            Token::new("*".to_owned()).unwrap(),
            Token::new("a".to_owned()).unwrap(),
            Token::new("+".to_owned()).unwrap(),
            Token::new("b".to_owned()).unwrap(),
        ]
    };

    assert_eq!(Function::new(&tokenize("function foo a b = 2 * a + b".to_owned()).unwrap()).unwrap(), expected);
}