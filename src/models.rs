use std::collections::{HashMap, HashSet};
use std::fmt;

pub struct Model {
	pub num_worlds: u32,
	pub accessibility_function: HashSet<(u32, u32)>,
	pub valuations: HashMap<char, HashSet<u32>>,
}

impl fmt::Debug for Model {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    	let preamble = "Model:\n  worlds:     ";
    	match self.num_worlds {
    		1 => write!(f, "{}{{1}}\n", preamble),
    		2 => write!(f, "{}{{1, 2}}\n", preamble),
    		3 => write!(f, "{}{{1, 2, 3}}\n", preamble),
    		4 => write!(f, "{}{{1, 2, 3, 4}}\n", preamble),
    		x => write!(f, "{}{{1, 2, ... {}}}\n", preamble, x),
    	}?;
    	if !self.accessibility_function.is_empty() {
    		write!(f, "  access fn.: {:?}\n", &self.accessibility_function)?;
    	}
		if !self.valuations.is_empty() {
			write!(f, "  valuations: {{\n")?;
			for (k, v) in self.valuations.iter() {
				write!(f, "    {}: {:?}", k, v)?;
			}
			write!(f, "\n  }}")?;
		}
		Ok(())
    }
	
}

#[derive(Debug)]
pub struct ModelBuilder {
	m: Model,
}

impl ModelBuilder {
	pub fn new() -> ModelBuilder {
		let m = Model {
			num_worlds: 1,
			accessibility_function: HashSet::new(),
			valuations: HashMap::new(),
		};
		ModelBuilder {
			m: m,
		}
	}

	pub fn add_access(&mut self, from: u32, to: u32) {
		self.m.num_worlds = self.m.num_worlds.max(to.max(from));
		self.m.accessibility_function.insert((from, to));
	}

	pub fn set_true_in(&mut self, world: u32, variable: char) {
		self.m.num_worlds = self.m.num_worlds.max(world);
		if !self.m.valuations.contains_key(&variable) {
			self.m.valuations.insert(variable, HashSet::new());
		} 
		self.m.valuations.get_mut(&variable).unwrap().insert(world);
	}

	pub fn finalize(self) -> Model {
		self.m
	}
}