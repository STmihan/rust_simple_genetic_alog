use rand::Rng;
use crate::things::things_types::Thing;
use crate::types::{Genome, Population};

pub fn calculate_fitness(genome: &Genome, things: &Vec<Thing>, max_weight: i32) -> i32 {
    if genome.len() != things.len() {
        panic!("Genome and thins must be the same length");
    }

    let mut weight = 0;
    let mut value = 0;

    for (i, thing) in things.iter().enumerate() {
        if genome[i] == 1 {
            weight += thing.weight;
            value += thing.value;

            if weight > max_weight {
                return 0;
            }
        }
    }

    return value;
}

fn generate_genome(length: usize) -> Genome {
    return (0..length).map(|_| rand::thread_rng().gen_range(0..1)).collect();
}

pub fn generate_population(population_size: usize, genome_length: usize) -> Population {
    return (0..population_size).map(|_| generate_genome(genome_length)).collect();
}
