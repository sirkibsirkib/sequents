
use ::sequents::{Sequent, StepResult};
use std::collections::HashSet;

pub struct Proof {
	steps: Vec<String>,
	proof_result: ProofResult,
	true_here: HashSet<char>,
	valid: bool,
}

pub enum ProofResult {
	Valid,
	Invalid,
	AnyValid(Vec<Proof>),
	BothValid(Box<Proof>, Box<Proof>),
}

impl Proof {
	pub fn proof_result(&self) -> &ProofResult {
		&self.proof_result
	}

	pub fn true_here(&self) -> &HashSet<char> {
		&self.true_here
	}

	pub fn valid(&self) -> bool {
		self.valid
	}

	pub fn new(mut m: Sequent) -> Proof {
		let mut steps = vec![format!("* Prove: {:?}", &m)];
		loop {
			use self::StepResult::*;
			match m.step() {
				Indeterminate(r, a) => {
					steps.push(format!("  [{}] {:?}", r, &a));
					m = a;
				},
				Valid(letters_on_left) => {
					steps.push(format!("  valid!"));
					return Proof {
						steps: steps,
						proof_result: ProofResult::Valid,
						true_here: letters_on_left,
						valid: true,
					}
				},
				Invalid(letters_on_left) => {
					steps.push(format!("  invalid!"));
					return Proof {
						steps: steps,
						proof_result: ProofResult::Invalid,
						true_here: letters_on_left,
						valid: false,
					}
				},
				ValidIfAny(r, v, letters_on_left) => {
					let proofs = v.into_iter().map(|x| Proof::new(x)).collect::<Vec<_>>();
					let valid = proofs.iter().fold(false, |a,b| a||b.valid);
					steps.push(format!("  [{}] valid if any... ({})", r, if valid {"valid"} else {"invalid"}));
					return Proof {
						steps: steps,
						proof_result: ProofResult::AnyValid(proofs),
						true_here: letters_on_left,
						valid: valid,
					}
				},
				ValidIfBoth(r, a, b, letters_on_left) => {
					let a = Box::new(Proof::new(a));
					let b = Box::new(Proof::new(b));
					let valid = a.valid && b.valid;
					steps.push(format!("  [{}] valid if both... ({})", r, if valid {"valid"} else {"invalid"}));
					return Proof {
						steps: steps,
						proof_result: ProofResult::BothValid(a, b),
						true_here: letters_on_left,
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
		use self::ProofResult::*;
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

	pub fn min_depth(&self) -> usize {
		use self::ProofResult::*;
		match self.proof_result {
			Valid => 0,
			Invalid => 0,
			AnyValid(ref proofs) => {
				proofs.iter()
				.map(|x| x.min_depth())
				.min().unwrap() + 1
			},
			BothValid(ref proof_a, ref proof_b) => {
				proof_a.min_depth().min(
					proof_b.min_depth()
				) + 1
			},
		}
	}
}
