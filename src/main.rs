use petgraph::algo::k_shortest_path;
use petgraph::dot::Dot;
use petgraph::graph::{Graph, NodeIndex};
use std::fs::File;
use std::io::Write;

fn main() {
    let mut graph: Graph<String, f32, petgraph::Undirected> = moravia();

    let start = graph.node_indices().find(|&n| graph[n] == "Brno").unwrap();
    let goal = graph
        .node_indices()
        .find(|&n| graph[n] == "Jemnice")
        .unwrap();

    let k = 1;
    let edge_cost = |e: petgraph::graph::EdgeReference<f32>| *e.weight();

    // Find k-shortest paths from start to goal
    let paths = k_shortest_path(&graph, start, Some(goal), k, edge_cost);

    // Extract the paths if they are stored properly
    // You may need to adjust how the paths are structured when returned
    for path in paths {
        // Assuming path is Vec<NodeIndex> structured directly
        let path_indices: Vec<NodeIndex> = vec![path.0]; // Adjust based on the actual return type

        visualize_path(&graph, path_indices);
        break;
    }

    // Optionally, print the entire graph in DOT format for visualization
    // println!("{:?}", Dot::new(&graph));
}

pub fn moravia() -> Graph<String, f32, petgraph::Undirected> {
    let mut g = Graph::new_undirected();
    let brno = g.add_node("Brno".to_string());
    let zdlch = g.add_node("Židlochovice".to_string());
    let pohor = g.add_node("Pohořelice".to_string());
    let vysko = g.add_node("Vyškov".to_string());
    let blansk = g.add_node("Blansko".to_string());
    let trebic = g.add_node("Třebíč".to_string());
    let mbud = g.add_node("Mor. Buďějovice".to_string());
    let jihl = g.add_node("Jihlava".to_string());
    let jemn = g.add_node("Jemnice".to_string());
    let znojmo = g.add_node("Znojmo".to_string());
    let novmest = g.add_node("Nové Město".to_string());
    let mtreb = g.add_node("Mor. Třebová".to_string());
    g.add_edge(brno, trebic, 87.5);
    g.add_edge(brno, zdlch, 21.9);
    g.add_edge(brno, vysko, 43.1);
    g.add_edge(brno, blansk, 26.4);
    g.add_edge(pohor, zdlch, 11.7);
    g.add_edge(pohor, trebic, 80.0);
    g.add_edge(blansk, mtreb, 61.8);
    g.add_edge(trebic, mbud, 27.3);
    g.add_edge(mbud, znojmo, 56.6);
    g.add_edge(brno, znojmo, 101.6);
    g.add_edge(mbud, jemn, 39.0);
    g.add_edge(jihl, trebic, 45.1);
    g.add_edge(jihl, jemn, 67.3);
    g.add_edge(jemn, znojmo, 82.6);
    g.add_edge(pohor, znojmo, 80.8);
    g.add_edge(novmest, jihl, 64.5);
    g.add_edge(novmest, brno, 87.6);
    g.add_edge(novmest, trebic, 70.9);
    g.add_edge(novmest, blansk, 75.0);
    g.add_edge(novmest, mtreb, 89.4);
    g.add_edge(vysko, blansk, 37.0);
    g.add_edge(vysko, zdlch, 56.9);
    g
}

fn visualize_path(graph: &Graph<String, f32, petgraph::Undirected>, path: Vec<NodeIndex>) {
    let mut dot_string = String::from("digraph G {\n");

    // Add edges of the found path with solid lines
    for window in path.windows(2) {
        if let [start, end] = window {
            let start_label = &graph[*start];
            let end_label = &graph[*end];
            dot_string.push_str(&format!("    \"{}\" -> \"{}\";\n", start_label, end_label));
        }
    }

    // Create a set of unique index pairs for the solid lines
    let path_edges: std::collections::HashSet<(NodeIndex, NodeIndex)> = path
        .windows(2)
        .filter_map(|window| {
            if let [start, end] = window {
                Some((*start, *end))
            } else {
                None
            }
        })
        .collect();

    // Add the remaining edges as dotted lines
    for edge in graph.edge_indices() {
        let (start, end) = graph.edge_endpoints(edge).unwrap();
        let start_label = &graph[start];
        let end_label = &graph[end];

        // Only add dotted lines if not part of the path
        if !path_edges.contains(&(start, end)) && !path_edges.contains(&(end, start)) {
            dot_string.push_str(&format!(
                "    \"{}\" -> \"{}\" [style=dotted];\n",
                start_label, end_label
            ));
        }
    }

    dot_string.push_str("}\n");

    // Write to a DOT file
    let mut file = File::create("path_visualization.dot").unwrap();
    file.write_all(dot_string.as_bytes()).unwrap();
}
