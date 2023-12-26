use std::fs::{File, self};
use std::io::{self, BufRead};
use std::path::Path;

use std::cmp::Ordering;
use std::collections::{HashSet, VecDeque};
use std::fmt::{Debug, Error, Formatter};
use std::ops::{Index, IndexMut};
use NodeType::NC;
use hashbag::HashBag;
use rand::{Rng, thread_rng};
use std::{collections::{hash_map,HashMap}, ops::Div};

pub type Node = usize;
pub type Cost = i32;

// ANCHOR: graphs_search_path_utils_Step
#[derive(Debug,Clone,Copy,Hash,Eq,PartialEq)]
pub enum NodeType {
    N(Node),
    NC(Node, Cost)
}
impl From<NodeType> for Node {
    fn from(nt: NodeType) -> Self {
        match nt { NodeType::N(node)|NC(node, _) => node }
    }
}
impl From<Node> for NodeType {
    fn from(value: Node) -> Self {
        NodeType::N(value)
    }
}
impl Ord for NodeType {
    fn cmp(&self, other: &Self) -> Ordering {
        other.partial_cmp(self).unwrap_or_else(|| panic!("Edge::cmp() - cannot compare nodes with type NodeType::N"))
    }
}
impl PartialOrd for NodeType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match other {
            NodeType::N(_) => None,
            NC(_, cost) => {
                let NC(_,sc) = self else { panic!("Edge::partial_cmp() - cannot compare NodeType::NC against NodeType::N") };
                Some(cost.cmp(sc))
            }
        }
    }
}
// ANCHOR_END: graphs_search_path_utils_Step

#[derive(Clone,Copy,Hash,Eq, PartialEq)]
pub struct Edge(pub Node, pub NodeType);

impl Debug for Edge {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("E")
            .field(&self.0)
            .field(&self.1)
            .finish()
    }
}

impl Edge {
    // fn has_node(&self, n:Node) -> bool {
    //     self.0 == n || self.1 == n
    // }
    // fn from(&self, e: &Self) -> bool {
    //     self.0 == e.1 // Edge(4,5).from( Edge(3,4) )
    // }
    // fn to(&self, e: &Self) -> bool {
    //     self.1 == e.0 // Edge(4,5).to( Edge(5,6) )
    // }
    // fn is_adjacent(&self, other:&Self) -> bool {
    //     self.from(other) || self.to(other)
    // }
    // fn is_loop(&self) -> bool {
    //     self.0 == self.1
    // }
    // fn reverse(&mut self) { swap( &mut self.1, &mut self.0); }
    // fn replace_node(&mut self, from:Node, to:Node) {
    //     if self.0 == from { self.0 = to } else if self.1 == from { self.1 = to }
    // }
}

// ANCHOR: graphs_search_path_utils_NodeTrack
#[derive(Debug,Copy,Clone,PartialEq)]
pub enum NodeState {
    Undiscovered,
    Discovered,
    Processed
}
#[derive(Debug,Clone)]
pub struct NodeTrack {
    visited:NodeState,
    dist:Cost,
    parent:Option<Node>
}
impl NodeTrack {
    pub fn visited(&mut self, s:NodeState) -> &mut Self {
        self.visited = s; self
    }
    pub fn distance(&mut self, d:Cost) -> &mut Self {
        self.dist = d; self
    }
    pub fn parent(&mut self, n:Node) -> &mut Self {
        self.parent =Some(n); self
    }
    pub fn is_discovered(&self) -> bool {
        self.visited != NodeState::Undiscovered
    }
}
#[derive(Debug)]
pub struct Tracker {
    list: HashMap<Node, NodeTrack>
}
pub trait Tracking {
    fn extract(&self, start:Node) -> (Vec<Node>, Cost) {
        (self.extract_path(start), self.extract_cost(start))
    }
    fn extract_path(&self, start: Node) -> Vec<Node>;
    fn extract_cost(&self, start: Node) -> Cost;
}
impl Tracking for Tracker {
    fn extract_path(&self, start:Node) -> Vec<Node> {
        let mut path = VecDeque::new();
        // reconstruct the shortest path starting from the target node
        path.push_front(start);
        // set target as current node
        let mut cur_node= start;
        // backtrace all parents until you reach None, that is, the start node
        while let Some(parent) = self[cur_node].parent {
            path.push_front(parent);
            cur_node = parent;
        }
        path.into()
    }
    fn extract_cost(&self, start:Node) -> Cost {
        self[start].dist
    }
}
impl Index<Node> for Tracker {
    type Output = NodeTrack;

    fn index(&self, index: Node) -> &Self::Output {
        self.list.get(&index).unwrap_or_else(|| panic!("Error: cannot find {index} in tracker {:?}", &self))
    }
}
impl IndexMut<Node> for Tracker {
    fn index_mut(&mut self, index: Node) -> &mut Self::Output {
        self.list.get_mut(&index).unwrap_or_else(|| panic!("Error: cannot find {index} in tracker"))
    }
}
// ANCHOR_END: graphs_search_path_utils_NodeTrack
// ANCHOR: graphs_search_path_utils_NodeTrack_graph
impl Graph {

    pub fn get_tracker(&self, visited: NodeState, dist: Cost, parent: Option<Node>) -> Tracker {
        Tracker {
            list: self.nodes.iter()
                .fold(HashMap::new(), |mut out, &node| {
                    out.entry(node)
                        .or_insert(NodeTrack { visited, dist, parent });
                    out
                })
        }
    }
}
// ANCHOR_END: graphs_search_path_utils_NodeTrack_graph

#[derive(PartialEq, Default)]
pub struct Graph {
    pub edges: HashMap<Node, HashSet<NodeType>>,
    pub nodes: HashSet<Node>
}

impl Graph {
    pub fn new() -> Graph {
        Graph::default()
    }
    pub fn export_edges(&self) -> HashSet<Edge> {
        use NodeType::*;

        self.edges.iter()
            .fold( HashSet::<Edge>::new(),|mut edges, (&src, dst_nodes)| {
                dst_nodes.iter()
                    .for_each(|&dst_node| {
                        edges.insert(Edge(src, dst_node));
                        match dst_node {
                            N(dst) => edges.insert(Edge(dst, N(src))),
                            NC(dst,cost) => edges.insert(Edge(dst, NC(src, cost)))
                        };
                    });
                edges
            })
    }
    pub fn import_edges( list: &[Vec<Node>] ) -> Result<Self, Error> {
        let mut graph = Graph::new();

        list.iter().
            map(|edges| {
                (&edges[0],&edges[1..])
            })
            .for_each(|(src, dst)| {
                graph.nodes.insert(*src);
                dst.iter()
                    .for_each(|dst| {
                        graph.edges.entry(*src)
                            .or_default()
                            .insert((*dst).into());
                    })
            });
        Ok(graph)
    }
    pub fn from_edge_list(edge_list: &[(Node, Node, Cost)]) -> Self {
        let mut adjacency_list: HashMap<Node, HashSet<NodeType>> = HashMap::new();
        let mut nodes = HashSet::new();

        for &(source, destination, cost) in edge_list.iter() {
            let destinations = adjacency_list
                .entry(source)
                .or_insert_with(HashSet::new);

            destinations.insert(NC(destination, cost));

            nodes.insert(source);
            nodes.insert(destination);
        }

        Graph {
            edges: adjacency_list,
            nodes,
        }
    }
    pub fn import_text_graph(file: &str, node_pat: char, edge_pat: char) -> Option<Graph> {
        use std::{fs::File, path::Path, io::{BufRead, BufReader}, str::FromStr};

        let mut g = Graph::new();
        let f = File::open(Path::new(file)).unwrap_or_else(|e| panic!("Could not open {file}: {e}"));
        let buf = BufReader::new(f);

        buf.lines().into_iter()
            .enumerate()
            .map(|(num,line)| (num, line.unwrap_or_else(|e| panic!("Cannot read line:{num} from file: {e}") )))
            .for_each(|(num,line)| {
                let mut part = line.split(node_pat);
                let node = Node::from_str(part.next().unwrap()).unwrap_or_else(|e| panic!("Line {num}: Cannot extract Node from line {e}"));
                g.nodes.insert(node);

                for txt in part {
                    let edge = match edge_pat {
                        '\0' => NodeType::N( Node::from_str(txt).unwrap_or_else(|e| panic!("Line {num}: Cannot convert {txt} to Edge {e}")) ),
                        ',' =>
                            if let Some((e_str, c_str)) = txt.split_once(edge_pat) {
                                NC(
                                    Node::from_str(e_str).unwrap_or_else(|e| panic!("Line {num}: Cannot convert {e_str} to Edge {e}")),
                                    Cost::from_str(c_str).unwrap_or_else(|e| panic!("Line {num}: Cannot convert {c_str} to Cost {e}"))
                                )
                            } else {
                                panic!("Cannot convert {txt} into (edge, cost): line {num} ends with a tab ??")
                            },
                        pat => panic!("Unknown delimiter:({}) within txt:({txt}",pat)
                    };
                    g.edges.entry(node)
                        .or_default()
                        .insert(edge);
                }
                // println!("{} -> {:?}",node, g.edges[&node])
            });
        Some(g)
    }
}


impl Clone for Graph {
    fn clone(&self) -> Self {
        Graph {
            edges: self.edges.clone(),
            nodes: self.nodes.clone()
        }
    }
}

impl Debug for Graph {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_map()
            .entries(self.edges.iter())
            .finish()
    }
}

#[derive(Debug)]
pub struct SuperEdges {
    list: HashMap<Node, HashBag<NodeType>>,
    length: usize
}

impl SuperEdges {

    pub fn get_random_edge(&self) -> Edge {
        let mut idx = thread_rng().gen_range(0..self.length);

        let mut iter = self.list.iter();
        if let Some((idx, &node, edges)) = loop {
            if let Some((node,edges)) = iter.next() {
                if idx < edges.len() {
                    break Some((idx, node, edges))
                }
                idx -= edges.len();
            } else {
                break None
            }
        } {
            Edge(
                node,
                edges.iter()
                    .nth(idx)
                    .copied()
                    .unwrap_or_else(|| panic!("get_random_edge(): cannot get dst node at position({idx} from src({node})"))
            )
        } else {
            panic!("get_random_edge(): Couldn't pick a random edge with index({idx}) from src")
        }
    }

    pub fn remove_edge(&mut self, src: Node, dst: Node) {
        // print!("remove_edge(): {:?} IN:{:?} -> Out:", edge, self);
        let edges = self.list.get_mut(&src).unwrap_or_else(|| panic!("remove_edge(): Node({src}) cannot be found within SuperEdges"));
        self.length -= edges.contains(&dst.into());
        while edges.remove(&dst.into()) != 0 { };
        // println!("{:?}",self);
    }
    
    pub fn move_edges(&mut self, old: Node, new: Node) {
        // Fix direction OLD -> *
        let old_edges = self.list
            .remove(&old)
            .unwrap_or_else(|| panic!("move_edges(): cannot remove old node({old})"));
        // print!("move_edges(): {old}:{:?}, {new}:{:?}", old_edges,self.list[&new]);
        self.list.get_mut(&new)
            .unwrap_or_else(|| panic!("move_edges(): failed to extend({new}) with {:?} from({new})", old_edges))
            .extend( old_edges.into_iter());

        // Fix Direction * -> OLD
        self.list.values_mut()
            .filter_map( |e| {
                let count = e.contains(&old.into());
                if  count > 0  { Some((count, e)) } else { None }
            })
            .for_each(|(mut count, edges)| {
                while edges.remove(&old.into()) != 0 {};
                while count != 0 { edges.insert(new.into()); count -= 1; }
            });
        // println!(" -> {:?}",self.list[&new]);
    }
}
// ANCHOR_END: graphs_min_cut_super_edges
// ANCHOR: graphs_min_cut_super_nodes
#[derive(Debug)]
/// Helper Structure that holds the `set` of merged nodes under a super node `key`
/// The HashMap therefore is used as [Key:Super Node, Value: Set of Merged Nodes]
/// A super node's set is a `Graph Component` in itself, that is, you can visit a Node from any other Node within the set
pub struct SuperNodes {
    super_nodes:HashMap<Node,HashSet<Node>>
}
impl Clone for SuperNodes {
    fn clone(&self) -> Self {
        SuperNodes { super_nodes: self.super_nodes.clone() }
    }
}
impl SuperNodes {
    /// Total size of `Graph Components`, that is, super nodes
    pub fn len(&self) -> usize { self.super_nodes.len() }
    /// Given an Graph node, the function returns the Super Node that it belongs
    /// for example given the super node [Key:1, Set:{1,2,3,4,5}]
    /// querying for node `3` will return `1` as its super node
    pub fn find_supernode(&self, node: &Node) -> Node {
        // is this a super node ?
        if self.contains_supernode(node) {
            // if yes, just return it
            *node
        } else {
            // otherwise find its super node and return it
            // get an Iterator for searching each super node
            let mut sets = self.super_nodes.iter();
            loop {
                // If next returns [Super Node, Node Set] proceed otherwise exist with None
                let Some((&src, set)) = sets.next() else { break None };
                // Is the queried Node in the set ?
                if set.contains(node) {
                    // yes, return the super node
                    break Some(src)
                }
            }.unwrap_or_else(|| panic!("find_supernode(): Unexpected error, cannot find super node for {node}"))
        }
    }
    /// Returns the graph component, aka `set` of nodes, for a given super node `key`,
    /// otherwise `None` if it doesn't exist
    pub fn contains_supernode(&self, node: &Node) -> bool {
        self.super_nodes.contains_key(node)
    }
    /// The function takes two super nodes and merges them into one
    /// The `dst` super node is merged onto the `src` super node
    pub fn merge_nodes(&mut self, src:Node, dst:Node) -> &mut HashSet<Node> {
        // remove both nodes that form the random edge and
        // hold onto the incoming/outgoing edges

        let super_src = self.super_nodes.remove(&src).unwrap();
        let super_dst = self.super_nodes.remove(&dst).unwrap();

        // combine the incoming/outgoing edges for attaching onto the new super-node
        let super_node = super_src.union(&super_dst).copied().collect::<HashSet<Node>>();
        // re-insert the src node as the new super-node and attach the resulting union
        self.super_nodes.entry(src).or_insert(super_node)
    }
    /// Provides an iterator that yields the Node Set of each super node
    pub fn iter(&self) -> SuperNodeIter {
        SuperNodeIter { iter: self.super_nodes.iter() }
    }
}
/// Ability for SuperNode struct to use indexing for search
/// e.g super_node[3] will return the HashSet corresponding to key `3`
impl Index<Node> for SuperNodes {
    type Output = HashSet<Node>;
    fn index(&self, index: Node) -> &Self::Output {
        &self.super_nodes[&index]
    }
}

/// HashNode Iterator structure
pub struct SuperNodeIter<'a> {
    iter: hash_map::Iter<'a, Node, HashSet<Node>>
}

/// HashNode Iterator implementation yields a HashSet at a time
impl<'a> Iterator for SuperNodeIter<'a> {
    type Item = &'a HashSet<Node>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(super_node) = self.iter.next() {
            Some(super_node.1)
        } else { None }
    }
}
// ANCHOR_END: graphs_min_cut_super_nodes
// ANCHOR: graphs_min_cut_super_edges_graph
/// Helper Graph functions
impl Graph {
    /// SuperEdges Constructor
    pub fn get_super_edges(&self) -> SuperEdges {
        let mut length = 0;
        let list = self.edges.iter()
            .map(|(&n,e)| (n, e.iter().copied().collect::<HashBag<NodeType>>())
            )
            .inspect(|(_,c)| length += c.len() )
            .collect();
        // println!("get_super_edges(): [{length}]{:?}",list);
        SuperEdges { list, length }
    }
    /// SuperNodes Constructor
    pub fn get_super_nodes(&self) -> SuperNodes {
        SuperNodes {
            super_nodes: self.nodes.iter()
                .map(|&node| (node, HashSet::<Node>::new()))
                .map(|(node, mut map)| {
                    map.insert(node);
                    (node, map)
                })
                .collect::<HashMap<Node, HashSet<Node>>>()
        }
    }
}
// ANCHOR_END: graphs_min_cut_super_edges_graph

pub trait MinimumCut {
    fn minimum_cut(&self) -> Option<Graph>;
    fn contract_graph(&self) -> Option<Graph>;
    fn get_crossing_edges(&self, src_set:&HashSet<Node>, dst_set:&HashSet<Node>) -> Graph;
}

impl MinimumCut for Graph {
    //noinspection RsExternalLinter
    // ANCHOR: graphs_min_cut
    fn minimum_cut(&self) -> Option<Graph> {

        // calculate the number of iterations as N*log(N)
        let nodes = self.nodes.len();
        let mut iterations = nodes as u32 * nodes as u32;
        println!("Run Iterations: {iterations}");

        // initialise min-cut min value and output as Option
        let mut min_cut = usize::MAX;
        let mut result = None;
        let repetitions = iterations as f32;

        // iterate N*log(N) time or exit if min-cut found has only 2 edges
        let mut f = f32::MAX;
        while iterations != 0 && f > 0.088 {

            // contract the graph
            if let Some(graph) = self.contract_graph() {

                // extract the number of edges
                let edges = graph.export_edges();
                // count the edges
                let edges = edges.len();

                // if number of edges returned is smaller than current
                // then store the min-cut returned from this iteration
                if edges <= 6 {
                    min_cut = edges;
                    result = Some(graph);
                    f = (min_cut as f32).div(repetitions);
                    println!("({iterations})({f:.3}) Min Cut !! => {:?}", edges);
                }
        }
            iterations -= 1;
        }
        result
    }
    // ANCHOR_END: graphs_min_cut

    // ANCHOR: graphs_contraction
    fn contract_graph(&self) -> Option<Graph> {

        if self.edges.is_empty() {
            return None;
        }

        // STEP 1: INITIALISE temporary super node and super edge structures
        let mut super_edges = self.get_super_edges();
        let mut super_nodes= self.get_super_nodes();

        // STEP 2: CONTRACT the graph, until 2 super nodes are left
        while super_nodes.len() > 2 {

            // STEP A: select a random edge
                // get a copy rather a reference so we don't upset the borrow checker
                // while we deconstruct the edge into src and dst nodes
            let Edge(src,dst) = super_edges.get_random_edge();
            // println!("While: E({src},{dst}):{:?}",super_edges.list);

            // STEP B : Contract the edge by merging the edge's nodes
                // remove both nodes that form the random edge and
                // hold onto the incoming/outgoing edges
                // combine the incoming/outgoing edges for attaching onto the new super-node
                // re-insert the src node as the new super-node and attach the resulting union
            super_nodes.merge_nodes(src, dst.into());

            // STEP C : Collapse/Remove newly formed edge loops since src & dst is the new super node
            super_edges.remove_edge( src, dst.into());
            super_edges.remove_edge( dst.into(), src);

            // STEP D : Identify all edges that still point to the dst removed as part of collapsing src and dst nodes
            // STEP E : Repoint all affected edges to the new super node src
            super_edges.move_edges(dst.into(), src);
        }

        // STEP 3 : find the edges between the two super node sets
        let mut snode_iter = super_nodes.iter();
        Some(
            self.get_crossing_edges(
                snode_iter.next().expect("There is no src super node"),
                snode_iter.next().expect("There is no dst super node")
            )
        )
    }
    // ANCHOR_END: graphs_contraction

    // ANCHOR: graphs_crossing
    /// Given two Super Node sets the function returns the crossing edges as a new Graph structure
    fn get_crossing_edges(&self, src_set: &HashSet<Node>, dst_set: &HashSet<Node>) -> Graph {
         src_set.iter()
            .map(|src|
                ( src,
                  // get src_node's edges from the original graph
                  self.edges.get(src)
                      .unwrap_or_else(|| panic!("get_crossing_edges(): cannot extract edges for node({src}"))
                      .iter()
                      .map(|&ntype| ntype.into() )
                      .collect::<HashSet<Node>>()
                )
            )
            .map(|(src, set)|
                // Keep only the edges nodes found in the dst_set (intersection)
                // we need to clone the reference before we push them
                // into the output graph structure
                (src, set.intersection(dst_set).copied().collect::<HashSet<Node>>())
            )
            .filter(|(_, edges)| !edges.is_empty() )
            .fold(Graph::new(), |mut out, (&src, edges)| {
                // println!("Node: {node} -> {:?}",edges);
                // add edges: direction dst -> src
                edges.iter()
                    .for_each(|&dst| {
                        out.nodes.insert(dst);
                        out.edges.entry(dst)
                            .or_default()
                            .insert(src.into() );
                    });
                // add edges: direction src -> dst
                out.nodes.insert(src);
                out.edges.insert(
                    src,
                    edges.into_iter().map(|edge| edge.into()).collect()
                );
                out
            })
        // println!("Crossing graph: {:?}", output);
    }
    // ANCHOR_END: graphs_crossing
}

fn part1(filename: &str) -> io::Result<()> {
    let path = Path::new(filename);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut ans = 0;

    let mut label_to_id: HashMap<String, usize> = HashMap::new();
    let mut id_to_label: HashMap<usize, String> = HashMap::new();
    let mut id = 0;

    let mut adj_list: Vec<Vec<Node>> = Vec::new();
    let mut adj_map: HashMap<Node, Vec<Node>> = HashMap::new();

    for line in reader.lines() {
        let line = line.unwrap();
        
        let (src, dst) = line.split_once(":").unwrap();

        for dst in dst.split_whitespace() {
            let mut fsrc = 0;
            {
                let srcs = src.to_string();
                let src_entry = label_to_id.entry(src.to_string());
                let src: &usize = src_entry.or_insert_with(|| {
                    id += 1;
                    id
                });
                fsrc = *src;
                id_to_label.entry(*src).or_insert_with(|| srcs);
            }

            {
                let dsts = dst.to_string();
                let dst_entry = label_to_id.entry(dst.to_string());
                let dst: &usize = dst_entry.or_insert_with(|| {
                    id += 1;
                    id
                });

                id_to_label.entry(*dst).or_insert_with(|| dsts);

                let adj_list = adj_map.entry(fsrc).or_default();
                adj_list.push(*dst);

                let dst_adj_list = adj_map.entry(dst.clone()).or_default();
                dst_adj_list.push(fsrc);
            }
        }
    }


    for (key, value) in adj_map.iter() {
        let mut list = vec![*key];
        list.append(&mut value.clone());

        adj_list.push(list);
    }

    let mut g = Graph::import_edges(&adj_list).expect("Error: Couldn't load input edges");

    let l = g.minimum_cut().unwrap();

    for (key, value) in l.edges.iter() {
        g.edges.entry(*key).and_modify(|e| e.retain(|v| !value.contains(v)));
    }

    fn dfs(graph: &Vec<Vec<usize>>, node: usize, visited: &mut Vec<bool>) -> usize {
        visited[node] = true;
        let mut count = 1;
        for &adj_node in &graph[node] {
            if !visited[adj_node] {
                count += dfs(graph, adj_node, visited);
            }
        }

        count
    }
    
    fn find_connected_components(graph: &Vec<Vec<usize>>) -> usize {
        let n = graph.len();
        let mut visited = vec![false; n];
        
        dfs(graph, 1, &mut visited)
    }

    let mut graph = vec![vec![]; g.nodes.len() + 1];
    for (key, value) in g.edges.iter() {
        graph[*key] = value.iter().map(|v| usize::from(*v)).collect();
    }

    let component1 = find_connected_components(&graph);
    let component2 = g.nodes.len() - component1;

    let ans = component1*component2;

    println!("{}, {}", component1, component2);

    println!("AoC 2023: {}", ans);

    Ok(())
}

fn part2(_filename: &str) -> io::Result<()> {
    Ok(())
}

fn main() {
    part1("./src/input.txt").unwrap();
    part2("./src/input.txt").unwrap();
}
