use std::fmt;

#[derive(Clone)]
pub enum Formula {
	Letter(u32),
	Negation(Box<Formula>),
	Conjunction(Box<Formula>, Box<Formula>),
	Disjunction(Box<Formula>, Box<Formula>),
	MDiamond(Box<Formula>),
	
	//not allowed inside
	MBox(Box<Formula>),
	Implication(Box<Formula>, Box<Formula>),
}
impl fmt::Debug for Formula {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	
		use Formula::*;
		match self {
			&Letter(x) => 					write!(f, "{:?}", &x),
			&Negation(ref x) => 			write!(f, "-{:?}", &x),
			&Conjunction(ref x, ref y) => 	write!(f, "({:?}&{:?})", &x, &y),
			&Disjunction(ref x, ref y) => 	write!(f, "({:?}V{:?})", &x, &y),
			&MDiamond(ref x) => 			write!(f, "<>{:?}", &x),
			&MBox(ref x) => 				write!(f, "[]{:?}", &x),
			&Implication(ref x, ref y) => 	write!(f, "({:?}->{:?})", &x, &y),
		}
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
		let mut lefts: Vec<u32> = vec![];
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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
		use Formula::*;
		let x = Implication(Box::new(Letter(0)), Box::new(Letter(1)));
		println!("{:?}", &x);
    }
}
