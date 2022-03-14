// in cargo.toml file
[dependencies]
rand = "*"
rand_distr = "*"


use rand::prelude::*;

let numbers: u8 = random();
let yes_no: bool = random();

let mut rng = thread_rng();
let number = rng.gen_range(0..10);

let letters = ['a' , 'b' , 'c' , 'd'];
let letter = letters.iter().choose(&mut rng);

let mut letters = letters;
letters.shuffle(&mut rng);

// Seeding is not cryptographic secure, so we
use rand::prelude::*;
use rand_pcg::Pcg64;
use rand_seeder::Seeder;

let rng = Pcg64::seed_from_u64(10);
let rng: Pcg64 = Seeder::from("seed value").make_rng();

//Distributions from
use rand::distributions::{Distributions , Uniform};
use rand::prelude::*;

let range = Uniform::from(5..500);
let mut rng = thread_rng();
range.sample(&mut rng);

// choose a random item from an Iterator
// sample a random number from a distributions