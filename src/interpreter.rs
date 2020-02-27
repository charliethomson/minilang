
use {
    std::{
        collections::HashMap,
    },
    crate::{
        function::Function,
        token::{
            Token, Ident,
        }
    },
};

pub struct Interpreter {
    context: Context,

} impl Interpreter {
    pub fn begin() -> Result<(), usize> {
        Ok(())
    }
}

pub fn rpn_gen(tokens: &Vec<Token>) -> Result<Vec<Token>, String> {
    Err("".to_owned())
}

pub fn rpn_eval(tokens_rpn: &Vec<Token>) -> Result<f64, String> {
    Err("".to_owned())
}

pub fn evaluate(tokens: &Vec<Token>) -> Result<f64, String> {
    let rpn = rpn_gen(tokens)?;
    rpn_eval(&rpn)
}


pub struct Context {
    pub functions: HashMap<Ident, Function>,
    pub variables: HashMap<Ident, f64>,
    builtins: Vec<String>,
} impl Context {
    fn new() -> Self {
        Context {
            functions: HashMap::new(),
            variables: HashMap::new(),
            builtins: vec![
                "sin".to_owned(),
                "cos".to_owned(),
                "tan".to_owned(),
                "asin".to_owned(),
                "acos".to_owned(),
                "atan".to_owned(),
                "min".to_owned(),
                "max".to_owned(),
            ]
        }
    }

    pub fn call_function(&self, ident: Ident, args: &Vec<Token>) -> Result<f64, String> {
        if let Some(func) = self.functions.get(&ident) {
            func.call(args, &self)
        } else if self.builtins.contains(&ident.internal_cloned()) {
            match ident.internal_cloned().as_str() {
                "sin" => {
                    match args.get(0) {
                        Some(a) => {
                            match a {
                                Token::Value(va) => Ok(va.sin()),
                                _ => {
                                    Err(format!("Error calling `sin`: expected value, recieved {}", a))
                                }
                            }
                        },
                        _ => {
                            Err(format!("Expected 1 argument, recieved 0"))
                        }
                    }
                },
                "cos" => {
                    match args.get(0) {
                        Some(a) => {
                            match a {
                                Token::Value(va) => Ok(va.cos()),
                                _ => {
                                    Err(format!("Error calling `cos`: expected value, recieved {}", a))
                                }
                            }
                        },
                        _ => {
                            Err(format!("Expected 1 argument, recieved 0"))
                        }
                    }

                },
                "tan" => {
                    match args.get(0) {
                        Some(a) => {
                            match a {
                                Token::Value(va) => Ok(va.tan()),
                                _ => {
                                    Err(format!("Error calling `tan`: expected value, recieved {}", a))
                                }
                            }
                        },
                        _ => {
                            Err(format!("Expected 1 argument, recieved 0"))
                        }
                    }

                },
                "asin" => {
                    match args.get(0) {
                        Some(a) => {
                            match a {
                                Token::Value(va) => Ok(va.asin()),
                                _ => {
                                    Err(format!("Error calling `asin`: expected value, recieved {}", a))
                                }
                            }
                        },
                        _ => {
                            Err(format!("Expected 1 argument, recieved 0"))
                        }
                    }

                },
                "acos" => {
                    match args.get(0) {
                        Some(a) => {
                            match a {
                                Token::Value(va) => Ok(va.acos()),
                                _ => {
                                    Err(format!("Error calling `acos`: expected value, recieved {}", a))
                                }
                            }
                        },
                        _ => {
                            Err(format!("Expected 1 argument, recieved 0"))
                        }
                    }

                },
                "atan" => {
                    match args.get(0) {
                        Some(a) => {
                            match a {
                                Token::Value(va) => Ok(va.atan()),
                                _ => {
                                    Err(format!("Error calling `atan`: expected value, recieved {}", a))
                                }
                            }
                        },
                        _ => {
                            Err(format!("Expected 1 argument, recieved 0"))
                        }
                    }

                },
                "min" => {
                    match (args.get(0), args.get(1)) {
                        (Some(a), Some(b)) => {
                            match (a, b) {
                                (Token::Value(va), Token::Value(vb)) => Ok(va.min(*vb)),
                                _ => {
                                    Err(format!("Error calling `min`: expected values, got {}, {}", a, b))
                                }
                            }
                        },
                        _ => {
                            Err(format!("Expected 2 arguments, recieved {}", args.len()))
                        }
                    }
                },
                "max" => {
                    match (args.get(0), args.get(1)) {
                        (Some(a), Some(b)) => {
                            match (a, b) {
                                (Token::Value(va), Token::Value(vb)) => Ok(va.max(*vb)),
                                _ => {
                                    Err(format!("Error calling `max`: expected values, got {}, {}", a, b))
                                }
                            }
                        },
                        _ => {
                            Err(format!("Expected 2 arguments, recieved {}", args.len()))
                        }
                    }

                },
                _ => unreachable!()
            }
        } else {
            Err(format!("Unknown function {}", ident))
        }
    }
}

#[test]
fn test_builtin_call() {
    let ctx = Context::new();

    assert!(ctx.call_function(Ident::new("sin".to_owned()).unwrap(), &vec![Token::Value(1.0)]).is_ok());
    assert!(ctx.call_function(Ident::new("sin".to_owned()).unwrap(), &vec![]).is_err());

    assert!(ctx.call_function(Ident::new("cos".to_owned()).unwrap(), &vec![Token::Value(1.0)]).is_ok());
    assert!(ctx.call_function(Ident::new("cos".to_owned()).unwrap(), &vec![]).is_err());

    assert!(ctx.call_function(Ident::new("tan".to_owned()).unwrap(), &vec![Token::Value(1.0)]).is_ok());
    assert!(ctx.call_function(Ident::new("tan".to_owned()).unwrap(), &vec![]).is_err());
    
    assert!(ctx.call_function(Ident::new("asin".to_owned()).unwrap(), &vec![Token::Value(1.0)]).is_ok());
    assert!(ctx.call_function(Ident::new("asin".to_owned()).unwrap(), &vec![]).is_err());

    assert!(ctx.call_function(Ident::new("acos".to_owned()).unwrap(), &vec![Token::Value(1.0)]).is_ok());
    assert!(ctx.call_function(Ident::new("acos".to_owned()).unwrap(), &vec![]).is_err());

    assert!(ctx.call_function(Ident::new("atan".to_owned()).unwrap(), &vec![Token::Value(1.0)]).is_ok());
    assert!(ctx.call_function(Ident::new("atan".to_owned()).unwrap(), &vec![]).is_err());
    

    assert!(ctx.call_function(Ident::new("min".to_owned()).unwrap(), &vec![Token::Value(10.0), Token::Value(15.0)]).is_ok());
    assert!(ctx.call_function(Ident::new("min".to_owned()).unwrap(), &vec![Token::Value(10.0)]).is_err());

    assert!(ctx.call_function(Ident::new("max".to_owned()).unwrap(), &vec![Token::Value(10.0), Token::Value(15.0)]).is_ok());
    assert!(ctx.call_function(Ident::new("max".to_owned()).unwrap(), &vec![Token::Value(10.0)]).is_err());

    assert_eq!(ctx.call_function(Ident::new("min".to_owned()).unwrap(), &vec![Token::Value(10.0), Token::Value(15.0)]).unwrap(), 10.0);
    assert_eq!(ctx.call_function(Ident::new("max".to_owned()).unwrap(), &vec![Token::Value(10.0), Token::Value(15.0)]).unwrap(), 15.0);

}