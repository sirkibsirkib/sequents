use std::fmt;
use ::formulae::*;
use std::collections::HashSet;

use super::UNICODE_MODE;

pub enum StepResult {
	Indeterminate(&'static str, Sequent),
	ValidIfAny(&'static str, Vec<Sequent>, HashSet<char>),
	ValidIfBoth(&'static str, Sequent, Sequent, HashSet<char>),
	Valid(HashSet<char>),
	Invalid(HashSet<char>),
}

pub struct Sequent {
	left: Vec<Formula>,
	right: Vec<Formula>,
}

impl fmt::Debug for Sequent {
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
		write!(f, "{}  {}  {}", &l, if unsafe{UNICODE_MODE} {"⇒"} else {"=>"}, &r)
    }
}


impl Sequent {
	pub fn new(left: Vec<Formula>, right: Vec<Formula>) -> Sequent {
		Sequent {
			left: left,
			right: right,
		}
	}

	fn letters_on_left(&self) -> HashSet<char> {
		let mut s = HashSet::new();
		for f in self.left.iter() {
			if let &Formula::Letter(x) = f {
				s.insert(x);
			}
		}
		s
	}
	
	pub fn certainly_valid(&self) -> bool {
		// if self.right.len() == 0 {
		// 	return true;
		// }
		let mut lefts: Vec<char> = vec![];
		for r in self.right.iter() {
			if r == &Formula::Top {
				return true
			}
		}
		for l in self.left.iter() {
			if let &Formula::Letter(x) = l {
				lefts.push(x);
			} else if &Formula::Bottom == l {
				return true;
			}
		}
		if lefts.len() == 0 {
			return false;
		}
		for r in self.right.iter() {
			if let &Formula::Letter(x) = r {
				if lefts.contains(&x) {
					return true;
				}
			}
		}
		false
	}

	pub fn try_ltop(&mut self) -> bool {
		for i in 0..self.left.len() {
			if let Formula::Top = self.left[i] {
				self.left.remove(i);
				return true;
			}
		}
		false
	}

	pub fn try_rbot(&mut self) -> bool {
		for i in 0..self.right.len() {
			if let Formula::Bottom = self.right[i] {
				self.right.remove(i);
				return true;
			}
		}
		false
	}

	//TODO check for bottoms on left
	
	// Attempts to take one step. returns Some(x) when successful where x is the rule applied
	pub fn step(mut self) -> StepResult {
		use StepResult::*;
		if self.certainly_valid() {
			return Valid(self.letters_on_left());
		}
		if self.try_ltop() {return Indeterminate("ltop", self);}
		if self.try_rbot() {return Indeterminate("rbot", self);}
		if self.try_lneg() {return Indeterminate("lneg", self);}
		if self.try_rneg() {return Indeterminate("rneg", self);}
		if self.try_land() {return Indeterminate("land", self);}
		if self.try_r_or() {return Indeterminate("r_or", self);}
		if let Some((a, b)) = self.try_l_or() {
			return ValidIfBoth("l_or", a, b, self.letters_on_left());
		}
		if let Some((a, b)) = self.try_rand() {
			return ValidIfBoth("rand", a, b, self.letters_on_left());
		}

		//TODO rules 5, 6
		let diam = self.try_diam();
		if !diam.is_empty() {
			return ValidIfAny("diam", diam, self.letters_on_left());
		}
		Invalid(self.letters_on_left())
	}

	pub fn try_lneg(&mut self) -> bool {
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

	pub fn try_rneg(&mut self) -> bool {
		for i in 0..self.right.len() {
			if let Formula::Negation(_) = self.right[i] {
				let n = self.right.remove(i);
				if let Formula::Negation(x) = n {
					self.left.push(*x);
				} else {panic!()}
				return true;
			}
		}
		false
	}

	pub fn try_land(&mut self) -> bool {
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



	pub fn try_r_or(&mut self) -> bool {
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

	pub fn try_l_or(&mut self) -> Option<(Sequent, Sequent)> {
		for i in 0..self.left.len() {
			if let Some(&Formula::Disjunction(ref x, ref y)) = self.left.get(i) {
				let mut lhs = (0..i).chain(i+1..self.left.len())
				.map(|x| self.left.get(x).unwrap().clone())
				.collect::<Vec<_>>();
				return Some((
					Sequent::new({let mut l = lhs.clone(); l.push((**x).clone()); l}, self.right.clone()),
					Sequent::new({let mut l = lhs.clone(); l.push((**y).clone()); l}, self.right.clone()),
				));
			}
		}
		None
	}

	pub fn try_rand(&mut self) -> Option<(Sequent, Sequent)> {
		for i in 0..self.right.len() {
			if let Some(&Formula::Conjunction(ref x, ref y)) = self.right.get(i) {
				let mut rhs = (0..i).chain(i+1..self.right.len())
				.map(|x| self.right.get(x).unwrap().clone())
				.collect::<Vec<_>>();
				return Some((
					Sequent::new(self.left.clone(), {let mut r = rhs.clone(); r.push((**x).clone()); r}),
					Sequent::new(self.left.clone(), {let mut r = rhs.clone(); r.push((**y).clone()); r}),
				));
			}
		}
		None
	}

	pub fn try_diam(&mut self) -> Vec<Sequent> {
		let mut vec = vec![];
		let rhs: Vec<Formula> = self.right.iter()
		.map(|x| if let &Formula::MDiamond(ref q) = x {Some((**q).clone())} else {None})
		.filter(|x| x.is_some())
		.map(|x| x.unwrap())
		.collect::<Vec<_>>();
		for l in self.left.iter() {
			if let &Formula::MDiamond(ref inner) = l {
				let x: Formula = (**inner).clone();
				vec.push(Sequent::new(
					vec![x],
					rhs.clone(),
				))
			}
		}
		vec
	}
}