use super::{part_one_prec_props_fn, part_two_prec_props_fn, Expr, OpTokenProps, Token, Token::*};

#[cfg(test)]
mod expr {

	use super::*;

	#[cfg(test)]
	mod shunting_yard {
		use super::*;

		#[test]
		fn npnmn_p_low() {
			const fn ppf(token: &Token) -> OpTokenProps {
				match token {
					Asterisk => OpTokenProps {
						precedence: 2,
						left_associative: true,
					},
					Plus => OpTokenProps {
						precedence: 1,
						left_associative: true,
					},
					_ => OpTokenProps {
						precedence: 0,
						left_associative: true,
					},
				}
			}

			let expr = Expr(vec![Number(1), Plus, Number(2), Asterisk, Number(3)]);

			assert_eq!(
				expr.shunting_yard(&ppf),
				vec![&Number(1), &Number(2), &Number(3), &Asterisk, &Plus]
			);
		}

		#[test]
		fn npnmn_p_high() {
			const fn ppf(token: &Token) -> OpTokenProps {
				match token {
					Asterisk => OpTokenProps {
						precedence: 1,
						left_associative: true,
					},
					Plus => OpTokenProps {
						precedence: 2,
						left_associative: true,
					},
					_ => OpTokenProps {
						precedence: 0,
						left_associative: true,
					},
				}
			}

			let expr = Expr(vec![Number(1), Plus, Number(2), Asterisk, Number(3)]);

			assert_eq!(
				expr.shunting_yard(&ppf),
				vec![&Number(1), &Number(2), &Plus, &Number(3), &Asterisk]
			);
		}
	}

	#[cfg(test)]
	mod evaluate {
		use super::*;

		#[cfg(test)]
		mod part_one_prec_props_fn {
			use super::{part_one_prec_props_fn, Expr, Token::*};

			#[test]
			fn e71() {
				assert_eq!(
					// 1 + 2 * 3 + 4 * 5 + 6 equals 71
					Expr(vec![
						Number(1),
						Plus,
						Number(2),
						Asterisk,
						Number(3),
						Plus,
						Number(4),
						Asterisk,
						Number(5),
						Plus,
						Number(6),
					])
					.evaluate(&part_one_prec_props_fn),
					Some(71)
				);
			}

			#[test]
			fn e51() {
				assert_eq!(
					// 1 + ( 2 * 3 ) + ( 4 * ( 5 + 6 ) ) becomes 51
					Expr(vec![
						Number(1),
						Plus,
						OpenParen,
						Number(2),
						Asterisk,
						Number(3),
						CloseParen,
						Plus,
						OpenParen,
						Number(4),
						Asterisk,
						OpenParen,
						Number(5),
						Plus,
						Number(6),
						CloseParen,
						CloseParen,
					])
					.evaluate(&part_one_prec_props_fn),
					Some(51)
				);
			}

			#[test]
			fn e26() {
				// 2 * 3 + ( 4 * 5 ) becomes 26
				assert_eq!(
					Expr(vec![
						Number(2),
						Asterisk,
						Number(3),
						Plus,
						OpenParen,
						Number(4),
						Asterisk,
						Number(5),
						CloseParen
					])
					.evaluate(&part_one_prec_props_fn),
					Some(26)
				);
			}

			#[test]
			fn e437() {
				// 5 + ( 8 * 3 + 9 + 3 * 4 * 3 ) becomes 437
				assert_eq!(
					Expr(vec![
						Number(5),
						Plus,
						OpenParen,
						Number(8),
						Asterisk,
						Number(3),
						Plus,
						Number(9),
						Plus,
						Number(3),
						Asterisk,
						Number(4),
						Asterisk,
						Number(3),
						CloseParen
					])
					.evaluate(&part_one_prec_props_fn),
					Some(437)
				);
			}

			#[test]
			fn e12240() {
				// 5 * 9 * ( 7 * 3 * 3 + 9 * 3 + ( 8 + 6 * 4 ) ) becomes 12240
				assert_eq!(
					Expr(vec![
						Number(5),
						Asterisk,
						Number(9),
						Asterisk,
						OpenParen,
						Number(7),
						Asterisk,
						Number(3),
						Asterisk,
						Number(3),
						Plus,
						Number(9),
						Asterisk,
						Number(3),
						Plus,
						OpenParen,
						Number(8),
						Plus,
						Number(6),
						Asterisk,
						Number(4),
						CloseParen,
						CloseParen
					])
					.evaluate(&part_one_prec_props_fn),
					Some(12240)
				);
			}

			#[test]
			fn e13632() {
				// ( ( 2 + 4 * 9 ) * ( 6 + 9 * 8 + 6 ) + 6 ) + 2 + 4 * 2 becomes 13632
				assert_eq!(
					Expr(vec![
						OpenParen,
						OpenParen,
						Number(2),
						Plus,
						Number(4),
						Asterisk,
						Number(9),
						CloseParen,
						Asterisk,
						OpenParen,
						Number(6),
						Plus,
						Number(9),
						Asterisk,
						Number(8),
						Plus,
						Number(6),
						CloseParen,
						Plus,
						Number(6),
						CloseParen,
						Plus,
						Number(2),
						Plus,
						Number(4),
						Asterisk,
						Number(2)
					])
					.evaluate(&part_one_prec_props_fn),
					Some(13632)
				);
			}
		}

		#[cfg(test)]
		mod part_two_prec_props_fn {
			use super::{part_two_prec_props_fn, Expr, Token::*};

			#[test]
			fn e231() {
				assert_eq!(
					// 1 + 2 * 3 + 4 * 5 + 6 becomes 231
					Expr(vec![
						Number(1),
						Plus,
						Number(2),
						Asterisk,
						Number(3),
						Plus,
						Number(4),
						Asterisk,
						Number(5),
						Plus,
						Number(6),
					])
					.evaluate(&part_two_prec_props_fn),
					Some(231)
				);
			}

			#[test]
			fn e51() {
				// 1 + ( 2 * 3 ) + ( 4 * ( 5 + 6 ) ) (still) becomes 51
				assert_eq!(
					Expr(vec![
						Number(1),
						Plus,
						OpenParen,
						Number(2),
						Asterisk,
						Number(3),
						CloseParen,
						Plus,
						OpenParen,
						Number(4),
						Asterisk,
						OpenParen,
						Number(5),
						Plus,
						Number(6),
						CloseParen,
						CloseParen,
					])
					.evaluate(&part_two_prec_props_fn),
					Some(51)
				);
			}

			#[test]
			fn e46() {
				// 2 * 3 + ( 4 * 5 ) becomes 46
				assert_eq!(
					Expr(vec![
						Number(2),
						Asterisk,
						Number(3),
						Plus,
						OpenParen,
						Number(4),
						Asterisk,
						Number(5),
						CloseParen
					])
					.evaluate(&part_two_prec_props_fn),
					Some(46)
				);
			}

			#[test]
			fn e1445() {
				// 5 + ( 8 * 3 + 9 + 3 * 4 * 3 ) becomes 1445
				assert_eq!(
					Expr(vec![
						Number(5),
						Plus,
						OpenParen,
						Number(8),
						Asterisk,
						Number(3),
						Plus,
						Number(9),
						Plus,
						Number(3),
						Asterisk,
						Number(4),
						Asterisk,
						Number(3),
						CloseParen
					])
					.evaluate(&part_two_prec_props_fn),
					Some(1445)
				);
			}

			#[test]
			fn e669060() {
				// 5 * 9 * ( 7 * 3 * 3 + 9 * 3 + ( 8 + 6 * 4 ) ) becomes 669060
				assert_eq!(
					Expr(vec![
						Number(5),
						Asterisk,
						Number(9),
						Asterisk,
						OpenParen,
						Number(7),
						Asterisk,
						Number(3),
						Asterisk,
						Number(3),
						Plus,
						Number(9),
						Asterisk,
						Number(3),
						Plus,
						OpenParen,
						Number(8),
						Plus,
						Number(6),
						Asterisk,
						Number(4),
						CloseParen,
						CloseParen
					])
					.evaluate(&part_two_prec_props_fn),
					Some(669_060)
				);
			}

			#[test]
			fn e23340() {
				// ( ( 2 + 4 * 9 ) * ( 6 + 9 * 8 + 6 ) + 6 ) + 2 + 4 * 2 becomes 23340
				assert_eq!(
					Expr(vec![
						OpenParen,
						OpenParen,
						Number(2),
						Plus,
						Number(4),
						Asterisk,
						Number(9),
						CloseParen,
						Asterisk,
						OpenParen,
						Number(6),
						Plus,
						Number(9),
						Asterisk,
						Number(8),
						Plus,
						Number(6),
						CloseParen,
						Plus,
						Number(6),
						CloseParen,
						Plus,
						Number(2),
						Plus,
						Number(4),
						Asterisk,
						Number(2)
					])
					.evaluate(&part_two_prec_props_fn),
					Some(23340)
				);
			}
		}
	}
}
