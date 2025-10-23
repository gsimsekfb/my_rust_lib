// last: 10/25

// Adjacency [List and Matrix] representation of undirected and 
// unweighted graph

// interv-1
// - impl fns: graph add_edge_matrix, add_edge_list, print_graph for 
//   the graph below:
// 
// Graph: vertices/nodes = 4, edges: 4 
//
//      0
//     / \
//    1---2
//         \
//          3
//
// Hint: Representations at the end




// -------------------------------------------------------





type Graph = Vec<Vec<usize>>;

fn add_edge_matrix(graph: &mut Graph, i: usize, j: usize) {
    graph[i][j] = 1;
    graph[j][i] = 1;
}

fn add_edge_list(graph: &mut Graph, i: usize, j: usize) {
    graph[i].push(j);
    graph[j].push(i);
}


fn print_graph(graph: &Graph) {
    for (i, elems) in graph.iter().enumerate() {
        print!("{i}: ");
        for e in elems { print!("{e} "); }
        println!("");
    }
}

#[test]
fn tt() {
    let mut graph: Graph = vec![vec![0;4];4];
    add_edge_matrix(&mut graph, 0, 1);
    add_edge_matrix(&mut graph, 0, 2);
    add_edge_matrix(&mut graph, 1, 2);
    add_edge_matrix(&mut graph, 2, 3);
    println!("");
    print_graph(&graph);

    let graph: Graph = vec![
        vec![ 0, 1, 1, 0 ],
        vec![ 1, 0, 1, 0 ],
        vec![ 1, 1, 0, 1 ],
        vec![ 0, 0, 1, 0 ]
    ];
    println!("");
    print_graph(&graph);

    let mut graph: Graph = vec![vec![];4]; 
    add_edge_list(&mut graph, 0, 1);
    add_edge_list(&mut graph, 0, 2);
    add_edge_list(&mut graph, 1, 2);
    add_edge_list(&mut graph, 2, 3);
    println!("");
    print_graph(&graph);
}


//// Hint:

// "Adjacency [List and Matrix] Representation"
// e.g.
// Graph: vertices/nodes = 4, edges: 4 
//
//      0
//     / \
//    1---2
//         \
//          3
// 
// Adjacency Matrix Representation
// 0 1 1 0 
// 1 0 1 0 
// 1 1 0 1 
// 0 0 1 0 
// 
// Adjacency List Representation:
// 0: 1 2 
// 1: 0 2 
// 2: 0 1 3 
// 3: 2 