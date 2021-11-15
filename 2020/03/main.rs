use std::fs;

#[derive(Debug)]
struct Slope {
  x: usize,
  y: usize
}

fn main() {
  let sample_map = read_input("sample-input.txt");
  let input_map = read_input("input.txt");

  let slopes: Vec<Slope> = vec![
    Slope { x: 1, y: 1 },
    Slope { x: 3, y: 1 },
    Slope { x: 5, y: 1 },
    Slope { x: 7, y: 1 },
    Slope { x: 1, y: 2 }
  ];

  let mut sample_total: usize = 1;
  let mut input_total: usize = 1;

  for slope in slopes {
    let sample_map_tree_count = calculate_trees_hit(&sample_map, &slope);
    let input_map_tree_count = calculate_trees_hit(&input_map, &slope);

    println!("Sample tree count: {} for (slope: {:?})", sample_map_tree_count, slope);
    println!("Input tree count: {} for (slope: {:?})", input_map_tree_count, slope);

    sample_total = sample_total * sample_map_tree_count;
    input_total = input_total * input_map_tree_count;
  }

  println!("Sample Total: {}", sample_total);
  println!("Input Total: {}", input_total);
}

fn read_input(filename: &str) -> Vec<Vec<char>> {
  let input: String = fs::read_to_string(filename).unwrap_or("".to_string());
  let lines: Vec<&str> = input.split("\n").collect();

  lines.iter().map(|line| line.chars().collect()).collect()
}

fn calculate_trees_hit(map: &Vec<Vec<char>>, slope: &Slope) -> usize {
  let mut tree_count: usize = 0;

  let mut x = 0;
  let mut y = 0;

  while y < map.len() {
    if y > 0 && feature_at(map, x as usize, y as usize) == '#' {
      tree_count = tree_count + 1;
    }

    x = x + slope.x;
    y = y + slope.y;
  }

  tree_count
}

fn feature_at(map: &Vec<Vec<char>>, x: usize, y: usize) -> char {
  let row_size = map[0].len();
  let xx: usize = x % row_size;
  let yy: usize = y % map.len();

  map[yy][xx]
}
