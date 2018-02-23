
use std::fmt;

use super::UNICODE_MODE;

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum FormulaType {
	Letter, Negation, Conjunction, Disjunction,
	MDiamond, MBox, Implication, None
}

impl FormulaType {
	pub fn bind_strength(&self) -> u8 {
		use self::FormulaType::*;
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
	
	//not allowed inside the sequent body
	MBox(Box<Formula>),
	Implication(Box<Formula>, Box<Formula>),
}
impl Formula {
	fn get_type(&self) -> FormulaType {
		use self::Formula::*;
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



    fn repr_ascii(&self, f: &mut String, inside_type: FormulaType) {
    	let my_type = self.get_type();
		let parens = Formula::need_parens(my_type, inside_type);
		if parens {f.push('(');}
		use Formula::*;
		match self {
			&Letter(x) => 					{f.push_str(&format!("{}", x));},
			&Negation(ref x) => 			{f.push('-'); x.repr_ascii(f, my_type);},
			&Conjunction(ref x, ref y) => 	{x.repr_ascii(f, my_type); f.push('&'); y.repr_ascii(f, my_type);},
			&Disjunction(ref x, ref y) => 	{x.repr_ascii(f, my_type); f.push('V'); y.repr_ascii(f, my_type);},
			&MDiamond(ref x) => 			{f.push_str("<>"); x.repr_ascii(f, my_type);},
			&MBox(ref x) => 				{f.push_str("[]"); x.repr_ascii(f, my_type);},
			&Implication(ref x, ref y) => 	{x.repr_ascii(f, my_type); f.push_str("->"); y.repr_ascii(f, my_type);},
		};
		if parens {f.push(')');}
    }

    fn repr_unicode(&self, f: &mut String, inside_type: FormulaType) {
    	let my_type = self.get_type();
		let parens = Formula::need_parens(my_type, inside_type);
		if parens {f.push('(');}
		use Formula::*;
		match self {
			&Letter(x) => 					{f.push_str(&format!("{}", x));},
			&Negation(ref x) => 			{f.push('¬'); x.repr_unicode(f, my_type);},
			&Conjunction(ref x, ref y) => 	{x.repr_unicode(f, my_type); f.push('∧'); y.repr_unicode(f, my_type);},
			&Disjunction(ref x, ref y) => 	{x.repr_unicode(f, my_type); f.push('∨'); y.repr_unicode(f, my_type);},
			&MDiamond(ref x) => 			{f.push('◇'); x.repr_unicode(f, my_type);},
			&MBox(ref x) => 				{f.push('□'); x.repr_unicode(f, my_type);},
			&Implication(ref x, ref y) => 	{x.repr_unicode(f, my_type); f.push('→'); y.repr_unicode(f, my_type);},
		};
		if parens {f.push(')');}
    }
}

impl fmt::Debug for Formula {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut s = String::new();
		if unsafe{UNICODE_MODE} {
    		self.repr_unicode(&mut s, FormulaType::None);
    	} else {
    		self.repr_ascii(&mut s, FormulaType::None);
    	}
		write!(f, "{}", &s)
    }
}
