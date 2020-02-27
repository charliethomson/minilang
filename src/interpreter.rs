use {
    crate::{
        function::Function,
        rpn::{rpn_eval, rpn_gen},
        token::{tokenize, Ident, Keyword, Operator, Token},
    },
    std::{
        collections::HashMap,
        io::{stdin, stdout, Write},
    },
};

pub struct Interpreter {
    context: Context,
}
impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            context: Context::new(),
        }
    }
    pub fn begin(mut self) -> Result<(), usize> {
        let stdin = stdin();

        loop {
            print!(">> ");
            stdout().flush().unwrap();

            let mut userin = String::new();

            stdin.read_line(&mut userin).unwrap();

            let tokens = match tokenize(userin.clone()) {
                Ok(toks) => toks,
                Err(e) => {
                    eprintln!("Error: {}", e);
                    continue;
                }
            };

            if let Some(Token::Keyword(kw)) = tokens.get(0) {
                match kw {
                    Keyword::Function => {
                        match self.context.new_func(&tokens) {
                            Ok(func) => println!("{}", func),
                            Err(e) => eprintln!("Error: {}", e),
                        };
                        continue;
                    }
                    Keyword::Variable => match self.context.new_var(&tokens) {
                        Ok((ident, val)) => println!("{} = {}", ident, val),
                        Err(e) => eprintln!("Error: {}", e),
                    },
                }
            } else {
                match evaluate(&tokens, &self.context) {
                    Ok(val) => println!("{}", val),
                    Err(e) => eprintln!("Error: {:?}", e),
                };
            }
        }
    }
}

pub fn evaluate(tokens: &Vec<Token>, ctx: &Context) -> Result<f64, String> {
    let rpn = rpn_gen(tokens, ctx)?;
    rpn_eval(&rpn, ctx)
}

pub struct Context {
    pub functions: HashMap<Ident, Function>,
    pub variables: HashMap<Ident, f64>,
}
impl Context {
    pub fn new() -> Self {
        let mut functions = HashMap::new();
        functions.insert(
            Ident::new("sin".to_owned()).unwrap(),
            Function {
                ident: Ident::new("sin".to_owned()).unwrap(),
                args: vec![Ident::new("a".to_owned()).unwrap()],
                code: vec![Token::Identifier(Ident::new("a".to_owned()).unwrap())],
            },
        );
        functions.insert(
            Ident::new("cos".to_owned()).unwrap(),
            Function {
                ident: Ident::new("cos".to_owned()).unwrap(),
                args: vec![Ident::new("a".to_owned()).unwrap()],
                code: vec![Token::Identifier(Ident::new("a".to_owned()).unwrap())],
            },
        );
        functions.insert(
            Ident::new("tan".to_owned()).unwrap(),
            Function {
                ident: Ident::new("tan".to_owned()).unwrap(),
                args: vec![Ident::new("a".to_owned()).unwrap()],
                code: vec![Token::Identifier(Ident::new("a".to_owned()).unwrap())],
            },
        );
        functions.insert(
            Ident::new("asin".to_owned()).unwrap(),
            Function {
                ident: Ident::new("asin".to_owned()).unwrap(),
                args: vec![Ident::new("a".to_owned()).unwrap()],
                code: vec![Token::Identifier(Ident::new("a".to_owned()).unwrap())],
            },
        );
        functions.insert(
            Ident::new("acos".to_owned()).unwrap(),
            Function {
                ident: Ident::new("acos".to_owned()).unwrap(),
                args: vec![Ident::new("a".to_owned()).unwrap()],
                code: vec![Token::Identifier(Ident::new("a".to_owned()).unwrap())],
            },
        );
        functions.insert(
            Ident::new("atan".to_owned()).unwrap(),
            Function {
                ident: Ident::new("atan".to_owned()).unwrap(),
                args: vec![Ident::new("a".to_owned()).unwrap()],
                code: vec![Token::Identifier(Ident::new("a".to_owned()).unwrap())],
            },
        );
        functions.insert(
            Ident::new("min".to_owned()).unwrap(),
            Function {
                ident: Ident::new("min".to_owned()).unwrap(),
                args: vec![
                    Ident::new("a".to_owned()).unwrap(),
                    Ident::new("b".to_owned()).unwrap(),
                ],
                code: vec![Token::Identifier(Ident::new("a".to_owned()).unwrap())],
            },
        );
        functions.insert(
            Ident::new("max".to_owned()).unwrap(),
            Function {
                ident: Ident::new("max".to_owned()).unwrap(),
                args: vec![
                    Ident::new("a".to_owned()).unwrap(),
                    Ident::new("b".to_owned()).unwrap(),
                ],
                code: vec![Token::Identifier(Ident::new("a".to_owned()).unwrap())],
            },
        );

        Context {
            functions,
            variables: HashMap::new(),
        }
    }

    pub fn new_var(&mut self, tokens: &Vec<Token>) -> Result<(Ident, f64), String> {
        if let Some(Token::Keyword(Keyword::Variable)) = tokens.first() {
            if let Some(Token::Identifier(ident)) = tokens.get(1) {
                if tokens.get(2) != Some(&Token::Operator(Operator::Assign)) {
                    Err(format!(
                        "Unexpected token before assignment operator in varible assignment: {:?}",
                        tokens.get(2)
                    ))
                } else {
                    let code = tokens.clone().drain(3..).collect::<Vec<Token>>();
                    let val = evaluate(&code, &self)?;
                    self.variables.insert(ident.clone(), val);
                    Ok((ident.clone(), val))
                }
            } else {
                Err("`var` keyword not followed by an identifier".to_owned())
            }
        } else {
            unreachable!()
        }
    }

    pub fn new_func(&mut self, tokens: &Vec<Token>) -> Result<Function, String> {
        let func = Function::new(tokens)?;
        self.functions.insert(func.ident.clone(), func.clone());
        Ok(func)
    }

    pub fn call_function(&self, ident: Ident, args: &Vec<Token>) -> Result<f64, String> {
        if let Some(func) = self.functions.get(&ident) {
            match ident.internal_cloned().as_str() {
                "sin" => {
                    if args.len() != 1 {
                        Err(format!("Expected 1 argument, got {}", args.len()))
                    } else {
                        let a = args.get(0).unwrap();
                        if let Token::Value(v) = a {
                            Ok(v.sin())
                        } else {
                            Err(format!("Expected Value, got {}", a))
                        }
                    }
                }
                "cos" => {
                    if args.len() != 1 {
                        Err(format!("Expected 1 argument, got {}", args.len()))
                    } else {
                        let a = args.get(0).unwrap();
                        if let Token::Value(v) = a {
                            Ok(v.sin())
                        } else {
                            Err(format!("Expected Value, got {}", a))
                        }
                    }
                }
                "tan" => {
                    if args.len() != 1 {
                        Err(format!("Expected 1 argument, got {}", args.len()))
                    } else {
                        let a = args.get(0).unwrap();
                        if let Token::Value(v) = a {
                            Ok(v.sin())
                        } else {
                            Err(format!("Expected Value, got {}", a))
                        }
                    }
                }
                "asin" => {
                    if args.len() != 1 {
                        Err(format!("Expected 1 argument, got {}", args.len()))
                    } else {
                        let a = args.get(0).unwrap();
                        if let Token::Value(v) = a {
                            Ok(v.sin())
                        } else {
                            Err(format!("Expected Value, got {}", a))
                        }
                    }
                }
                "acos" => {
                    if args.len() != 1 {
                        Err(format!("Expected 1 argument, got {}", args.len()))
                    } else {
                        let a = args.get(0).unwrap();
                        if let Token::Value(v) = a {
                            Ok(v.sin())
                        } else {
                            Err(format!("Expected Value, got {}", a))
                        }
                    }
                }
                "atan" => {
                    if args.len() != 1 {
                        Err(format!("Expected 1 argument, got {}", args.len()))
                    } else {
                        let a = args.get(0).unwrap();
                        if let Token::Value(v) = a {
                            Ok(v.sin())
                        } else {
                            Err(format!("Expected Value, got {}", a))
                        }
                    }
                }
                "min" => {
                    if args.len() != 2 {
                        Err(format!("Expected 2 arguments, got {}", args.len()))
                    } else {
                        let (a, b) = (args.get(0).unwrap(), args.get(1).unwrap());
                        if let (Token::Value(va), Token::Value(vb)) = (a, b) {
                            Ok(va.min(*vb))
                        } else {
                            Err(format!("Expected (Value, Value), got ({}, {})", a, b))
                        }
                    }
                }
                "max" => {
                    if args.len() != 2 {
                        Err(format!("Expected 2 arguments, got {}", args.len()))
                    } else {
                        let (a, b) = (args.get(0).unwrap(), args.get(1).unwrap());
                        if let (Token::Value(va), Token::Value(vb)) = (a, b) {
                            Ok(va.max(*vb))
                        } else {
                            Err(format!("Expected (Value, Value), got ({}, {})", a, b))
                        }
                    }
                }
                _ => func.call(args, &self),
            }
        } else {
            Err(format!("Unknown function {}", ident))
        }
    }
}

#[test]
fn test_builtin_call() {
    let ctx = Context::new();

    assert!(ctx
        .call_function(
            Ident::new("sin".to_owned()).unwrap(),
            &vec![Token::Value(1.0)]
        )
        .is_ok());
    assert!(ctx
        .call_function(Ident::new("sin".to_owned()).unwrap(), &vec![])
        .is_err());

    assert!(ctx
        .call_function(
            Ident::new("cos".to_owned()).unwrap(),
            &vec![Token::Value(1.0)]
        )
        .is_ok());
    assert!(ctx
        .call_function(Ident::new("cos".to_owned()).unwrap(), &vec![])
        .is_err());

    assert!(ctx
        .call_function(
            Ident::new("tan".to_owned()).unwrap(),
            &vec![Token::Value(1.0)]
        )
        .is_ok());
    assert!(ctx
        .call_function(Ident::new("tan".to_owned()).unwrap(), &vec![])
        .is_err());

    assert!(ctx
        .call_function(
            Ident::new("asin".to_owned()).unwrap(),
            &vec![Token::Value(1.0)]
        )
        .is_ok());
    assert!(ctx
        .call_function(Ident::new("asin".to_owned()).unwrap(), &vec![])
        .is_err());

    assert!(ctx
        .call_function(
            Ident::new("acos".to_owned()).unwrap(),
            &vec![Token::Value(1.0)]
        )
        .is_ok());
    assert!(ctx
        .call_function(Ident::new("acos".to_owned()).unwrap(), &vec![])
        .is_err());

    assert!(ctx
        .call_function(
            Ident::new("atan".to_owned()).unwrap(),
            &vec![Token::Value(1.0)]
        )
        .is_ok());
    assert!(ctx
        .call_function(Ident::new("atan".to_owned()).unwrap(), &vec![])
        .is_err());

    assert!(ctx
        .call_function(
            Ident::new("min".to_owned()).unwrap(),
            &vec![Token::Value(10.0), Token::Value(15.0)]
        )
        .is_ok());
    assert!(ctx
        .call_function(
            Ident::new("min".to_owned()).unwrap(),
            &vec![Token::Value(10.0)]
        )
        .is_err());

    assert!(ctx
        .call_function(
            Ident::new("max".to_owned()).unwrap(),
            &vec![Token::Value(10.0), Token::Value(15.0)]
        )
        .is_ok());
    assert!(ctx
        .call_function(
            Ident::new("max".to_owned()).unwrap(),
            &vec![Token::Value(10.0)]
        )
        .is_err());

    assert_eq!(
        ctx.call_function(
            Ident::new("min".to_owned()).unwrap(),
            &vec![Token::Value(10.0), Token::Value(15.0)]
        )
        .unwrap(),
        10.0
    );
    assert_eq!(
        ctx.call_function(
            Ident::new("max".to_owned()).unwrap(),
            &vec![Token::Value(10.0), Token::Value(15.0)]
        )
        .unwrap(),
        15.0
    );
}

#[test]
fn test_greater_eval() {
    let mut ctx = Context::new();

    ctx.variables
        .insert(Ident::new("a".to_owned()).unwrap(), 10.0);

    let with_vars = evaluate(&tokenize("a + 2".to_owned()).unwrap(), &ctx);
    eprintln!("With vars: {:?}", with_vars);

    let add = Function {
        ident: Ident::new("add".to_owned()).unwrap(),
        args: vec![
            Ident::new("a".to_owned()).unwrap(),
            Ident::new("b".to_owned()).unwrap(),
        ],
        code: vec![
            Token::new("a".to_owned()).unwrap(),
            Token::new("+".to_owned()).unwrap(),
            Token::new("b".to_owned()).unwrap(),
        ],
    };
    ctx.functions
        .insert(Ident::new("add".to_owned()).unwrap(), add.clone());
    let with_fn_call = evaluate(&tokenize("add 1 2".to_owned()).unwrap(), &ctx);
    eprintln!("With fn call: {:?}", with_fn_call);
}
