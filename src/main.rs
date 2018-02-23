use std::env;

mod parsing;
mod formulae;
mod sequents;
mod proofs;


static mut UNICODE_MODE: bool = false;  

use parsing::parse;
use formulae::Formula;
use sequents::{Sequent, StepResult};
use proofs::Proof;


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


fn clean(s: String) -> String {
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
}

fn input() -> Option<Formula> {
	let mut args = String::new();
	for a in env::args().skip(1) {
		if a == "--unicode" {
			unsafe {
				UNICODE_MODE = true;
			}
		} else {
			args.push_str(&a);
		}
	}
	args = clean(args);
	parse(&mut args)
}

fn main() {
	
	if let Some(y) = input() {
		println!("Given: {:?}", &y);
		let x = preprocess(y.clone());
		if x != y {
			println!("...preprocessed to: {:?}", &x);
		}
		drop(y);
		let m = Sequent::new(
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