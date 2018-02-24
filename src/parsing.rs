use ::formulae::*;

fn has_redundant_brackets(s: &str) -> bool {
	if s.len() < 2
	|| !s.starts_with('(')
	|| !s.ends_with(')') {
		return false;
	}
	let mut depth = 1;
	for c in s[1..s.len()-1].chars() {
		match c {
			'(' => depth += 1,
			')' => {
				depth -= 1;
				if depth == 0 {
					return false;
				}
			},
			_ => (),
		}
	}
	true
}

pub fn parse(mut s: &str) -> Option<Formula> {
	while has_redundant_brackets(s) {
		s = &s[1..s.len()-1];
	}
	use Formula::*;
	if s.chars().count() == 1 {
		return match s.chars().next() {
			Some('⊤') => Some(Top),
			Some('⊥') => Some(Bottom),
			Some(x) if x.is_lowercase() => Some(Letter(x)),
			_ => None,
		}
	}
	let mut depth = 0;
	let mut best = FormulaType::None;
	let mut best_index = 1337;
	for (i, c) in s.char_indices() {
		if depth > 0 {
			match c {
				'(' => depth += 1,
				')' => depth -= 1,
				_ => (),
			}
		} else {
			match c {
				'(' => depth += 1,
				')' => return None,
				'¬' if i == 0 => {
					if best.bind_strength() > FormulaType::Negation.bind_strength() {
						best = FormulaType::Negation;
						best_index = i;
					}
				}
				'◇' if i == 0 => {
					if best.bind_strength() > FormulaType::MDiamond.bind_strength() {
						best = FormulaType::MDiamond;
						best_index = i;
					}
				}
				'□' if i == 0 => {
					if best.bind_strength() > FormulaType::MBox.bind_strength() {
						best = FormulaType::MBox;
						best_index = i;
					}
				}
				'→' => {
					if best.bind_strength() > FormulaType::Implication.bind_strength() {
						best = FormulaType::Implication;
						best_index = i;
					}
				}
				'∧' => {
					if best.bind_strength() > FormulaType::Conjunction.bind_strength() {
						best = FormulaType::Conjunction;
						best_index = i;
					}
				}
				'∨' => {
					if best.bind_strength() > FormulaType::Disjunction.bind_strength() {
						best = FormulaType::Disjunction;
						best_index = i;
					}
				}
				_ => (),
			}
		}
	}
	match best {
		FormulaType::None => None,
		FormulaType::Negation => {
			parse(&s[best_index + '¬'.len_utf8()..])
			.map(|x| Negation(Box::new(x)))
		},
		FormulaType::MDiamond => {
			parse(&s[best_index + '◇'.len_utf8()..])
			.map(|x| MDiamond(Box::new(x)))
		},
		FormulaType::MBox => {
			parse(&s[best_index + '□'.len_utf8()..])
			.map(|x| MBox(Box::new(x)))
		},
		FormulaType::Implication => {
			let a = parse(&s[..best_index]);
			let b = parse(&s[best_index + '→'.len_utf8()..]);
			if let (Some(x), Some(y)) = (a,b) {
				Some(Implication(Box::new(x), Box::new(y)))
			} else {None}
		},
		FormulaType::Conjunction => {
			let a = parse(&s[..best_index]);
			let b = parse(&s[best_index + '∧'.len_utf8()..]);
			if let (Some(x), Some(y)) = (a,b) {
				Some(Conjunction(Box::new(x), Box::new(y)))
			} else {None}
		},
		FormulaType::Disjunction => {
			let a = parse(&s[..best_index]);
			let b = parse(&s[best_index + '∨'.len_utf8()..]);
			if let (Some(x), Some(y)) = (a,b) {
				Some(Disjunction(Box::new(x), Box::new(y)))
			} else {None}
		},
		_ => None,
	}
}

pub fn to_unicode(s: String) -> String {
	s
	.replace("->", "→")
	.replace("=>", "⇒")
	.replace("-", "¬")
	.replace("~", "¬")
	.replace("/\\", "∧")
	.replace("&", "∧")
	.replace("V", "∨")
	.replace("\\/", "∨")
	.replace("<>", "◇")
	.replace("[]", "□")
	.replace(" ", "")
	.replace("T", "⊤")
	.replace("F", "⊥")
}