use aoc_runner_derive::{aoc, aoc_generator};
use bit_set::*;
use itertools::Itertools;
use std::cmp::max;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::fmt::Debug;

use petgraph::algo::dijkstra;
use petgraph::graph::NodeIndex;
use petgraph::{Graph, Undirected};

#[derive(Debug, Clone)]
pub struct NodeData {
    valve_id: String,
    flow_rate: i64,
    enable_cost: i64,
}

#[derive(Debug, Clone)]
pub struct EdgeData {
    travel_cost: i64,
    skipped_rooms: Vec<String>,
}

type GraphData = Graph<NodeData, EdgeData, Undirected>;

pub struct GenData {
    graph: GraphData,
    start_node: NodeIndex,
}
pub type InData<'a> = &'a GenData;
pub type OutData = i64;

// Solution ---------------------------------------------------------

#[aoc_generator(day16)]
pub fn input_generator(input: &str) -> GenData {
    let mut results: GraphData = Graph::default();
    let mut nodes: HashMap<&str, NodeIndex> = HashMap::new();

    for ln in input.lines() {
        if ln == "" {
            continue;
        }

        let (_, rest) = ln.split_once(' ').unwrap();
        let (valve_id, rest) = rest.split_once(' ').unwrap();
        let (_, rest) = rest.split_once('=').unwrap();
        let (rate, rest) = rest.split_once(';').unwrap();
        let (_, rest) = rest.split_once(" to ").unwrap();
        let (_, connecting_list) = rest.split_once(" ").unwrap();
        let connecting_list = connecting_list.split(", ").collect_vec();

        let node = NodeData {
            valve_id: valve_id.to_owned(),
            flow_rate: rate.parse().unwrap(),
            enable_cost: 1,
        };

        let idx = results.add_node(node);
        nodes.insert(valve_id, idx);
        for node_id in connecting_list {
            match nodes.get(node_id) {
                // Undirected graph. If the node isn't in the dictionary, we haven't visited said node,
                // and we'll make the connection when we visit the other node.
                None => {
                    continue;
                }
                Some(index) => {
                    results.add_edge(idx, *index, EdgeData { travel_cost: 1, skipped_rooms: vec![] });
                }
            }
        }
    }

    // Post-process the graph. Remove nodes with 0 flow, etc.
    // 
    // (Ideally, you'd want to do all this while building the graph, but 
    //    the nodes don't all exist while building the graph. That makes 
    //    this work harder.)
    for n_idx in nodes.values() {
        let n_data = results.node_weight(*n_idx).unwrap();
        if n_data.flow_rate != 0 { continue; }

        let n_data = n_data.clone();
        let neighbors = results.neighbors(*n_idx).collect_vec();
        for (i, n_src) in neighbors.iter().enumerate() {
            for n_dst in neighbors.iter().skip(i) {
                let s_edge = results.find_edge(*n_src, *n_idx).unwrap();
                let s_data = results.edge_weight(s_edge).unwrap().to_owned();
                let d_edge = results.find_edge(*n_idx, *n_dst).unwrap();
                let d_data = results.edge_weight(d_edge).unwrap().to_owned();

                let mut path: Vec<String> = Vec::new();
                path.extend(s_data.skipped_rooms);
                path.push(n_data.valve_id.clone());
                path.extend(d_data.skipped_rooms);

                results.add_edge(*n_src, *n_dst, EdgeData { 
                    travel_cost: s_data.travel_cost + d_data.travel_cost, 
                    skipped_rooms: path,
                });
            }
        }

        for neighbor in neighbors {
            if let Some(e_idx) = results.find_edge(*n_idx, neighbor)
            {
                results.remove_edge(e_idx);
            }
        }

        results.remove_node(*n_idx);
    }

    // Fill in missing links
    for n_idx in nodes.values() {
        let report = dijkstra(&results, *n_idx, None, |edge_ref| edge_ref.weight().travel_cost);

        for (t_idx, _travel_cost) in report {
            if results.contains_edge(*n_idx, t_idx) { continue; }

            todo!();
        }
    }

    GenData { graph: results, start_node: nodes["AA"] }
}

#[derive(Debug, PartialEq, Eq)]
struct VisitData {
    node_index: NodeIndex,
    nodes_on: BitSet,
    current_score: i64,
    time_remaining: u8,
    path_valves: Vec<String>,
}

impl Ord for VisitData {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let a = self.current_score * (self.time_remaining as i64 + 1);
        let b = other.current_score * (other.time_remaining as i64 + 1);
        a.cmp(&b)
    }
}

impl PartialOrd for VisitData {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[aoc(day16, part1, bfs)]
pub fn solve_part1_bfs(input: InData) -> OutData {
    let GenData {
        graph,
        start_node: start_node_idx
    } = input;
    let start_node_idx = *start_node_idx;

    let mut work_queue: VecDeque<VisitData> = VecDeque::new();
    let max_valves = graph.node_count();

    work_queue.push_back(VisitData {
        node_index: start_node_idx,
        nodes_on: BitSet::with_capacity(graph.node_count()),
        current_score: 0,
        time_remaining: 30,
        path_valves: vec!["AA".to_owned()],
    });

    let mut max_score: i64 = 0;

    // Swap the line comments for DFS vs. BFS
    //while let Some(c_visit) = work_queue.pop_front() { // BFS / queue
    while let Some(c_visit) = work_queue.pop_back() {
        // DFS / stack
        max_score = max(max_score, c_visit.current_score);
        if c_visit.time_remaining < 2 || c_visit.nodes_on.len() == max_valves {
            continue;
        }

        // let c_node = graph.node_weight(c_visit.node_index).unwrap();
        // println!("    Visiting node: {:?} - {:?} ({: >3}m, {: >7}pts )  \t-\t[{:?}]", c_visit.node_index, c_node.valve_id, c_visit.time_remaining, c_visit.current_score, &c_visit.path_valves);

        let neighbors = graph.neighbors(c_visit.node_index)
            .filter(|idx| !c_visit.nodes_on.contains(idx.index()))
            .filter(|idx| graph.node_weight(*idx).unwrap().flow_rate != 0);

        for n_idx in neighbors {
            let n_data = graph.node_weight(n_idx).unwrap();
            let e_idx = graph.find_edge(c_visit.node_index, n_idx).unwrap();
            let e_data = graph.edge_weight(e_idx).unwrap();
            let route_cost = e_data.travel_cost + n_data.enable_cost;

            if c_visit.time_remaining as i64 >= route_cost {
                let new_time = c_visit.time_remaining as i64 - route_cost;
                let new_score = c_visit.current_score + n_data.flow_rate * (new_time as i64);
                let mut new_nodes_on = c_visit.nodes_on.clone();
                new_nodes_on.insert(n_idx.index());
                let mut new_path_valves = c_visit.path_valves.clone();
                new_path_valves.push(n_data.valve_id.clone());

                let move_and_on_data = VisitData {
                    node_index: n_idx,
                    nodes_on: new_nodes_on,
                    current_score: new_score,
                    time_remaining: new_time as u8,
                    path_valves: new_path_valves.clone(),
                };

                work_queue.push_back(move_and_on_data);
            }
        }
    }

    max_score
}

#[derive(Debug, PartialEq, Eq)]
struct VisitData2 {
    idx: [NodeIndex; 2],
    eta: [u8; 2],
    nodes_on: BitSet,
    current_score: i64,
    time_remaining: u8,
    path: HashMap<u8, Vec<(usize, String)>>,
}

impl VisitData2 {
    fn ready_count(&self) -> usize {
        self.eta.iter().filter(|eta| *eta <= &self.time_remaining).count()
    }

    fn which_ready(&self) -> Option<usize> {
        self.eta.iter().enumerate()
            .filter(|(_, eta)| *eta <= &self.time_remaining).map(|(i, _)| i)
            .exactly_one()
            .ok()
    }
}

// impl Debug for VisitData2 {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        
//     }
// }

fn get_node_targets<'a>(graph: &'a GraphData, nodes_on: &'a BitSet) -> impl Iterator<Item = NodeIndex> + 'a
{
    graph.node_indices()
        .filter(move |idx| !nodes_on.contains(idx.index()))
        .filter(|idx| graph.node_weight(*idx).unwrap().flow_rate != 0)
}

fn actor_can_reach(graph: &GraphData, time_left: u8, source: NodeIndex, dest: NodeIndex) -> bool {
    route_cost(graph, source, dest) < time_left as i64
}

fn route_cost(graph: &GraphData, source: NodeIndex, dest: NodeIndex) -> i64 {
    let n_data = graph.node_weight(dest).unwrap();
    let e_idx = graph.find_edge(source, dest).unwrap();
    let e_data = graph.edge_weight(e_idx).unwrap();
    n_data.enable_cost + e_data.travel_cost
}

fn get_next_visit(graph: &GraphData, c_visit: &VisitData2, which_actor: usize, n_idx: NodeIndex) -> VisitData2 {
    let c_idx = c_visit.idx[which_actor];
    let n_data = graph.node_weight(n_idx).unwrap();
    let route_cost = route_cost(graph, c_idx, n_idx);
    
    let mut new_nodes_on = c_visit.nodes_on.clone();
    new_nodes_on.insert(n_idx.index());
    let new_idx = match which_actor {
        0 => [n_idx, c_visit.idx[1]],
        1 => [c_visit.idx[0], n_idx],
        _ => panic!("This shouldn't happen."),
    };
    let new_eta = match which_actor {
        0 => [c_visit.eta[0] - route_cost as u8, c_visit.eta[1]],
        1 => [c_visit.eta[0], c_visit.eta[1] - route_cost as u8],
        _ => panic!("This shouldn't happen."),
    };
    let new_time = *new_eta.iter().max().unwrap();
    let new_score = c_visit.current_score + n_data.flow_rate * (new_time as i64);

    // The next node that we're targeting might not be the next one in time (see `new_time`)
    let mut new_path = c_visit.path.clone();
    let path_which = (new_eta[1] == new_time) as usize;
    let path_label = graph.node_weight(new_idx[path_which]).unwrap().valve_id.clone();
    new_path.insert(new_time, vec![(path_which, path_label)]);

    // We can finally return our result
    VisitData2 {
        idx: new_idx,
        eta: new_eta,
        nodes_on: new_nodes_on,
        current_score: new_score,
        time_remaining: new_time,
        path: new_path,
    }
}

fn get_next_visit2(graph: &GraphData, c_visit: &VisitData2, z_idx: NodeIndex, o_idx: NodeIndex) -> VisitData2 {
    let [z_c_idx, o_c_idx] = c_visit.idx;
    let z_n_data = graph.node_weight(z_idx).unwrap();
    let o_n_data = graph.node_weight(o_idx).unwrap();
    let z_route_cost = route_cost(graph, z_c_idx, z_idx);
    let o_route_cost = route_cost(graph, o_c_idx, o_idx);
    
    let new_eta = [c_visit.eta[0] - z_route_cost as u8, c_visit.eta[1] - o_route_cost as u8];
    let new_time = c_visit.time_remaining as i64 - *new_eta.iter().min().unwrap() as i64;
    let new_score = c_visit.current_score + z_n_data.flow_rate * (new_eta[0] as i64) + o_n_data.flow_rate * (new_eta[1] as i64);
    let mut new_nodes_on = c_visit.nodes_on.clone();
    new_nodes_on.insert(z_idx.index());
    new_nodes_on.insert(o_idx.index());
    let new_idx = [z_idx, o_idx];

    // Our next path segment needs to reflect which actors are actually going to be acting in that time.
    let new_val_iter = new_idx.iter()
        .enumerate()
        .map(|(i, idx)| (i, graph.node_weight(*idx).unwrap().valve_id.clone()))
        .filter(|(i, _)| new_eta[*i] as i64 == new_time);
    let mut new_path = c_visit.path.clone();
    new_path.insert(new_time as u8, new_val_iter.collect());

    // ...and return
    VisitData2 {
        idx: new_idx,
        eta: new_eta,
        nodes_on: new_nodes_on,
        current_score: new_score,
        time_remaining: new_time as u8,
        path: new_path,
    }
}

#[aoc(day16, part2)]
pub fn solve_part2(input: InData) -> OutData {
    let GenData {
        graph,
        start_node: start_node_idx
    } = input;
    let start_node_idx = *start_node_idx;

    let mut work_queue: VecDeque<VisitData2> = VecDeque::new();
    let max_valves = graph.node_count();

    // dbg!(&node_options_map);

    work_queue.push_back(VisitData2 {
        idx: [start_node_idx, start_node_idx],
        eta: [26, 26],
        nodes_on: BitSet::with_capacity(graph.node_count()),
        current_score: 0,
        time_remaining: 26,
        path: [(26, vec![(0, "AA".to_string()), (1, "AA".to_string())])].iter().cloned().collect(),
    });

    let mut max_score: i64 = 0;

    // Swap the line comments for DFS vs. BFS
    //while let Some(c_visit) = work_queue.pop_front() { // BFS / queue
    while let Some(c_visit) = work_queue.pop_back() {
        // DFS / stack
        max_score = max(max_score, c_visit.current_score);
        if c_visit.time_remaining < 2 || c_visit.nodes_on.len() == max_valves {
            continue;
        }

        match c_visit.ready_count() {
            0 => {
                // This probably shouldn't happen, but what the hey...
                let next_clock = *c_visit.eta.iter().max().unwrap();
                let next_visit = VisitData2 {
                    time_remaining: next_clock,
                    ..c_visit
                };
                work_queue.push_back(next_visit);
            },
            1 => {
                let which = c_visit.which_ready().unwrap();

                for t in get_node_targets(graph, &c_visit.nodes_on) {
                    if actor_can_reach(graph, c_visit.eta[which], c_visit.idx[which], t)  {
                        let n_visit = get_next_visit(graph, &c_visit, which, t);
                        work_queue.push_back(n_visit);
                    }
                }

            },
            2 => {
                let targets: BitSet = get_node_targets(graph, &c_visit.nodes_on).map(|ni| ni.index()).collect();
                let mut zero_targets: BitSet = targets.iter().filter(|t| actor_can_reach(graph, c_visit.eta[0], c_visit.idx[0], NodeIndex::new(*t))).collect();
                let mut one_targets: BitSet = targets.iter().filter(|t| actor_can_reach(graph, c_visit.eta[1], c_visit.idx[1], NodeIndex::new(*t))).collect();
                let common_targets = zero_targets.intersection(&one_targets).collect();
                zero_targets.difference_with(&common_targets);
                one_targets.difference_with(&common_targets);

                // Let's first queue up as if Zero takes an exclusive target
                if zero_targets.len() > 0 {
                    let one_rest = one_targets.union(&common_targets);
                    for z_choice in &zero_targets {
                        for o_choice in one_rest.clone() {
                            let z_idx = NodeIndex::new(z_choice);
                            let o_idx = NodeIndex::new(o_choice);
                            let nv = get_next_visit2(graph, &c_visit, z_idx, o_idx);
                            work_queue.push_back(nv);
                        }
                    }
                }

                // Now for if One takes an exclusive target
                if one_targets.len() > 0 {
                    let zero_rest = zero_targets.union(&common_targets);
                    for z_choice in zero_rest {
                        for o_choice in &one_targets {
                            let z_idx = NodeIndex::new(z_choice);
                            let o_idx = NodeIndex::new(o_choice);
                            let nv = get_next_visit2(graph, &c_visit, z_idx, o_idx);
                            work_queue.push_back(nv);
                        }
                    }
                }

                // Finally if both take a common target
                for combo in common_targets.iter().combinations(2) {
                    let [z_choice, o_choice] = combo[..] else { todo!() };
                    let z_idx = NodeIndex::new(z_choice);
                    let o_idx = NodeIndex::new(o_choice);
                    let nv = get_next_visit2(graph, &c_visit, z_idx, o_idx);
                    work_queue.push_back(nv);
                }
            },
            _ => panic!("This can't actually happen"),
        }
    }

    max_score
}

#[allow(unused)]
const TEST_IN: &str = r#"
Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II
"#;

#[test]
pub fn test_part1() {
    assert_eq!(solve_part1_bfs(&input_generator(TEST_IN)), 1651);
    // assert_eq!(solve_part1_first_pass(&input_generator(TEST_IN)), 1651);
}

#[test]
pub fn test_part2() {
    assert_eq!(solve_part2(&input_generator(TEST_IN)), 1707);
}
