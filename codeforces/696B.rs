use std::io;

fn dfs_sizes(vertex: usize, childs: &Vec<Vec<usize>>, tree_sizes: &mut Vec<usize>) {
    tree_sizes[vertex] = 1;
    let vertex_childs: &Vec<usize> = &childs[vertex];
    for child in vertex_childs {
        dfs_sizes(*child, childs, tree_sizes);
        tree_sizes[vertex] += tree_sizes[*child]
    }
}

fn dfs_result(
    vertex: usize,
    result_from_top: f64,
    childs: &Vec<Vec<usize>>,
    tree_sizes: &Vec<usize>,
    results: &mut Vec<f64>
) {
    results[vertex] = result_from_top;
    let childs_sum_sizes: usize = tree_sizes[vertex] - 1;
    let vertex_childs: &Vec<usize> = &childs[vertex];
    for child in vertex_childs {
        let current_result: f64 = (childs_sum_sizes as f64 - tree_sizes[*child] as f64) / 2.0 + 1.0;
        dfs_result(*child, result_from_top + current_result, childs, tree_sizes, results);
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed read_line");
    let n = input.trim().parse::<usize>().unwrap();
    input.clear();
    io::stdin().read_line(&mut input).expect("failed read_line");
    let parents: Vec<usize> = input
        .trim()
        .split_whitespace()
        .map(|x|x.parse::<usize>().unwrap())
        .collect();
    let mut childs: Vec<Vec<usize>> = Vec::new();
    for _ in 0..n {
        childs.push(Vec::new());
    }
    for vertex in 1..n {
        let parent =  parents[vertex - 1] - 1;
        childs[parent].push(vertex);
    }
    let mut tree_sizes: Vec<usize> = Vec::new();
    tree_sizes.resize(n, 0);
    dfs_sizes(0, &childs, &mut tree_sizes);
    let mut results: Vec<f64> = Vec::new();
    results.resize(n, 0.0);
    dfs_result(0, 1.0, &childs, &tree_sizes, &mut results);
    for result in results {
        print!("{:.6} ", result);
    }
    print!("\n");
}
