use std::fs;

// Function to read city coordinates from a file and return them as a vector of tuples (x, y)
pub fn read_city_coords(path: &str) -> Vec<(f64,f64)> {
    let content = fs::read_to_string(path).unwrap();
    let mut coords: Vec<(f64, f64)> = Vec::new();
    let mut reading: bool = false;

    for line in content.lines() {
        if line.starts_with("NODE_COORD_SECTION") {
            reading = true;
            continue;
        }

        if reading {
            let parts: Vec<&str> = line.split_whitespace().collect();

            if parts.len() == 3 {
                let x: f64 = parts[1].parse().unwrap();
                let y: f64 = parts[2].parse().unwrap();
                coords.push((x, y));
            }
        }
    }

    coords
}

// Function to build a distance matrix from city coordinates using Euclidean distance
pub fn build_distance_matrix(coords: &Vec<(f64, f64)>) -> Vec<Vec<f64>> {
    let city_num = coords.len();
    let mut matrix = vec![vec![0.0; city_num]; city_num];

    for i in 0..city_num {
        for j in 0..city_num {
            let dx = coords[i].0 - coords[j].0;
            let dy = coords[i].1 - coords[j].1;

            matrix[i][j] = (dx*dx + dy*dy).sqrt();
        }
    }
    
    matrix
}

pub fn read_cost_pairs(path: &str) -> Vec<(f64, f64)> {
    let content = fs::read_to_string(path).expect("Failed to read the file!");
    let mut pairs = Vec::new();

    for line in content.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.len() == 2 {
            let a: f64 = parts[0].parse().expect("Couldn't parse the value!");
            let b: f64 = parts[1].parse().expect("Couldn't parse the value!");
            
            pairs.push((a, b));
        }
    }

    pairs
}

fn weighted_score(a: f64, b: f64, alpha: f64, beta: f64) -> f64 {
    alpha * a + beta * b
}

pub fn find_best_pair(pairs: &[(f64, f64)], alpha: f64, beta: f64) -> Option<((f64, f64), f64)> {
    if pairs.is_empty() {
        return None;
    }

    let mut best_pair = pairs[0];
    let mut best_score = weighted_score(pairs[0].0, pairs[0].1, alpha, beta);

    for &(a, b) in &pairs[1..] {
        let score = weighted_score(a, b, alpha, beta);

        if score < best_score {
            best_pair = (a, b);
            best_score = score;
        }
    }

    Some((best_pair, best_score))
}