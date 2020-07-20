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

fn main() {
    let mut graph: HashMap<&str, HashMap<&str, u32>> = HashMap::new();

    let book: HashMap<&str, u32> = make_edge([("poster", 0), ("lp", 5)].to_vec());
    let poster: HashMap<&str, u32> = make_edge([("bass_guitar", 30), ("drums", 35)].to_vec());
    let lp: HashMap<&str, u32> = make_edge([("bass_guitar", 15), ("drums", 20)].to_vec());

    let bass_guitar: HashMap<&str, u32> = make_edge([("piano", 20)].to_vec());
    let drums: HashMap<&str, u32> = make_edge([("piano", 10)].to_vec());

    graph.insert("book", book);
    graph.insert("poster", poster);
    graph.insert("lp", lp);
    graph.insert("bass_guitar", bass_guitar);
    graph.insert("drums", drums);
    graph.insert("piano", HashMap::new());

    println!("{:#?}", graph);
}
