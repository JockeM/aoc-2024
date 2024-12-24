use std::collections::{HashMap, HashSet};

use itertools::Itertools;

advent_of_code::solution!(23);

pub fn part_one(input: &str) -> Option<u32> {
    let edges = input
        .lines()
        .map(|line| line.split_once('-').unwrap())
        .collect_vec();

    let mut graph = HashMap::<&str, HashSet<&str>>::new();
    for (a, b) in &edges {
        graph.entry(*a).or_default().insert(*b);
        graph.entry(*b).or_default().insert(*a);
    }

    let mut triangles = HashSet::new();
    for (a, b) in &edges {
        if let (Some(neighbors_a), Some(neighbors_b)) = (graph.get(a), graph.get(b)) {
            for &c in neighbors_a.intersection(neighbors_b) {
                let mut triangle = vec![*a, *b, c];
                triangle.sort();
                triangles.insert(triangle);
            }
        }
    }

    let found = triangles.into_iter().collect_vec();

    Some(
        found
            .iter()
            .filter(|t| t.iter().any(|e| e.starts_with('t')))
            .count() as u32,
    )
}

fn is_clique(graph: &HashMap<&str, HashSet<&str>>, nodes: &[&str]) -> bool {
    for i in 0..nodes.len() {
        for j in i + 1..nodes.len() {
            let res = graph.get(nodes[i]);
            if res.map_or(true, |node| !node.contains(nodes[j])) {
                return false;
            }
        }
    }
    true
}

fn find_largest_clique<'a>(
    graph: &HashMap<&'a str, HashSet<&'a str>>,
    nodes: &[&'a str],
    current: &mut Vec<&'a str>,
    largest: &mut Vec<&'a str>,
) {
    if current.len() > largest.len() {
        *largest = current.clone();
    }

    for (i, &node) in nodes.iter().enumerate() {
        current.push(node);
        if is_clique(graph, current) {
            find_largest_clique(graph, &nodes[i + 1..], current, largest);
        }
        current.pop();
    }
}

pub fn part_two(input: &str) -> Option<String> {
    let edges = input
        .lines()
        .map(|line| line.split_once('-').unwrap())
        .collect_vec();

    let mut graph = HashMap::<&str, HashSet<&str>>::new();
    for (a, b) in &edges {
        graph.entry(*a).or_default().insert(*b);
        graph.entry(*b).or_default().insert(*a);
    }

    let nodes = graph.keys().cloned().collect_vec();
    let mut largest_clique = Vec::new();

    find_largest_clique(&graph, &nodes, &mut Vec::new(), &mut largest_clique);

    Some(largest_clique.iter().sorted().join(","))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("co,de,ka,ta".to_string()));
    }
}
