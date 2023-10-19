pub type Genome = Vec<i32>;
pub type Population = Vec<Genome>;
pub type FitnessFunc = fn(&Genome) -> i32;
