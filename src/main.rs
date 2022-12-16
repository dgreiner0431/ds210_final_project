use csv::Reader;
use std::error::Error;
use std::vec::Vec;
use core::cmp::Reverse;
use std::collections::BinaryHeap;



#[derive(Debug,Copy,Clone)]
struct Outedge {
    vertex: usize,
    length: usize,
}

#[derive(Debug)]
struct Graph {
    n: usize,
    outedges: Vec<AdjacencyList>,
}

type Vertex = usize;
type Distance = usize;
type Edge = (Vertex, Vertex, Distance);

type AdjacencyList = Vec<Outedge>;

impl Graph {
    fn create_directed(n: usize, edges: &Vec<Edge>) -> Graph {
        let mut outedges = vec![vec![]; n];
        for (u, v, length) in edges {
            outedges[*u].push(Outedge {
                vertex: *v,
                length: *length,
            });
        }
        Graph { n, outedges }
    }

    fn create_edges_from_csv(csv_file: &str) -> Result<Graph, Box<dyn Error>> {
        let mut reader = Reader::from_path(csv_file)?;
        let mut edges = Vec::new();
        let mut nodes = Vec::new();

        for result in reader.records() {
            let record = result?;
            let col5 = record[4].to_string();
            let col7 = record[6].to_string();
            nodes.push((col5, col7));
        }

        let n = nodes.len();

        for i in 0..nodes.len() {
            for j in 0..nodes.len() {
                if nodes[i].0 == nodes[j].0 || nodes[i].1 == nodes[j].1 {
                    edges.push((i, j, 0));
                }
            }
        }

        Ok(Self::create_directed(n, &edges))
    }
}

fn dijkstra(graph: Graph) -> Vec<Option<Distance>> {
    //pick starting node
    let start: Vertex = 0;

    //set tentative distance for start node = 0
    //set tentative distance for all other nodes equal to infinity
    let mut distances: Vec<Option<Distance> > = vec![None; graph.n];
    distances[start] = Some(0);


    let mut pq = BinaryHeap::<Reverse<(Distance,Vertex)>>::new();
    pq.push(Reverse((0,start)));

    /*
    Here, we consider all unvisited neighbors and calculate distances through current node
    we mark the node as visited and remove it from unvisted set
     */
    while let Some(Reverse((dist,v))) = pq.pop() {
        for Outedge{vertex,length} in graph.outedges[v].iter() {
            let new_dist = dist + *length;
            let update = match distances[*vertex] {
                None => {true} |
                Some(d) => {new_dist < d}
            };
            if update {
                distances[*vertex] = Some(new_dist);
                pq.push(Reverse((new_dist,*vertex)));
            }
        }
    };
    return distances
}


fn main() -> Result<(), Box<dyn Error>> {
    let _graph = Graph::create_edges_from_csv("famous.csv")?;

    println!("{:?}",_graph);

    //I will now begin the implementation of Dijkstra's shortest_paths
    dijkstra(_graph);


    Ok(())

    #[test]
    fn test_dist1() {
        let graph = Graph::create_edges_from_csv("famous.csv").unwrap();
        let distances = dijkstra(graph);
        let test_dist = distances[0];
        assert_eq!(test_dist, Some<0>); 
        //the distance from a node to itself should be equal to zero
    }
}


