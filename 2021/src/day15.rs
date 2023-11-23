use std::cmp::Ordering;
use std::collections::BinaryHeap;

const OFFSETS: [Point; 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];

type Point = (isize, isize);
type Node = Vec<Edge>;

#[derive(Debug)]
pub struct Edge {
    node: usize, // index in node list this edge points to
    cost: usize, // cost to move from the node that owns this edge to the node at `node`
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: usize,
}

// min-heap instead of max-heap, we want the shorter paths over the longer ones
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[aoc_generator(day15, part1)]
pub fn input_generator_part1(input: &str) -> Vec<Node> {
    let points: Vec<Vec<usize>> = generate_points(input);
    generate_nodes(points)
}

#[aoc_generator(day15, part2)]
pub fn input_generator_part2(input: &str) -> Vec<Node> {
    let mut points: Vec<Vec<usize>> = generate_points(input);
    expand_points(&mut points);
    generate_nodes(points)
}

// string -> 2D Vec of integer (risk values)
fn generate_points(input: &str) -> Vec<Vec<usize>> {
    input
        .split("\n")
        .map(|row| row.chars().map(|n| n as usize - '0' as usize).collect())
        .collect()
}

// 2D Vec of integers -> Vec of Node
fn generate_nodes(points: Vec<Vec<usize>>) -> Vec<Node> {
    points
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            let points = points.clone();

            (0..row.len()).map(move |x| find_edges(&points, (x as isize, y as isize)))
        })
        .collect()
}

// 🤷🏻‍♂️ tile the points 5 times horizontally and vertically, this is gross
fn expand_points(points: &mut Vec<Vec<usize>>) {
    let original_points = points.clone();
    let height = original_points.len();
    let width = original_points[0].len();

    // expand horizontally
    for dx in 0..4 {
        for y in 0..height {
            for x in 0..width {
                let value = points[y][x + width * dx];
                points[y].push(value % 9 + 1);
            }
        }
    }

    // expand vertically
    for dy in 1..5 {
        for y in 0..height {
            points.push(Vec::with_capacity(width * 5));

            for x in 0..width * 5 {
                let new_y = y + height * dy;
                let value = points[new_y - height][x];
                points[new_y].push(value % 9 + 1);
            }
        }
    }
}

#[aoc(day15, part1)]
pub fn part1(graph: &Vec<Node>) -> usize {
    if let Some(distance) = shortest_path(&graph) {
        distance
    } else {
        panic!("Could not find a shortest path")
    }
}

#[aoc(day15, part2)]
pub fn part2(graph: &Vec<Node>) -> usize {
    if let Some(distance) = shortest_path(&graph) {
        distance
    } else {
        panic!("Could not find a shortest path")
    }
}

fn find_edges(points: &Vec<Vec<usize>>, origin: Point) -> Vec<Edge> {
    valid_neighbors(&origin, (points[0].len() as isize, points.len() as isize))
        .iter()
        .map(|&(x, y)| {
            Edge {
                // dest node position in master list
                node: x as usize + y as usize * points.len(),
                cost: points[y as usize][x as usize],
            }
        })
        .collect()
}

// Djikstra's Algorithm https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm
fn shortest_path(graph: &Vec<Node>) -> Option<usize> {
    let start = 0;
    let end = graph.len() - 1;
    let mut dist: Vec<_> = (0..graph.len()).map(|_| usize::MAX).collect();
    let mut heap = BinaryHeap::new();

    // start at the beginning, it's free
    dist[start] = 0;

    heap.push(State {
        cost: 0,
        position: start,
    });

    // iterate over the lowest cost next node
    while let Some(State { cost, position }) = heap.pop() {
        if position == end {
            return Some(cost);
        }

        // there's a better way
        if cost > dist[position] {
            continue;
        }

        // for each node we can reach, check if that pathway is cheaper
        for edge in &graph[position] {
            let next = State {
                cost: cost + edge.cost,
                position: edge.node,
            };

            // if it is, add it to the heap and continue
            if next.cost < dist[next.position] {
                heap.push(next);

                // relaxation, we found a better way
                dist[next.position] = next.cost;
            }
        }
    }

    None
}

fn valid_neighbors(point: &Point, dimensions: (isize, isize)) -> Vec<Point> {
    OFFSETS
        .iter()
        .map(|(dx, dy)| (point.0 + dx, point.1 + dy))
        .filter(|&(x, y)| valid_point(x, y, dimensions.0, dimensions.1))
        .collect()
}
fn valid_point(x: isize, y: isize, width: isize, height: isize) -> bool {
    x >= 0 && x < width && y >= 0 && y < height
}
