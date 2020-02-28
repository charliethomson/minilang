use crate::{
    interpreter::Context,
    token::{Operator, OperatorAssociativity, Token},
};

pub fn rpn_gen(tokens: &Vec<Token>, ctx: &Context) -> Result<Vec<Token>, String> {
    let mut stack: Vec<Token> = Vec::new();
    let mut output = Vec::new();
    let tokens = tokens.clone();
    for tok in tokens {
        match tok {
            Token::Value(_) => output.push(tok),
            Token::Identifier(ident) => {
                if let Some(func) = ctx.functions.get(&ident) {
                    stack.push(Token::Function(func.clone()));
                } else if let Some(val) = ctx.variables.get(&ident) {
                    output.push(Token::Value(*val));
                } else {
                    return Err(format!("Unknown identifier {}", ident));
                }
            }
            Token::Operator(op) => {
                match op {
                    Operator::LParen => {
                        stack.push(tok);
                    }
                    Operator::RParen => {
                        let mut got = false;
                        while let Some(top) = stack.pop() {
                            if top == Token::Operator(Operator::LParen) {
                                got = true;
                                break;
                            } else {
                                output.push(top);
                            }
                        }
                        // check if we got a paren
                        if !got {
                            return Err("Mismatched parentheses".to_owned());
                        }
                    }
                    Operator::Assign => {
                        unreachable!();
                    }
                    _ => {
                        while let Some(top) = stack.pop() {
                            if match top {
                                Token::Operator(Operator::LParen) => {
                                    stack.push(top);
                                    break;
                                }
                                Token::Operator(op2) => match op.associativity() {
                                    OperatorAssociativity::Left => {
                                        op2.precedence() >= op.precedence()
                                    }
                                    OperatorAssociativity::Right => {
                                        op2.precedence() > op.precedence()
                                    }
                                },
                                Token::Function(_) => true,
                                _ => false,
                            } {
                                output.push(top);
                            } else {
                                stack.push(top);
                                break;
                            }
                        }
                        stack.push(tok);
                    }
                }
            }
            _ => (),
        }
    }

    while let Some(top) = stack.pop() {
        output.push(top);
    }

    Ok(output)
}

pub fn rpn_eval(tokens_rpn: &Vec<Token>, ctx: &Context) -> Result<f64, String> {
    let mut stack = Vec::new();
    let tokens = tokens_rpn.clone();
    for tok in tokens {
        match tok {
            Token::Value(v) => stack.push(v),
            Token::Operator(op) => match op {
                Operator::USub => {
                    if let Some(v) = stack.pop() {
                        stack.push(-v);
                    } else {
                        eprintln!("NONE 1")
                    }
                }
                _ => {
                    if let (Some(a), Some(b)) = (stack.pop(), stack.pop()) {
                        stack.push(op.operate(a, b)?);
                    } else {
                        eprintln!("NONE 2")
                    }
                }
            },
            Token::Function(func) => {
                let argc = func.argc();
                if argc > stack.len() {
                    return Err(format!(
                        "Parsing error: Not enough items on the stack to call {}",
                        func
                    ));
                } else {
                    let args = stack
                        .drain(stack.len() - argc..)
                        .map(Token::Value)
                        .collect::<Vec<Token>>();

                    stack.push(ctx.call_function(func.ident.clone(), &args)?)
                }
            }
            _ => eprintln!("{:?}", tok),
        }
    }

    if let Some(v) = stack.pop() {
        Ok(v)
    } else {
        Err(format!("Failed: {:?}", stack))
    }
}

#[test]
fn test_rpn_eval() {
    use crate::{
        function::Function,
        token::{tokenize, Ident},
    };
    // 10 + 2 * 3 = 60
    let tokens = vec![
        Token::new("10".to_owned()).unwrap(),
        Token::new("2".to_owned()).unwrap(),
        Token::new("3".to_owned()).unwrap(),
        Token::new("*".to_owned()).unwrap(),
        Token::new("+".to_owned()).unwrap(),
    ];
    let mut ctx = Context::new();
    eprintln!("{:?}", rpn_eval(&tokens, &ctx));

    let mul = Function {
        ident: Ident::new("mul".to_owned()).unwrap(),
        args: vec![
            Ident::new("a".to_owned()).unwrap(),
            Ident::new("b".to_owned()).unwrap(),
        ],
        code: tokenize("a * b".to_owned()).unwrap(),
    };
    ctx.functions
        .insert(Ident::new("mul".to_owned()).unwrap(), mul.clone());
    let tokens = vec![
        Token::new("10".to_owned()).unwrap(),
        Token::new("2".to_owned()).unwrap(),
        Token::new("3".to_owned()).unwrap(),
        Token::Function(mul),
        Token::new("+".to_owned()).unwrap(),
    ];
    eprintln!("{:?}", rpn_eval(&tokens, &ctx));
}
