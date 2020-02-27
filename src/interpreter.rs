use {
    crate::{
        function::Function,
        rpn::{rpn_eval, rpn_gen},
        token::{tokenize, Ident, Token},
    },
    std::{
        collections::HashMap,
        io::{stdin, stdout, Read, Stdin, Write},
    },
};

pub struct Interpreter {
    context: Context,
}
impl Interpreter {
    pub fn begin() -> Result<(), usize> {
        // let mut stdin = stdin();

        // let mut userin = String::new();

        // loop {
        //     println!(">>");
        //     stdout().flush().unwrap();

        //     stdin.read_to_string(&mut userin).unwrap();

        //     let tokens = tokenize(userin.clone(), &self.ctx)

        // }

        Ok(())
    }

    pub fn eval_line(&self, s: String) -> Result<f64, String> {
        Ok(0.0)
    }
}

pub fn evaluate(tokens: &Vec<Token>, ctx: &Context) -> Result<f64, String> {
    let rpn = rpn_gen(tokens, ctx)?;
    rpn_eval(&rpn, ctx)
}

pub struct Context {
    pub functions: HashMap<Ident, Function>,
    pub variables: HashMap<Ident, f64>,
    builtins: Vec<String>,
}
impl Context {
    pub fn new() -> Self {
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
            ],
        }
    }

    pub fn call_function(&self, ident: Ident, args: &Vec<Token>) -> Result<f64, String> {
        if let Some(func) = self.functions.get(&ident) {
            func.call(args, &self)
        } else if self.builtins.contains(&ident.internal_cloned()) {
            match ident.internal_cloned().as_str() {
                "sin" => match args.get(0) {
                    Some(a) => match a {
                        Token::Value(va) => Ok(va.sin()),
                        _ => Err(format!(
                            "Error calling `sin`: expected value, recieved {}",
                            a
                        )),
                    },
                    _ => Err(format!("Expected 1 argument, recieved 0")),
                },
                "cos" => match args.get(0) {
                    Some(a) => match a {
                        Token::Value(va) => Ok(va.cos()),
                        _ => Err(format!(
                            "Error calling `cos`: expected value, recieved {}",
                            a
                        )),
                    },
                    _ => Err(format!("Expected 1 argument, recieved 0")),
                },
                "tan" => match args.get(0) {
                    Some(a) => match a {
                        Token::Value(va) => Ok(va.tan()),
                        _ => Err(format!(
                            "Error calling `tan`: expected value, recieved {}",
                            a
                        )),
                    },
                    _ => Err(format!("Expected 1 argument, recieved 0")),
                },
                "asin" => match args.get(0) {
                    Some(a) => match a {
                        Token::Value(va) => Ok(va.asin()),
                        _ => Err(format!(
                            "Error calling `asin`: expected value, recieved {}",
                            a
                        )),
                    },
                    _ => Err(format!("Expected 1 argument, recieved 0")),
                },
                "acos" => match args.get(0) {
                    Some(a) => match a {
                        Token::Value(va) => Ok(va.acos()),
                        _ => Err(format!(
                            "Error calling `acos`: expected value, recieved {}",
                            a
                        )),
                    },
                    _ => Err(format!("Expected 1 argument, recieved 0")),
                },
                "atan" => match args.get(0) {
                    Some(a) => match a {
                        Token::Value(va) => Ok(va.atan()),
                        _ => Err(format!(
                            "Error calling `atan`: expected value, recieved {}",
                            a
                        )),
                    },
                    _ => Err(format!("Expected 1 argument, recieved 0")),
                },
                "min" => match (args.get(0), args.get(1)) {
                    (Some(a), Some(b)) => match (a, b) {
                        (Token::Value(va), Token::Value(vb)) => Ok(va.min(*vb)),
                        _ => Err(format!(
                            "Error calling `min`: expected values, got {}, {}",
                            a, b
                        )),
                    },
                    _ => Err(format!("Expected 2 arguments, recieved {}", args.len())),
                },
                "max" => match (args.get(0), args.get(1)) {
                    (Some(a), Some(b)) => match (a, b) {
                        (Token::Value(va), Token::Value(vb)) => Ok(va.max(*vb)),
                        _ => Err(format!(
                            "Error calling `max`: expected values, got {}, {}",
                            a, b
                        )),
                    },
                    _ => Err(format!("Expected 2 arguments, recieved {}", args.len())),
                },
                _ => unreachable!(),
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
