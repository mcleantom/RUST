use super::*;
use helper::print_vec;
use rand::{thread_rng, Rng};

pub struct Simulation {
    iterations: usize,
    crossover_probability: f64,
    mutation_probability: f64,
    population_size: usize,
    number_of_cities: usize,
    cities: Vec<City>,
    number_of_mutations: usize,
    number_of_crossovers: usize,
    pub fitness: f64,
    pub dna: Vec<usize>,
}

impl Simulation {
    pub fn new(
        iterations: usize,
        crossover_propability: f64,
        mutation_propability: f64,
        population_size: usize,
        cities: Vec<City>,
    ) -> Self {
        assert_eq!(
            population_size % 10,
            0,
            "Population size must be divisible by 10",
        );
        let number_of_cities = cities.len();
        let number_of_mutations = 0;
        let number_of_crossovers = 0;
        let fitness = 0.0;
        let dna = Vec::new();
        Simulation {
            iterations,
            crossover_probability: crossover_propability,
            mutation_probability: mutation_propability,
            population_size,
            number_of_cities,
            cities,
            number_of_mutations,
            number_of_crossovers,
            fitness,
            dna,
        }
    }

    pub fn run(&mut self, debug_level: usize, skip: usize) {
        let mut population = random_population(self.population_size, &self.cities);
        let mut champion = find_fittest(&population);

        for i in 0..self.iterations {
            population = self.generate_population(population);
            let challenger = find_fittest(&population);
            debug_print(
                debug_level,
                skip,
                i,
                &population,
                &champion,
                &challenger,
                self.number_of_cities,
            );
            if challenger.fitness < champion.fitness {
                champion = challenger;
            }
        }
        self.fitness = champion.fitness;
        self.dna = champion.dna;

        if debug_level >= 2 {
            self.print();
        }
    }

    pub fn generate_population(&mut self, individuals: Vec<Individual>) -> Vec<Individual> {
        assert_eq!(self.population_size % 2, 0, "Population size must be even");
        let cumulative_weights = get_cumulative_weights(&individuals);
        let mut next_population = Vec::new();

        for _ in 0..(self.population_size / 2) {
            let (mom, dad) = select_parents(&cumulative_weights, &individuals);
            let (mut baby1, mut baby2) = self.generate_children(mom, dad);
            self.might_mutate_child(&mut baby1);
            self.might_mutate_child(&mut baby2);
            next_population.push(baby1);
            next_population.push(baby2);
        }
        next_population
    }

    fn generate_children(
        &mut self,
        mom: &Individual,
        dad: &Individual,
    ) -> (Individual, Individual) {
        if thread_rng().gen_bool(self.crossover_probability) {
            mom.cross_over(dad, &self.cities)
        } else {
            (mom.clone(), dad.clone())
        }
    }

    fn might_mutate_child(&mut self, child: &mut Individual) {
        if thread_rng().gen_bool(self.mutation_probability) {
            child.mutate(&self.cities);
        }
    }

    pub fn print(&self) {
        let x = self.population_size * self.iterations;

        println!("\n --------------- \n STATS \n --------------- \n");
        println!("BEST TRAVEL PATH: {:?}", self.dna);
        println!("Fitness Score: {} ", self.fitness);
        println!(
            "{} mutations out of {} individuals produced",
            self.number_of_mutations, x
        );
        println!(
            "{} cross-overs out of {} individuals produced",
            self.number_of_crossovers, x
        );

        println!("\n --------------- \n SPECS \n --------------- \n");
        println!("iterations: {:?}", self.iterations);
        println!("crossover_probability: {:?}", self.crossover_probability);
        println!("mutation_probability: {:?}", self.mutation_probability);
        println!("population_size: {:?}", self.population_size);
        println!("number_of_cities: {:?}", self.number_of_cities);
        println!("\n Cities: ");
        print_vec(&self.cities);

        println!("\n --------------- \n END \n --------------- \n");
    }
}

pub fn random_population(population_size: usize, cities: &[City]) -> Vec<Individual> {
    let mut individuals: Vec<Individual> = Vec::new();
    let number_of_cities = cities.len();
    for _ in 0..population_size {
        let dna = random_dna(number_of_cities);
        let individual = Individual::new(dna, &cities);
        individuals.push(individual);
    }
    individuals
}

fn debug_print(
    debug_level: usize,
    skip: usize,
    i: usize,
    population: &[Individual],
    champion: &Individual,
    challenger: &Individual,
    n: usize,
) {
    if debug_level == 1 && (i % skip == 0) {
        print!(
            "{}, {}, {}, {},",
            i, n, champion.fitness, challenger.fitness
        );

        for i in 0..n {
            print!(" {},", champion.dna[i]);
        }

        for i in 0..(n - 1) {
            print!(" {},", challenger.dna[i]);
        }

        println!(" {}", challenger.dna[n - 1]);
    }

    if debug_level == 3 {
        println!(
            "#{}: \n current_champion: {:?} \n challenger: {:?}",
            i, champion, challenger
        );
    }

    if debug_level == 4 {
        println!(
            "\n --------------- \n {}: Current Population \n --------------- \n",
            i
        );
        print_vec(population);
    }
}
