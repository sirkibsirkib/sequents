use std::fmt;
use ::formulae::*;

pub enum StepResult {
	Indeterminate(u8, Sequent),
	ValidIfAny(u8, Vec<Sequent>),
	ValidIfBoth(u8, Sequent, Sequent),
	Valid(Sequent),
	Invalid(Sequent),
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
		write!(f, "{}  ⇒  {}", &l, &r)
    }
	
}


impl Sequent {
	pub fn new(left: Vec<Formula>, right: Vec<Formula>) -> Sequent {
		Sequent {
			left: left,
			right: right,
		}
	}
	
	pub fn certainly_valid(&self) -> bool {
		let mut lefts: Vec<char> = vec![];
		for l in self.left.iter() {
			if let &Formula::Letter(x) = l {
				lefts.push(x);
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
	
	// Attempts to take one step. returns Some(x) when successful where x is the rule applied
	pub fn step(mut self) -> StepResult {
		use StepResult::*;
		if self.certainly_valid() {
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

	pub fn try_rule_5(&mut self) -> Option<(Sequent, Sequent)> {
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

	pub fn try_rule_6(&mut self) -> Option<(Sequent, Sequent)> {
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

	pub fn rule_diamond(&mut self) -> Vec<Sequent> {
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
				vec.push(Sequent::new(
					vec![x],
					rhs.clone(),
				))
			}
		}
		vec
	}
}