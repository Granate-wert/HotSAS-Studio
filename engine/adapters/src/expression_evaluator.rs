use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(f64),
    Variable(String),
    Function(String),
    Operator(char),
    LeftParen,
    RightParen,
    Comma,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
}

impl Op {
    fn precedence(&self) -> u8 {
        match self {
            Op::Add | Op::Sub => 1,
            Op::Mul | Op::Div => 2,
            Op::Pow => 3,
        }
    }

    fn right_associative(&self) -> bool {
        matches!(self, Op::Pow)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum RpnToken {
    Number(f64),
    Variable(String),
    Function(String, usize), // name, arity
    Operator(Op),
}

/// Parse an infix expression string into tokens.
/// Supports: numbers (decimal), variables (letters/digits/underscore),
/// functions: sqrt, exp, ln, log10, pow, abs
/// operators: + - * / ^
/// constants: pi
/// parentheses and commas.
pub fn tokenize(expression: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let chars: Vec<char> = expression.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        let c = chars[i];

        if c.is_whitespace() {
            i += 1;
            continue;
        }

        if c.is_ascii_digit() || c == '.' {
            let start = i;
            while i < chars.len() && (chars[i].is_ascii_digit() || chars[i] == '.') {
                i += 1;
            }
            let num_str: String = chars[start..i].iter().collect();
            match num_str.parse::<f64>() {
                Ok(n) => tokens.push(Token::Number(n)),
                Err(_) => return Err(format!("invalid number: {num_str}")),
            }
            continue;
        }

        if c.is_alphabetic() || c == '_' {
            let start = i;
            while i < chars.len() && (chars[i].is_alphanumeric() || chars[i] == '_') {
                i += 1;
            }
            let name: String = chars[start..i].iter().collect();
            match name.as_str() {
                "pi" => tokens.push(Token::Number(std::f64::consts::PI)),
                "sqrt" | "exp" | "ln" | "log10" | "pow" | "abs" => {
                    tokens.push(Token::Function(name));
                }
                _ => tokens.push(Token::Variable(name)),
            }
            continue;
        }

        match c {
            '+' => tokens.push(Token::Operator('+')),
            '-' => tokens.push(Token::Operator('-')),
            '*' => tokens.push(Token::Operator('*')),
            '/' => tokens.push(Token::Operator('/')),
            '^' => tokens.push(Token::Operator('^')),
            '(' => tokens.push(Token::LeftParen),
            ')' => tokens.push(Token::RightParen),
            ',' => tokens.push(Token::Comma),
            _ => return Err(format!("unexpected character: {c}")),
        }
        i += 1;
    }

    Ok(tokens)
}

#[derive(Debug, Clone, PartialEq)]
enum StackItem {
    Op(Op),
    LParen,
    Function(String, usize), // name, arg_count
}

/// Convert infix tokens to RPN using the shunting-yard algorithm.
pub fn to_rpn(tokens: &[Token]) -> Result<Vec<RpnToken>, String> {
    let mut output: Vec<RpnToken> = Vec::new();
    let mut stack: Vec<StackItem> = Vec::new();

    let mut i = 0;
    while i < tokens.len() {
        match &tokens[i] {
            Token::Number(n) => output.push(RpnToken::Number(*n)),
            Token::Variable(v) => output.push(RpnToken::Variable(v.clone())),
            Token::Function(f) => {
                stack.push(StackItem::Function(f.clone(), 1));
            }
            Token::Operator(op_char) => {
                // Detect unary minus: at start, after '(', after ',', or after another operator
                let is_unary = *op_char == '-'
                    && (i == 0
                        || matches!(
                            tokens[i - 1],
                            Token::Operator(_) | Token::LeftParen | Token::Comma
                        ));
                if is_unary {
                    output.push(RpnToken::Number(0.0));
                }
                let op = match op_char {
                    '+' => Op::Add,
                    '-' => Op::Sub,
                    '*' => Op::Mul,
                    '/' => Op::Div,
                    '^' => Op::Pow,
                    _ => return Err(format!("unknown operator: {op_char}")),
                };

                while let Some(StackItem::Op(top_op)) = stack.last() {
                    if top_op.precedence() > op.precedence()
                        || (top_op.precedence() == op.precedence() && !op.right_associative())
                    {
                        output.push(RpnToken::Operator(match stack.pop().unwrap() {
                            StackItem::Op(o) => o,
                            _ => unreachable!(),
                        }));
                    } else {
                        break;
                    }
                }
                stack.push(StackItem::Op(op));
            }
            Token::LeftParen => stack.push(StackItem::LParen),
            Token::RightParen => {
                // Pop until left paren
                let mut found = false;
                while let Some(item) = stack.pop() {
                    match item {
                        StackItem::LParen => {
                            found = true;
                            break;
                        }
                        StackItem::Op(op) => output.push(RpnToken::Operator(op)),
                        StackItem::Function(_, _) => {
                            // Functions should not be popped here; they stay until after paren
                            found = true;
                            stack.push(item);
                            break;
                        }
                    }
                }
                if !found {
                    return Err("mismatched parentheses".to_string());
                }
                // If token before '(' was a function, pop it to output
                if let Some(StackItem::Function(name, arity)) = stack.last() {
                    let name = name.clone();
                    let arity = *arity;
                    stack.pop();
                    output.push(RpnToken::Function(name, arity));
                }
            }
            Token::Comma => {
                // Argument separator: pop until left paren
                let mut found = false;
                while let Some(item) = stack.last() {
                    match item {
                        StackItem::LParen => {
                            found = true;
                            break;
                        }
                        _ => {
                            if let StackItem::Op(op) = stack.pop().unwrap() {
                                output.push(RpnToken::Operator(op));
                            }
                        }
                    }
                }
                if !found {
                    return Err("misplaced comma or mismatched parentheses".to_string());
                }
                // Increment arg count for the function on stack
                for item in stack.iter_mut().rev() {
                    if let StackItem::Function(_, ref mut count) = item {
                        *count += 1;
                        break;
                    }
                }
            }
        }
        i += 1;
    }

    while let Some(item) = stack.pop() {
        match item {
            StackItem::Op(op) => output.push(RpnToken::Operator(op)),
            StackItem::LParen | StackItem::Function(_, _) => {
                return Err("mismatched parentheses".to_string());
            }
        }
    }

    Ok(output)
}

/// Evaluate an RPN expression with the given variable values.
/// Returns the single resulting value, or an error.
pub fn eval_rpn(rpn: &[RpnToken], variables: &BTreeMap<String, f64>) -> Result<f64, String> {
    let mut stack: Vec<f64> = Vec::new();

    for token in rpn {
        match token {
            RpnToken::Number(n) => stack.push(*n),
            RpnToken::Variable(v) => match variables.get(v) {
                Some(val) => stack.push(*val),
                None => return Err(format!("missing variable: {v}")),
            },
            RpnToken::Operator(op) => {
                let b = stack.pop().ok_or("insufficient values for operator")?;
                let a = stack.pop().ok_or("insufficient values for operator")?;
                let result = match op {
                    Op::Add => a + b,
                    Op::Sub => a - b,
                    Op::Mul => a * b,
                    Op::Div => {
                        if b == 0.0 {
                            return Err("division by zero".to_string());
                        }
                        a / b
                    }
                    Op::Pow => a.powf(b),
                };
                stack.push(result);
            }
            RpnToken::Function(name, arity) => {
                if stack.len() < *arity {
                    return Err(format!(
                        "insufficient arguments for function {name} (expected {arity})"
                    ));
                }
                let args: Vec<f64> = stack.split_off(stack.len().saturating_sub(*arity));
                let result = match name.as_str() {
                    "sqrt" => {
                        if args[0] < 0.0 {
                            return Err("sqrt of negative number".to_string());
                        }
                        args[0].sqrt()
                    }
                    "exp" => args[0].exp(),
                    "ln" => {
                        if args[0] <= 0.0 {
                            return Err("ln of non-positive number".to_string());
                        }
                        args[0].ln()
                    }
                    "log10" => {
                        if args[0] <= 0.0 {
                            return Err("log10 of non-positive number".to_string());
                        }
                        args[0].log10()
                    }
                    "abs" => args[0].abs(),
                    "pow" => {
                        if args.len() != 2 {
                            return Err("pow requires 2 arguments".to_string());
                        }
                        args[0].powf(args[1])
                    }
                    _ => return Err(format!("unknown function: {name}")),
                };
                stack.push(result);
            }
        }
    }

    if stack.len() != 1 {
        return Err(format!("expected exactly one result, got {}", stack.len()));
    }

    let result = stack[0];
    if result.is_nan() {
        return Err("result is NaN".to_string());
    }
    if result.is_infinite() {
        return Err("result is infinite".to_string());
    }

    Ok(result)
}

/// Evaluate an expression string with variables.
/// Expression format: "output = expr" or just "expr".
/// Returns the computed value.
pub fn evaluate_expression(
    expression: &str,
    variables: &BTreeMap<String, f64>,
) -> Result<f64, String> {
    // Strip optional "output_name = " prefix
    let expr_part = if let Some(pos) = expression.find('=') {
        &expression[pos + 1..]
    } else {
        expression
    };

    let tokens = tokenize(expr_part)?;
    let rpn = to_rpn(&tokens)?;
    eval_rpn(&rpn, variables)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    fn vars(map: &[(&str, f64)]) -> BTreeMap<String, f64> {
        map.iter().map(|(k, v)| (k.to_string(), *v)).collect()
    }

    #[test]
    fn basic_arithmetic() {
        let v = vars(&[]);
        assert_eq!(evaluate_expression("2 + 3", &v).unwrap(), 5.0);
        assert_eq!(evaluate_expression("10 - 4", &v).unwrap(), 6.0);
        assert_eq!(evaluate_expression("3 * 4", &v).unwrap(), 12.0);
        assert_eq!(evaluate_expression("8 / 2", &v).unwrap(), 4.0);
    }

    #[test]
    fn parentheses() {
        let v = vars(&[]);
        assert_eq!(evaluate_expression("(2 + 3) * 4", &v).unwrap(), 20.0);
        assert_eq!(evaluate_expression("2 + (3 * 4)", &v).unwrap(), 14.0);
    }

    #[test]
    fn variables() {
        let v = vars(&[("R", 10_000.0), ("C", 100e-9)]);
        let result = evaluate_expression("1 / (2 * pi * R * C)", &v).unwrap();
        assert!((result - 159.1549).abs() < 0.01);
    }

    #[test]
    fn sqrt_function() {
        let v = vars(&[("x", 16.0)]);
        assert_eq!(evaluate_expression("sqrt(x)", &v).unwrap(), 4.0);
        assert_eq!(evaluate_expression("sqrt(25)", &vars(&[])).unwrap(), 5.0);
    }

    #[test]
    fn exp_and_ln() {
        let v = vars(&[]);
        assert!((evaluate_expression("exp(1)", &v).unwrap() - 2.71828).abs() < 0.001);
        assert!((evaluate_expression("ln(10)", &v).unwrap() - 2.302585).abs() < 0.001);
    }

    #[test]
    fn log10_and_abs() {
        let v = vars(&[]);
        assert!((evaluate_expression("log10(1000)", &v).unwrap() - 3.0).abs() < 0.001);
        assert_eq!(evaluate_expression("abs(-5)", &v).unwrap(), 5.0);
    }

    #[test]
    fn pow_function() {
        let v = vars(&[]);
        assert_eq!(evaluate_expression("pow(2, 3)", &v).unwrap(), 8.0);
    }

    #[test]
    fn power_operator() {
        let v = vars(&[]);
        assert_eq!(evaluate_expression("2 ^ 3", &v).unwrap(), 8.0);
    }

    #[test]
    fn division_by_zero() {
        let v = vars(&[]);
        assert!(evaluate_expression("1 / 0", &v).is_err());
    }

    #[test]
    fn sqrt_negative() {
        let v = vars(&[]);
        assert!(evaluate_expression("sqrt(-1)", &v).is_err());
    }

    #[test]
    fn ln_non_positive() {
        let v = vars(&[]);
        assert!(evaluate_expression("ln(0)", &v).is_err());
        assert!(evaluate_expression("ln(-1)", &v).is_err());
    }

    #[test]
    fn complex_expression_rc_low_pass() {
        let v = vars(&[("R", 10_000.0), ("C", 100e-9)]);
        let result = evaluate_expression("fc = 1 / (2 * pi * R * C)", &v).unwrap();
        assert!((result - 159.1549).abs() < 0.01);
    }

    #[test]
    fn ohms_law() {
        let v = vars(&[("I", 0.002), ("R", 10_000.0)]);
        assert_eq!(evaluate_expression("V = I * R", &v).unwrap(), 20.0);
    }

    #[test]
    fn voltage_divider() {
        let v = vars(&[("Vin", 5.0), ("R1", 10_000.0), ("R2", 10_000.0)]);
        assert_eq!(
            evaluate_expression("Vout = Vin * R2 / (R1 + R2)", &v).unwrap(),
            2.5
        );
    }

    #[test]
    fn capacitive_reactance() {
        let v = vars(&[("f", 1000.0), ("C", 100e-9)]);
        let result = evaluate_expression("Xc = 1 / (2 * pi * f * C)", &v).unwrap();
        assert!((result - 1591.549).abs() < 0.1);
    }

    #[test]
    fn lc_resonant_frequency() {
        let v = vars(&[("L", 10e-3), ("C", 100e-9)]);
        let result = evaluate_expression("f0 = 1 / (2 * pi * sqrt(L * C))", &v).unwrap();
        assert!((result - 5032.92).abs() < 0.1);
    }

    #[test]
    fn inverting_op_amp_gain() {
        let v = vars(&[("Rf", 100_000.0), ("Rin", 10_000.0)]);
        assert_eq!(evaluate_expression("Av = -Rf / Rin", &v).unwrap(), -10.0);
    }

    #[test]
    fn non_inverting_op_amp_gain() {
        let v = vars(&[("Rf", 90_000.0), ("Rg", 10_000.0)]);
        assert_eq!(evaluate_expression("Av = 1 + Rf / Rg", &v).unwrap(), 10.0);
    }

    #[test]
    fn led_series_resistor() {
        let v = vars(&[("Vs", 5.0), ("Vf", 2.0), ("I", 0.02)]);
        assert_eq!(evaluate_expression("R = (Vs - Vf) / I", &v).unwrap(), 150.0);
    }

    #[test]
    fn rlc_quality_factor() {
        let v = vars(&[("R", 10.0), ("L", 10e-3), ("C", 100e-9)]);
        let result = evaluate_expression("Q = (1 / R) * sqrt(L / C)", &v).unwrap();
        assert!((result - 31.6227).abs() < 0.01);
    }

    #[test]
    fn efficiency_percent() {
        let v = vars(&[("Pout", 8.0), ("Pin", 10.0)]);
        assert_eq!(
            evaluate_expression("eta = Pout / Pin * 100", &v).unwrap(),
            80.0
        );
    }

    #[test]
    fn parallel_resistors() {
        let v = vars(&[("R1", 10_000.0), ("R2", 10_000.0)]);
        assert_eq!(
            evaluate_expression("R = (R1 * R2) / (R1 + R2)", &v).unwrap(),
            5000.0
        );
    }

    #[test]
    fn capacitor_charge_voltage() {
        let v = vars(&[("Vfinal", 5.0), ("t", 1e-3), ("R", 10_000.0), ("C", 100e-9)]);
        let result = evaluate_expression("Vc = Vfinal * (1 - exp(-t / (R * C)))", &v).unwrap();
        assert!((result - 3.1606).abs() < 0.01);
    }

    #[test]
    fn missing_variable() {
        let v = vars(&[("R", 10_000.0)]);
        assert!(evaluate_expression("V = I * R", &v).is_err());
    }
}
