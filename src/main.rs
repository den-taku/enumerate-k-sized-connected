use std::collections::HashSet;

fn main() {
    let k = 7;
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

    let ans = ans
        .into_iter()
        .map(|mut v| {
            v.sort();
            v
        })
        .collect::<HashSet<_>>();

    let rev_ans = reverse_search(vertices, &edges, k);
    assert_eq!(rev_ans, ans)
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
        if edges.contains(&(start, j)) || edges.contains(&(j, start)) {
            visited[j] = true;
            in_connected(sub, vertices, edges, j, visited)
        }
    }
}

fn reverse_search(
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
