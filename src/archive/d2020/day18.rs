#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Token {
	Number(u64),
	OpenParen,
	CloseParen,
	Asterisk,
	Plus,
}

impl core::fmt::Display for Token {
	fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
		match self {
			Token::Number(n) => write!(f, "{n}"),
			Token::Plus => write!(f, "+"),
			Token::Asterisk => write!(f, "*"),
			Token::OpenParen => write!(f, "("),
			Token::CloseParen => write!(f, ")"),
		}
	}
}

#[derive(Debug)]
pub struct Expr(Vec<Token>);

impl core::fmt::Display for Expr {
	fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
		let disp = self
			.0
			.iter()
			.map(std::string::ToString::to_string)
			.collect::<Vec<String>>()
			.join(" ");

		write!(f, "{disp}")
	}
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq)]
struct OpTokenProps {
	precedence: usize,
	left_associative: bool,
}

impl Expr {
	fn process_rpn(output_queue: &[&Token]) -> Option<u64> {
		let mut final_stack: Vec<u64> = Vec::new();

		for token in output_queue {
			match token {
				Token::Number(n) => final_stack.push(*n),
				Token::Asterisk => {
					let a = final_stack.pop().unwrap();
					let b = final_stack.pop().unwrap();
					final_stack.push(a * b);
				}
				Token::Plus => {
					let a = final_stack.pop().unwrap();
					let b = final_stack.pop().unwrap();
					final_stack.push(a + b);
				}
				_ => panic!("panik!"),
			}
		}

		final_stack.pop()
	}

	/// Perform the Shunting Yard Algorithm on the stream of tokens `tokens` using a precedence map function.
	///
	/// # Arguments
	///
	/// - `prec_props_fn` - an ideally `const fn` that takes a token and returns its precedence properties;
	///   will only be called on operators.
	fn shunting_yard(&self, prec_props_fn: &dyn Fn(&Token) -> OpTokenProps) -> Vec<&Token> {
		let mut output_queue: Vec<&Token> = vec![];
		let mut operator_stack: Vec<&Token> = vec![];

		let tokens: &[Token] = &self.0;

		for token in tokens {
			match token {
				Token::Number(_) => output_queue.push(token),
				// no impl for functions
				Token::OpenParen => operator_stack.push(token),
				Token::CloseParen => {
					while operator_stack.last().unwrap() != &&Token::OpenParen {
						output_queue.push(operator_stack.pop().unwrap());
					}

					if let Some(Token::OpenParen) = operator_stack.last() {
						let _ = operator_stack.pop();
					}

					// no function tokens
				}
				// token is an operator!
				_ => {
					fn should_pop(
						token: &Token,
						last: Option<&&Token>,
						prec_props_fn: &dyn Fn(&Token) -> OpTokenProps,
					) -> bool {
						match last {
							Some(Token::Asterisk | Token::Plus | Token::OpenParen) => {
								let token_props = prec_props_fn(token);
								let last_props = prec_props_fn(last.unwrap());

								// While:
								// - There is an operator at the top of the stack, AND
								// - The operator at the top of the operator stack has greater precedence than the token, OR
								//   - The operator at the top
								((last_props.precedence > token_props.precedence)
									|| (last_props.precedence == token_props.precedence
										&& token_props.left_associative))
									&& last != Some(&&Token::OpenParen)
							}
							_ => false,
						}
					}

					while should_pop(token, operator_stack.last(), prec_props_fn) {
						output_queue.push(operator_stack.pop().unwrap());
					}

					operator_stack.push(token);
				}
			}
		}

		while let Some(element) = operator_stack.pop() {
			output_queue.push(element);
		}

		assert!(operator_stack.is_empty());

		output_queue
	}

	fn evaluate(&self, prec_props_fn: &dyn Fn(&Token) -> OpTokenProps) -> Option<u64> {
		let rpn = self.shunting_yard(prec_props_fn);
		Self::process_rpn(&rpn)
	}
}

const fn part_one_prec_props_fn(token: &Token) -> OpTokenProps {
	match *token {
		Token::Asterisk | Token::Plus => OpTokenProps {
			precedence: 1,
			left_associative: true,
		},
		_ => OpTokenProps {
			precedence: 0,
			left_associative: true,
		},
	}
}

const fn part_two_prec_props_fn(token: &Token) -> OpTokenProps {
	match *token {
		Token::Asterisk => OpTokenProps {
			precedence: 1,
			left_associative: true,
		},
		Token::Plus => OpTokenProps {
			precedence: 2,
			left_associative: true,
		},
		_ => OpTokenProps {
			precedence: 0,
			left_associative: true,
		},
	}
}

#[cfg(test)]
mod tests;

pub type Intermediate = Vec<Expr>;
type Output = u64;

pub fn parse(input: &str) -> Result<Intermediate, core::convert::Infallible> {
	let mut exprs: Vec<Expr> = Vec::new();

	for line in input.lines() {
		let mut number: Option<u32> = None;

		let mut tokens: Vec<Token> = vec![];

		for c in line.chars() {
			if c.is_ascii_digit() {
				// TODO Not strictly necessary since we only have single digits
				if let Some(n) = number {
					number = Some(n * 10 + c.to_digit(10).expect("invalid digit"));
				} else {
					number = Some(c.to_digit(10).expect("invalid digit"));
				}
			} else {
				if let Some(n) = number {
					tokens.push(Token::Number(u64::from(n)));
					number = None;
				}

				match c {
					'(' => tokens.push(Token::OpenParen),
					')' => tokens.push(Token::CloseParen),

					' ' => {}
					'+' => tokens.push(Token::Plus),
					'*' => tokens.push(Token::Asterisk),
					_ => unimplemented!(),
				}
			}
		}

		if let Some(n) = number {
			tokens.push(Token::Number(u64::from(n)));
		}

		exprs.push(Expr(tokens));
	}

	Ok(exprs)
}

pub fn part_one(exprs: &Intermediate) -> Option<Output> {
	exprs
		.iter()
		.map(|expr| expr.evaluate(&part_one_prec_props_fn))
		.sum()
}

pub fn part_two(exprs: &Intermediate) -> Option<Output> {
	exprs
		.iter()
		.map(|expr| expr.evaluate(&part_two_prec_props_fn))
		.sum()
}

crate::generate_solver!(solve, =>, self);
