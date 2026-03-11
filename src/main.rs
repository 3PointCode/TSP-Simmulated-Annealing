mod algorithms;
mod utils;

use algorithms::{nearest_neighbor, simmulated_annealing};
use utils::{read_city_coords, build_distance_matrix, read_cost_pairs, find_best_pair};

fn main() {
    let pairs = read_cost_pairs("data/best.euclidAB300.tsp");
    let coords = read_city_coords("data/euclidA300.tsp");
    let distance = build_distance_matrix(&coords);
    let coords = read_city_coords("data/euclidB300.tsp");
    let time = build_distance_matrix(&coords);
    let (_, cost) = nearest_neighbor(&distance, &time, 0, 0.5, 0.5);
    println!("Total cost: {}", cost.round());

    let (optimized_route, optimized_cost) = simmulated_annealing(&distance, &time, 0, 0.5, 0.5, 10000.0, 0.001, 0.995, 1000);
    match find_best_pair(&pairs, 0.5, 0.5) {
        Some(((a, b), score)) => {
            println!("Best Route Pair: ({}, {})", a, b);
            println!("Best Route Score: {}", score);
        },
        None => println!("No pairs found!"),
    }
    println!("Optimized Route: {:?}", optimized_route);
    println!("Optimized Cost: {}", optimized_cost.round());
}