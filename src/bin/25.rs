use std::collections::HashMap;

use itertools::Itertools;
use petgraph::{
    graph::UnGraph,
    stable_graph::{NodeIndex, StableGraph, StableUnGraph},
    visit::{Bfs, EdgeRef, IntoNodeIdentifiers},
    Graph, Undirected,
};
use priority_queue::PriorityQueue;

advent_of_code::solution!(25);

fn min_cut_phase(
    graph: &StableGraph<(), i32, Undirected>,
) -> Option<((NodeIndex, NodeIndex), i32)> {
    let mut pq = PriorityQueue::<NodeIndex, i32, ahash::RandomState>::from(
        graph.node_identifiers().map(|node| (node, 0)).collect_vec(),
    );

    let mut cut_w = None;
    let (mut s, mut t) = (None, None);
    while let Some((nx, val)) = pq.pop() {
        s = t;
        t = Some(nx);
        cut_w = Some(val);
        for edge in graph.edges(nx) {
            pq.change_priority_by(&edge.target(), |x| *x += *edge.weight());
        }
    }

    s.zip(t).zip(cut_w)
}

fn min_cut(graph: &mut Graph<i32, i32, Undirected>) -> Result<Option<(i32, Vec<NodeIndex>)>, ()> {
    let mut graph_with_super_nodes =
        StableUnGraph::with_capacity(graph.node_count(), graph.edge_count());

    let mut node_map = HashMap::with_capacity(graph.node_count());
    let mut rev_node_map = HashMap::with_capacity(graph.node_count());

    for node in graph.node_identifiers() {
        let index = graph_with_super_nodes.add_node(());
        node_map.insert(node, index);
        rev_node_map.insert(index, node);
    }

    for edge in graph.edge_references() {
        let cost = *edge.weight();
        let source = node_map[&edge.source()];
        let target = node_map[&edge.target()];
        graph_with_super_nodes.add_edge(source, target, cost);
    }

    if graph_with_super_nodes.node_count() == 0 {
        return Ok(None);
    }

    let (mut best_phase, mut min_cut_val) = (None, None);

    let mut contractions = Vec::new();
    for phase in 0..(graph_with_super_nodes.node_count() - 1) {
        if let Some(((s, t), cut_w)) = min_cut_phase(&graph_with_super_nodes) {
            if min_cut_val.is_none() || Some(cut_w) < min_cut_val {
                best_phase = Some(phase);
                min_cut_val = Some(cut_w);
            }
            // now merge nodes ``s`` and  ``t``.
            contractions.push((s, t));
            let edges = graph_with_super_nodes
                .edges(t)
                .map(|edge| (s, edge.target(), *edge.weight()))
                .collect::<Vec<_>>();
            for (source, target, cost) in edges {
                if let Some(edge_index) = graph_with_super_nodes.find_edge(source, target) {
                    graph_with_super_nodes[edge_index] += cost;
                } else {
                    graph_with_super_nodes.add_edge(source, target, cost);
                }
            }
            graph_with_super_nodes.remove_node(t);
        }
    }

    // Recover the optimal partitioning from the contractions
    let min_cut = best_phase.map(|phase| {
        let mut clustered_graph = StableUnGraph::<(), ()>::default();
        clustered_graph.extend_with_edges(&contractions[..phase]);

        let node = contractions[phase].1;
        if clustered_graph.contains_node(node) {
            let mut cluster = Vec::new();
            let mut bfs = Bfs::new(&clustered_graph, node);
            while let Some(nx) = bfs.next(&clustered_graph) {
                cluster.push(rev_node_map[&nx])
            }
            cluster
        } else {
            vec![rev_node_map[&node]]
        }
    });

    Ok(min_cut_val.zip(min_cut))
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut graph = UnGraph::new_undirected();
    let mut nodes = HashMap::new();

    input.lines().for_each(|line| {
        let (from, to) = line.split(':').collect_tuple().unwrap();
        let from_node = nodes
            .entry(from)
            .or_insert_with(|| graph.add_node(1))
            .clone();
        let to = to.split_whitespace().map(|s| s.trim());
        to.for_each(|to| {
            let to_node = nodes.entry(to).or_insert_with(|| graph.add_node(1));
            graph.add_edge(from_node.clone(), to_node.clone(), 1);
        });
    });

    let node_count = graph.node_count();

    let (_cut_length, lhs_nodes) = min_cut(&mut graph).unwrap().unwrap();

    let result = lhs_nodes.len() * (node_count - lhs_nodes.len());

    Some(result as u32)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(54));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
