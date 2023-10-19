use rand::Rng;
use weighted_rand::builder::NewBuilder;

use crate::things::things_types::Thing;
use crate::types::Genome;

mod network;
mod types;
mod things;

pub const THINGS: &[Thing] = &[
    Thing::new_const("Mints", 5, 25),
    Thing::new_const("Socks", 10, 38),
    Thing::new_const("Tissues", 15, 80),
    Thing::new_const("Phone", 500, 2003),
    Thing::new_const("Baseball Cap", 100, 70),
    Thing::new_const("Sunglasses", 20, 100),
    Thing::new_const("Keys", 5, 20),
    Thing::new_const("Wallet", 50, 100),
    Thing::new_const("Headphones", 50, 100),
    Thing::new_const("Water Bottle", 50, 100),
    Thing::new_const("Laptop", 1000, 2000),
    Thing::new_const("Notebook", 50, 100),
    Thing::new_const("Pens", 5, 20),
    Thing::new_const("Pencils", 5, 20),
    Thing::new_const("Eraser", 5, 20),
    Thing::new_const("Charger", 50, 100),
    Thing::new_const("Gum", 5, 20),
    Thing::new_const("Lip Balm", 5, 20),
    Thing::new_const("Hand Sanitizer", 5, 20),
    Thing::new_const("Lotion", 5, 20),
    Thing::new_const("Deodorant", 5, 20),
    Thing::new_const("Perfume", 5, 20),
    Thing::new_const("Watch", 50, 100),
    Thing::new_const("Bracelet", 50, 100),
    Thing::new_const("Necklace", 50, 100),
    Thing::new_const("Earrings", 50, 100),
];
pub const MAX_WEIGHT: i32 = 3000;
pub const FITNESS_LIMIT: i32 = 99999;

pub const POPULATION_SIZE: usize = 10;
pub const GENOME_LENGTH: usize = THINGS.len();
pub const GENERATIONS: i32 = 30000;
pub const MUTATION_CHANCE: f32 = 0.5;
pub const NUM_MUTATIONS: i32 = 1;


fn fitness_func(genome: &Genome) -> i32 {
    let things = THINGS.to_vec();
    return things::things_nn::calculate_fitness(genome, &things, MAX_WEIGHT);
}

fn print_stat(genome: &Genome) {
    let things = THINGS.to_vec();
    let fitness = genome.iter().enumerate().map(|(i, g)| g * things[i].value).sum::<i32>();
    let weight = genome.iter().enumerate().map(|(i, g)| g * things[i].weight).sum::<i32>();
    println!("-----------------------------");
    for i in 0..genome.len() {
        if genome[i] == 1 {
            println!("{}", things[i].name);
        }
    }
    println!("-----------------------------");
    println!("Fitness: {}, Weight: {}", fitness, weight);
    println!("Items count: {}", genome.iter().sum::<i32>());
}

fn main() {
    let mut population = things::things_nn::generate_population(
        POPULATION_SIZE,
        GENOME_LENGTH,
    );
    let start_time = std::time::Instant::now();
    let (final_population, generation_number, history) = network::run(&mut population,
                                                                      fitness_func,
                                                                      FITNESS_LIMIT,
                                                                      GENERATIONS,
                                                                      MUTATION_CHANCE,
                                                                      NUM_MUTATIONS,
    );
    let end_time = std::time::Instant::now();

    print_stat(&final_population[0]);
    println!("-----------------------------");
    println!("Write last history to history.txt");
    std::fs::write("history.txt", history.iter().map(|h| h.to_string()).collect::<Vec<String>>().join("\n")).expect("Unable to write file");
    println!("-----------------------------");
    println!("Time: {}ms", end_time.duration_since(start_time).as_millis());
    println!("Number of Generations: {}", generation_number);
}
