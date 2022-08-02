use std::collections::HashSet;

fn graph1() -> (usize, usize, HashSet<(usize, usize)>) {
    let k = 6;
    let vertices = 8;
    let edges = vec![
        (1, 2),
        (1, 3),
        (1, 5),
        (1, 6),
        (2, 3),
        (2, 4),
        (3, 4),
        (3, 5),
        (4, 7),
        (6, 7),
        (6, 8),
        (7, 8),
    ]
    .into_iter()
    .map(|(a, b)| (a - 1, b - 1))
    .collect::<HashSet<_>>();
    (k, vertices, edges)
}

fn graph2() -> (usize, usize, HashSet<(usize, usize)>) {
    let k = 4;
    let vertices = 5;
    let edges = vec![(1, 2), (1, 5), (2, 3), (3, 5), (4, 5)]
        .into_iter()
        .map(|(a, b)| (a - 1, b - 1))
        .collect();
    (k, vertices, edges)
}

fn graph3() -> (usize, usize, HashSet<(usize, usize)>) {
    let k = 4;
    let vertices = 7;
    let edges = vec![(1, 4), (2, 6), (2, 7), (3, 7), (4, 5), (4, 6), (4, 7)]
        .into_iter()
        .map(|(a, b)| (a - 1, b - 1))
        .collect();
    (k, vertices, edges)
}

fn enumerate_all(k: usize, vertices: usize, edges: HashSet<(usize, usize)>) -> Vec<Vec<usize>> {
    let mut ans = Vec::new();
    for t in 0..1 << vertices {
        let sub = {
            let mut sub = Vec::with_capacity(vertices);
            for i in 0..vertices {
                if t >> i & 1 == 1 {
                    sub.push(i);
                }
            }
            sub
        };
        if sub.len() != k {
            continue;
        }
        if connected(&sub, vertices, &edges) {
            ans.push(sub)
        }
    }

    for (i, a) in ans.clone().into_iter().enumerate() {
        println!(
            "{i}: {}",
            a.iter()
                .map(|e| e + 1)
                .map(|e| e.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
    println!();
    ans
}

fn main() {
    let (k, vertices, edges) = graph1();

    let ans = enumerate_all(k, vertices, edges.clone());

    let _ans = ans
        .into_iter()
        .map(|mut v| {
            v.sort();
            v
        })
        .collect::<HashSet<_>>();
    let _ans_pro = proximity_search(vertices, &edges, k);

    let (k, vertices, edges) = graph2();
    let _ans = enumerate_all(k, vertices, edges.clone());
    let _ans_pro = proximity_search(vertices, &edges, k);
    // assert_eq!(ans_pro, ans);

    let (k, vertices, edges) = graph3();
    let mut ans = enumerate_all(k, vertices, edges.clone())
        .into_iter()
        .enumerate()
        .map(|(i, v)| (v, i))
        .collect::<Vec<_>>();
    ans.sort();
    for (a, i) in ans {
        println!(
            "{i}: {}",
            a.iter()
                .map(|e| e + 1)
                .map(|e| e.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
    println!();
    let _ans_pro = proximity_search(vertices, &edges, k);
}

fn connected(sub: &[usize], vertices: usize, edges: &HashSet<(usize, usize)>) -> bool {
    let mut visited = vec![false; sub.len()];
    let start = sub[0];
    visited[0] = true;
    in_connected(sub, vertices, edges, start, &mut visited);
    visited.iter().all(|&e| e)
}

fn in_connected(
    sub: &[usize],
    vertices: usize,
    edges: &HashSet<(usize, usize)>,
    start: usize,
    visited: &mut [bool],
) {
    for j in 0..sub.len() {
        if visited[j] {
            continue;
        }
        if edges.contains(&(start, sub[j])) || edges.contains(&(sub[j], start)) {
            visited[j] = true;
            in_connected(sub, vertices, edges, sub[j], visited)
        }
    }
}

fn _reverse_search(
    _vertices: usize,
    _edges: &HashSet<(usize, usize)>,
    _k: usize,
) -> HashSet<Vec<usize>> {
    let _first_ans = {
        // let mut s = Vec::with_capacity(k);
        // for i in 0..vertices {
        //     //
        // }
    };
    unimplemented!()
}

fn proximity_search(
    vertices: usize,
    edges: &HashSet<(usize, usize)>,
    k: usize,
) -> HashSet<Vec<usize>> {
    let mut adj = vec![Vec::new(); vertices];
    for &(u, v) in edges {
        adj[u].push(v);
        adj[v].push(u);
    }
    let mut ans = HashSet::new();
    let mut first_ans = vec![0];
    while first_ans.len() != k {
        for i in 0..vertices {
            if !first_ans.contains(&i) {
                first_ans.push(i);
                if !connected(&first_ans, vertices, edges) {
                    first_ans.pop();
                } else {
                    break;
                }
            }
        }
    }
    first_ans.sort();
    ans.insert(first_ans.clone());
    rec(vertices, edges, k, &mut ans, first_ans);
    for (i, a) in ans.clone().into_iter().enumerate() {
        println!(
            "{i}: {}",
            a.iter()
                .map(|e| e + 1)
                .map(|e| e.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
    println!();
    ans
}

fn rec(
    vertices: usize,
    edges: &HashSet<(usize, usize)>,
    k: usize,
    ans: &mut HashSet<Vec<usize>>,
    s: Vec<usize>,
) {
    for i in 0..k {
        let mut cand = Vec::new();
        for j in 0..k {
            if i == j {
                continue;
            }
            cand.push(s[j]);
        }
        for j in 0..vertices {
            if s.contains(&j) {
                continue;
            }
            cand.push(j);
            if connected(&cand, vertices, edges) {
                let mut new = cand.clone();
                new.sort();
                if ans.contains(&new) {
                    cand.pop();
                    continue;
                }
                ans.insert(new.clone());
                rec(vertices, edges, k, ans, new);
            }
            cand.pop();
        }
    }
}
