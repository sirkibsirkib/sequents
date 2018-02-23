
use ::sequents::{Sequent, StepResult};

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
	pub fn new(mut m: Sequent) -> Proof {
		let mut steps = vec![format!("• prove : {:?}", &m)];
		loop {
			use self::StepResult::*;
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
		if depth == 0 {
			println!("{}", if self.valid {"VALID"} else {"INVALID"});
		}
	}
}
