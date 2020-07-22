use std::collections::HashMap;

//      
// Book ->(0) Poster ->(30) bass_guitar ->(20) piano
//                   ->(35) drums ->(10) piano
//      ->(5) lp ->(15) bass_guitar ->(20) piano
//               ->(20) drums ->(10) piano
//      

fn make_edge(edges_with_weights: Vec<(&str, u32)>) -> HashMap<&str, u32> {
    let mut node: HashMap<&str, u32> = HashMap::new();
    for (edge, weight) in &edges_with_weights {
        node.insert(edge, *weight);
    }
    node
}

fn find_lowest_cost_node(costs: HashMap<&str, Option<u32>>) -> Option<(&str, u32)> {
    costs
}

fn main() {
    let mut graph: HashMap<&str, HashMap<&str, u32>> = HashMap::new();
    //
    // NOTE:
    // The None type of the option represents infinity for costs.
    //
    let mut costs: HashMap<&str, Option<u32>> = HashMap::new();
    let mut parents: HashMap<&str, &str> = HashMap::new();
    let mut processed: Vec<&str> = Vec::new();

    let entries = [
        ("book", [("poster", 0), ("lp", 5)].to_vec()),
        ("poster", [("bass_guitar", 30), ("drums", 35)].to_vec()),
        ("lp", [("bass_guitar", 15), ("drums", 20)].to_vec()),
        ("bass_guitar", [("piano", 20)].to_vec()),
        ("drums", [("piano", 10)].to_vec()),
        ("piano", [].to_vec()),
    ];

    for (entry, edges) in &entries {
        graph.insert(entry, make_edge(edges.to_vec()));
    }

    let mut node = find_lowest_cost_node(costs);


    println!("{:#?}", graph);
}
