use std::fmt;

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum FormulaType {
	Letter, Negation, Conjunction, Disjunction, MDiamond, MBox, Implication, None
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


#[derive(Clone)]
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
	pub fn step(&mut self) -> Option<u8> {
		if self.validity() == Validity::Valid {
			return None;
		}
		if self.try_rule_1() {return Some(1)};
		if self.try_rule_2() {return Some(2)};
		if self.try_rule_3() {return Some(3)};
		if self.try_rule_4() {return Some(4)};
		//TODO
		None
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
		write!(f, "{} ⇒ {}", &l, &r)
    }
	
}

// pub fn parse(text: &str) -> Option<Formula> {
// 	use Formula::*;
// 	for i in 0..text.len() {
// 		if i == '-' {
// 			return Formula::Negation(Box::new(parse()))
// 		}
// 	}
// }

#[cfg(test)]
mod tests {
	use super::*;
    #[test]
    fn it_works() {
		use Formula::*;

		let mut m = MetaImpl::new(
			vec![],
			vec![
				Negation(Box::new(
					Conjunction(
						Box::new(Negation(Box::new(Letter('p')))),
						Box::new(Negation(Box::new(Letter('r')))),
					),
				)),
				Disjunction(
					Box::new(Letter('r')),
					Box::new(Letter('p')),
				),
				Negation(Box::new(Letter('q'))),
			],
		); 
		println!("starting with: {:?}", &m);
		let mut step = Some(99);
		while let Some(_) = step {
			step = m.step();
			println!("{:?}\t{:?}", &step, &m);
		}
    }
}




// let x = Implication(
// 	Box::new(Disjunction(
// 		Box::new(Letter(0)),
// 		Box::new(MBox(Box::new(Letter(1)))),
// 	)),
// 	Box::new(Implication(
// 		Box::new(Letter(1)),
// 		Box::new(MBox(Box::new(MDiamond(Box::new(Letter(2)))),))
// 	)),
// );
