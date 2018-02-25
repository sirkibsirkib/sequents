use std::env;

mod parsing;
mod formulae;
mod sequents;
mod proofs;
mod models;

static mut UNICODE_MODE: bool = false;

use parsing::{to_unicode, parse};
use formulae::Formula;
use sequents::{Sequent, StepResult};
use proofs::Proof;
use models::ModelBuilder;


pub fn preprocess(f: Formula) -> Formula {
	use Formula::*;
	match f {
		//stops
		Top => Top,
		Bottom => Bottom,
		Letter(x) => Letter(x),

		//go deeper, untouched
		Negation(a) => Negation(Box::new(preprocess(*a))),
		Conjunction(a, b) => Conjunction(Box::new(preprocess(*a)), Box::new(preprocess(*b))),
		Disjunction(a, b) => Disjunction(Box::new(preprocess(*a)), Box::new(preprocess(*b))),
		MDiamond(a) => MDiamond(Box::new(preprocess(*a))),

		//go deeper, rewritten
		MBox(a) => Negation(Box::new(MDiamond(Box::new(Negation(Box::new(preprocess(*a))))))),
		Implication(a, b) => Disjunction(
			Box::new(Negation(Box::new(preprocess(*a)))),
			Box::new(preprocess(*b))
		),
	}
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
	args = to_unicode(args);
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
		if p.valid() {
			println!("VALID!");
		} else {
			//find counterexample
			println!("INVALID!\nCounter-example:");
			let mut builder = ModelBuilder::new();
			let mut next_avail_world = 2;
			build_counter_model(1, &p, &mut next_avail_world, &mut builder);
			println!("{:?}", builder.finalize());
		}
	} else {
		println!("Failed to recognize forumla input args!");
	}
}

fn build_counter_model(curr_world: u32, proof: &Proof, next_avail_world: &mut u32, builder: &mut ModelBuilder) {
	//step 1: ensure current world has needed valuations
	for letter in proof.true_here() {
		builder.set_true_in(curr_world, *letter);
	}
	use proofs::ProofResult::*;
	match proof.proof_result() {
		&Valid => (),
		&Invalid => (),
		&AnyValid(ref proofs) => {
			//pick the shallowest sub-proof that is valid!
			let mut min_depth = 999999999;
			let mut best = None;
			for p in proofs.iter() { //.filter(|p| p.valid())
				let m = p.min_depth();
				if m < min_depth {
					best = Some(p);
					min_depth = m;
				}
			}
			match best {
				None => (), //done here! no successors?? TODO
				Some(ref b) => {
					let wid = *next_avail_world;
					*next_avail_world += 1;
					builder.add_access(curr_world, wid);
					build_counter_model(wid, b, next_avail_world, builder);
				},
			}
		},
		&BothValid(ref a, ref b) => {
			let (do_a, do_b) = if proof.valid() {
				//need to prove both
				(true, true)
			} else { //proof invalid!
				if a.valid() {
					(false, true)
				} else { //a invalid
					if b.valid() {
						(true, false)
					} else { //b invalid
						//do the cheapest
						if a.min_depth() <= b.min_depth() {
							(true, false)
						} else {
							(false, true)
						}
					}
				}
			};
			if do_a {
				build_counter_model(curr_world, a, next_avail_world, builder);
			}
			if do_b {
				build_counter_model(curr_world, b, next_avail_world, builder);
			}
		}
	}
}
