use rand::Rng;
use weighted_rand::builder::NewBuilder;
use crate::types::{FitnessFunc, Genome, Population};

fn sort_population(population: &mut Population, fitness_func: FitnessFunc) {
    let mut pop = population;
    pop.sort_by(|a, b| fitness_func(b).cmp(&fitness_func(a)));
}

fn selection_pair(population: &Population, fitness_func: FitnessFunc) -> Population {
    let weights: Vec<u32> = population.iter().map(|g| fitness_func(g) as u32).collect();
    let builder = weighted_rand::builder::WalkerTableBuilder::new(weights.as_slice());
    let table = builder.build();
    let mut population_pair: Population = Vec::new();

    for i in (0..2).map(|_| table.next()) {
        population_pair.push(population[i].clone());
    }

    return population_pair;
}

fn single_point_crossover(parent1: &Genome, parent2: &Genome) -> (Genome, Genome) {
    if parent1.len() != parent2.len() {
        panic!("Genomes must be the same length");
    }

    let length = parent1.len();
    let crossover_point = rand::thread_rng().gen_range(0..length);

    let mut child1 = parent1[0..crossover_point].to_vec();
    child1.extend_from_slice(&parent2[crossover_point..length]);

    let mut child2 = parent2[0..crossover_point].to_vec();
    child2.extend_from_slice(&parent1[crossover_point..length]);

    return (child1, child2);
}

fn mutation(genome: &Genome, num: i32, probability: f32) -> Genome {
    let mut mutated_genome = genome.clone();

    for _ in 0..num {
        let index = rand::thread_rng().gen_range(0..genome.len());
        let value = rand::thread_rng().gen_range(0.0..1.0);

        if value > probability {
            mutated_genome[index] = genome[index];
        } else {
            mutated_genome[index] = (genome[index] - 1).abs();
        }
    }

    return mutated_genome;
}

pub fn run(
    population: &mut Population,
    fitness_func: FitnessFunc,
    fitness_limit: i32,
    generation_limit: i32,
    mutation_probability: f32,
    mutation_num: i32,
) -> (Population, i32, Vec<i32>) {
    let mut history: Vec<i32> = Vec::new();
    let mut population = population.clone();
    let mut counter = 0;
    for _ in 0..generation_limit {
        counter += 1;
        sort_population(&mut population, fitness_func);

        if fitness_func(&population[0]) >= fitness_limit {
            continue;
        }

        let mut next_generation = population[0..2].to_vec();

        for _ in 0..((population.len() / 2) - 1) {
            let parents = selection_pair(&population, fitness_func);
            let mut offsprings = single_point_crossover(&parents[0], &parents[1]);
            offsprings.0 = mutation(&offsprings.0, mutation_num, mutation_probability);
            offsprings.1 = mutation(&offsprings.1, mutation_num, mutation_probability);
            next_generation.push(offsprings.0);
            next_generation.push(offsprings.1);
        }

        population = next_generation;
        history.push(fitness_func(&population[0]));
    }

    sort_population(&mut population, fitness_func);

    return (population.to_vec(), counter, history);
}
