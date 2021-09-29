/// Graph - Undirected - Bridges (橋検出)
use crate::graph::undirected::lowlink::*;
pub fn bridges(g: &Vec<Vec<usize>>) -> Vec<(usize, usize)> {
    let lowlink = LowLink::new(&g);
    let mut es = vec![];
    for u in 0..lowlink.size {
        for &v in g[u].iter() {
            if lowlink.ord[u] < lowlink.low[v] {
                es.push((u, v));
            }
        }
    }
    es
}

#[cfg(test)]
mod test_bridges {
    use crate::graph::undirected::bridges::*;

    #[test]
    fn test_bridges() {
        macro_rules! add {
            ($g:expr, $i:expr, $j:expr) => {
                $g[$i].push($j);
                $g[$j].push($i);
            };
        }

        let mut g = vec![vec![]; 10];
        add!(g, 0, 1);
        add!(g, 1, 2);
        add!(g, 1, 3);
        add!(g, 1, 4);
        add!(g, 3, 4);
        add!(g, 0, 5);
        add!(g, 5, 6);
        add!(g, 5, 9);
        add!(g, 6, 7);
        add!(g, 6, 8);
        add!(g, 6, 9);
        add!(g, 8, 9);

        let bs = bridges(&g);
        assert_eq!(bs, vec![(0, 1), (0, 5), (1, 2), (6, 7),]);
    }
}
