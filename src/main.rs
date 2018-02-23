use std::fmt;
use std::env;

#[derive(Eq, PartialEq, Copy, Clone)]

pub enum FormulaType {
	Letter, Negation, Conjunction, Disjunction,
	MDiamond, MBox, Implication, None
}

impl FormulaType {
	pub fn bind_strength(&self) -> u8 {
		use FormulaType::*;
		match self {
			&None => 99,
			&Letter => 99, 
			&Negation => 3,
			&Conjunction => 2,
			&Disjunction => 2,
			&MDiamond => 3,
			&MBox => 3,
			&Implication => 1,
		}
	}
}


#[derive(Clone, Eq, PartialEq)]
pub enum Formula {
	Letter(char),
	Negation(Box<Formula>),
	Conjunction(Box<Formula>, Box<Formula>),
	Disjunction(Box<Formula>, Box<Formula>),
	MDiamond(Box<Formula>),
	
	//not allowed inside
	MBox(Box<Formula>),
	Implication(Box<Formula>, Box<Formula>),
}
impl Formula {
	fn get_type(&self) -> FormulaType {
		use Formula::*;
		match self {
			&Letter(_) => 			FormulaType::Letter,
			&Negation(_) => 		FormulaType::Negation,
			&Conjunction(_,_) => 	FormulaType::Conjunction,
			&Disjunction(_,_) => 	FormulaType::Disjunction,
			&MDiamond(_) => 		FormulaType::MDiamond,
			&MBox(_) => 			FormulaType::MBox,
			&Implication(_,_) => 		FormulaType::Implication,
		}
	}

	fn need_parens(my_type: FormulaType, their_type: FormulaType) -> bool {
		if my_type == their_type{
			return my_type == FormulaType::Implication;
		} else if their_type == FormulaType::None {
			return false;
		}
		their_type.bind_strength() > my_type.bind_strength()
	}

    fn repr(&self, f: &mut String, inside_type: FormulaType) {
    	let my_type = self.get_type();
		let parens = Formula::need_parens(my_type, inside_type);
		if parens {f.push('(');}
		use Formula::*;
		match self {
			&Letter(x) => 					{f.push_str(&format!("{}", x));},
			&Negation(ref x) => 			{f.push('¬'); x.repr(f, my_type);},
			&Conjunction(ref x, ref y) => 	{x.repr(f, my_type); f.push('∧'); y.repr(f, my_type);},
			&Disjunction(ref x, ref y) => 	{x.repr(f, my_type); f.push('∨'); y.repr(f, my_type);},
			&MDiamond(ref x) => 			{f.push('◇'); x.repr(f, my_type);},
			&MBox(ref x) => 				{f.push('□'); x.repr(f, my_type);},
			&Implication(ref x, ref y) => 	{x.repr(f, my_type); f.push('→'); y.repr(f, my_type);},
		};
		if parens {f.push(')');}
    }
}

impl fmt::Debug for Formula {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut s = String::new();
		self.repr(&mut s, FormulaType::None);
		write!(f, "{}", &s)
    }
}


pub fn preprocess(f: Formula) -> Formula {
	use Formula::*;
	match f {
		Letter(x) => Letter(x),
		Negation(a) => Negation(Box::new(preprocess(*a))),
		Conjunction(a, b) => Conjunction(Box::new(preprocess(*a)), Box::new(preprocess(*b))),
		Disjunction(a, b) => Disjunction(Box::new(preprocess(*a)), Box::new(preprocess(*b))),
		MDiamond(a) => MDiamond(Box::new(preprocess(*a))),
		MBox(a) => Negation(Box::new(MDiamond(Box::new(Negation(Box::new(preprocess(*a))))))),
		Implication(a, b) => Disjunction(
			Box::new(Negation(Box::new(preprocess(*a)))),
			Box::new(preprocess(*b))
		),
	}
}

pub struct MetaImpl {
	left: Vec<Formula>,
	right: Vec<Formula>,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Validity {
	Indeterminate,
	Valid,
	Invalid,
}

pub enum StepResult {
	Indeterminate(u8, MetaImpl),
	ValidIfAny(u8, Vec<MetaImpl>),
	ValidIfBoth(u8, MetaImpl, MetaImpl),
	Valid(MetaImpl),
	Invalid(MetaImpl),
}

impl MetaImpl {
	pub fn new(left: Vec<Formula>, right: Vec<Formula>) -> MetaImpl {
		MetaImpl {
			left: left,
			right: right,
		}
	}
	
	pub fn validity(&self) -> Validity {
		let mut lefts: Vec<char> = vec![];
		for l in self.left.iter() {
			if let &Formula::Letter(x) = l {
				lefts.push(x);
			}
		}
		if lefts.len() == 0 {return Validity::Indeterminate;}
		for r in self.right.iter() {
			if let &Formula::Letter(x) = r {
				if lefts.contains(&x) {
					return Validity::Valid;
				}
			}
		}
		Validity::Indeterminate
	}
	
	// Attempts to take one step. returns Some(x) when successful where x is the rule applied
	pub fn step(mut self) -> StepResult {
		use StepResult::*;
		if self.validity() == Validity::Valid {
			return Valid(self);
		}
		if self.try_rule_1() {return Indeterminate(1, self);}
		if self.try_rule_2() {return Indeterminate(2, self);}
		if self.try_rule_3() {return Indeterminate(3, self);}
		if self.try_rule_4() {return Indeterminate(4, self);}
		if let Some((a, b)) = self.try_rule_5() {
			return ValidIfBoth(5, a, b);
		}
		if let Some((a, b)) = self.try_rule_6() {
			return ValidIfBoth(6, a, b);
		}

		//TODO rules 5, 6
		let r7 = self.rule_diamond();
		if !r7.is_empty() {
			return ValidIfAny(7, r7);
		}
		Invalid(self)
	}

	pub fn try_rule_1(&mut self) -> bool {
		for i in 0..self.left.len() {
			if let Formula::Negation(_) = self.left[i] {
				let n = self.left.remove(i);
				if let Formula::Negation(x) = n {
					self.right.push(*x);
				} else {panic!()}
				return true;
			}
		}
		false
	}

	pub fn try_rule_2(&mut self) -> bool {
		for i in 0..self.right.len() {
			if if let Formula::Negation(_) = self.right[i] {true} else {false} {
				let n = self.right.remove(i);
				if let Formula::Negation(x) = n {
					self.left.push(*x);
				} else {panic!()}
				return true;
			}
		}
		false
	}

	pub fn try_rule_3(&mut self) -> bool {
		for i in 0..self.left.len() {
			if let Formula::Conjunction(_,_) = self.left[i] {
				let n = self.left.remove(i);
				if let Formula::Conjunction(x, y) = n {
					self.left.insert(i, *x);
					self.left.insert(i+1, *y);
				} else {panic!()}
				return true;
			}
		}
		false
	}



	pub fn try_rule_4(&mut self) -> bool {
		for i in 0..self.right.len() {
			if let Formula::Disjunction(_,_) = self.right[i] {
				let n = self.right.remove(i);
				if let Formula::Disjunction(x, y) = n {
					self.right.insert(i, *x);
					self.right.insert(i+1, *y);
				} else {panic!()}
				return true;
			}
		}
		false
	}

	pub fn try_rule_5(&mut self) -> Option<(MetaImpl, MetaImpl)> {
		for i in 0..self.left.len() {
			if let Some(&Formula::Disjunction(ref x, ref y)) = self.left.get(i) {
				let mut lhs = (0..i).chain(i+1..self.left.len())
				.map(|x| self.left.get(x).unwrap().clone())
				.collect::<Vec<_>>();
				return Some((
					MetaImpl::new({let mut l = lhs.clone(); l.push((**x).clone()); l}, self.right.clone()),
					MetaImpl::new({let mut l = lhs.clone(); l.push((**y).clone()); l}, self.right.clone()),
				));
			}
		}
		None
	}

	pub fn try_rule_6(&mut self) -> Option<(MetaImpl, MetaImpl)> {
		for i in 0..self.right.len() {
			if let Some(&Formula::Conjunction(ref x, ref y)) = self.right.get(i) {
				let mut rhs = (0..i).chain(i+1..self.right.len())
				.map(|x| self.right.get(x).unwrap().clone())
				.collect::<Vec<_>>();
				return Some((
					MetaImpl::new(self.left.clone(), {let mut r = rhs.clone(); r.push((**x).clone()); r}),
					MetaImpl::new(self.left.clone(), {let mut r = rhs.clone(); r.push((**y).clone()); r}),
				));
			}
		}
		None
	}

	pub fn rule_diamond(&mut self) -> Vec<MetaImpl> {
		let mut vec = vec![];
		let rhs: Vec<Formula> = self.right.iter()
		.map(|x| if let &Formula::MDiamond(ref q) = x {Some((**q).clone())} else {None})
		.filter(|x| x.is_some())
		.map(|x| x.unwrap())
		.collect::<Vec<_>>();
		if rhs.is_empty() {
			return vec;
		}
		for l in self.left.iter() {
			if let &Formula::MDiamond(ref inner) = l {
				let x: Formula = (**inner).clone();
				vec.push(MetaImpl::new(
					vec![x],
					rhs.clone(),
				))
			}
		}
		vec
	}
}

impl fmt::Debug for MetaImpl {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut l = String::new();
		for x in self.left.iter() {
			if !l.is_empty() {l.push(',');}
			l.push_str(&format!("{:?}", x));
		}
		let mut r = String::new();
		for x in self.right.iter() {
			if !r.is_empty() {r.push(',');}
			r.push_str(&format!("{:?}", x));
		}
		write!(f, "{}  ⇒  {}", &l, &r)
    }
	
}

pub struct Proof {
	steps: Vec<String>,
	proof_result: ProofResult, 
	valid: bool,
}

pub enum ProofResult {
	Valid,
	Invalid,
	AnyValid(Vec<Proof>),
	BothValid(Box<Proof>, Box<Proof>),
}


impl Proof {
	pub fn new(mut m: MetaImpl) -> Proof {
		let mut steps = vec![format!("• prove : {:?}", &m)];
		loop {
			use StepResult::*;
			match m.step() {
				Indeterminate(r, a) => {
					steps.push(format!("  rule {}: {:?}", r, &a));
					m = a;
				},
				Valid(_) => {
					steps.push(format!("  valid!"));
					return Proof {
						steps: steps,
						proof_result: ProofResult::Valid,
						valid: true,
					}
				},
				Invalid(_) => {
					steps.push(format!("  invalid!"));
					return Proof {
						steps: steps,
						proof_result: ProofResult::Invalid,
						valid: false,
					}
				},
				ValidIfAny(r, v) => {
					let proofs = v.into_iter().map(|x| Proof::new(x)).collect::<Vec<_>>();
					let valid = proofs.iter().fold(false, |a,b| a||b.valid);
					steps.push(format!("  rule {}: valid if any... ({})", r, if valid {"valid"} else {"invalid"}));
					return Proof {
						steps: steps,
						proof_result: ProofResult::AnyValid(proofs),
						valid: valid,
					}
				},
				ValidIfBoth(r, a, b) => {
					let a = Box::new(Proof::new(a));
					let b = Box::new(Proof::new(b));
					let valid = a.valid || b.valid;
					steps.push(format!("  rule {}: valid if both... ({})", r, if valid {"valid"} else {"invalid"}));
					return Proof {
						steps: steps,
						proof_result: ProofResult::BothValid(a, b),
						valid: valid,
					}
				},
			}
		}
	}

	pub fn print(&self, depth: u8) {
		for s in self.steps.iter() {
			for _ in 0..depth {print!("    ")}
			println!("{}", &s)
		}
		use ProofResult::*;
		match self.proof_result {
			Valid => (),
			Invalid => (),
			AnyValid(ref v) => {
				for q in v.iter() {
					q.print(depth+1);
				}
			},
			BothValid(ref a, ref b) => {
				a.print(depth+1);
				b.print(depth+1);
			},
		} 
	}
}

fn strip_redundant_brackets(s: &mut String) -> bool {
	if s.len() < 2
	|| s.chars().next().unwrap() != '('
	|| s.chars().last().unwrap() != ')' {
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
	*s = s[1..s.len()-1].to_owned();
	true
}

fn parse(s: &mut String) -> Option<Formula> {
	use Formula::*;
	while strip_redundant_brackets(s) {}
	if s.len() == 1 {
		if let Some(c) = s.chars().next() {
			if c.is_alphabetic() {
				return Some(Letter(c));
			}
		}
		return None;
	}
	let mut depth = 0;
	let mut best = FormulaType::None;
	let mut best_index = 1337;
	for (i, c) in s.chars().enumerate() {
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
			parse(&mut s.chars().skip(1).collect())
			.map(|x| Negation(Box::new(x)))
		},
		FormulaType::MDiamond => {
			parse(&mut s.chars().skip(1).collect())
			.map(|x| MDiamond(Box::new(x)))
		},
		FormulaType::MBox => {
			parse(&mut s.chars().skip(1).collect())
			.map(|x| MBox(Box::new(x)))
		},
		FormulaType::Implication => {
			let a = parse(&mut s.chars().take(best_index).collect());
			let b = parse(&mut s.chars().skip(best_index+1).collect());
			if let (Some(x), Some(y)) = (a,b) {
				Some(Implication(Box::new(x), Box::new(y)))
			} else {None}
		},
		FormulaType::Conjunction => {
			let a = parse(&mut s.chars().take(best_index).collect());
			let b = parse(&mut s.chars().skip(best_index+1).collect());
			if let (Some(x), Some(y)) = (a,b) {
				Some(Conjunction(Box::new(x), Box::new(y)))
			} else {None}
		},
		FormulaType::Disjunction => {
			let a = parse(&mut s.chars().take(best_index).collect());
			let b = parse(&mut s.chars().skip(best_index+1).collect());
			if let (Some(x), Some(y)) = (a,b) {
				Some(Disjunction(Box::new(x), Box::new(y)))
			} else {None}
		},
		_ => None,
	}
}

fn input() -> Option<Formula> {
	let mut args: String = env::args().skip(1).collect();
	args = args
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
	.replace(" ", "");
	parse(&mut args)
}

fn main() {
	if let Some(mut y) = input() {
		println!("Given: {:?}", &y);
		let x = preprocess(y.clone());
		if x != y {
			println!(" = {:?}", &x);
		}
		drop(y);
		let m = MetaImpl::new(
			vec![],
			vec![x],
		); 
		println!("starting with: {:?}...", &m);
		let p = Proof::new(m);
		p.print(0);
	} else {
		println!("Failed to recognize forumla input args!");
	}
}