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
	Letter(u8),
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
		if my_type == their_type {
			return false;
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
			&Letter(x) => 					{f.push_str(&format!("{}", (x + 'a' as u8) as char));},
			&Negation(ref x) => 			{f.push('-'); x.repr(f, my_type);},
			&Conjunction(ref x, ref y) => 	{x.repr(f, my_type); f.push('&'); y.repr(f, my_type);},
			&Disjunction(ref x, ref y) => 	{x.repr(f, my_type); f.push('V'); y.repr(f, my_type);},
			&MDiamond(ref x) => 			{f.push('<'); f.push('>'); x.repr(f, my_type);},
			&MBox(ref x) => 				{f.push('['); f.push(']'); x.repr(f, my_type);},
			&Implication(ref x, ref y) => 	{x.repr(f, my_type); f.push('-'); f.push('>'); y.repr(f, my_type);},
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
		let mut lefts: Vec<u8> = vec![];
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
	
	pub fn step(&mut self) -> Validity {
		if self.validity() == Validity::Valid {
			return Validity::Valid;
		}
		//TODO
		Validity::Indeterminate
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
		let x = Implication(
			Box::new(Disjunction(
				Box::new(Letter(0)),
				Box::new(MBox(Box::new(Letter(1)))),
			)),
			Box::new(Letter(1)),
		);
		println!("{:?}", &x);
		let x = preprocess(x);
		println!("{:?}", &x);
    }
}
