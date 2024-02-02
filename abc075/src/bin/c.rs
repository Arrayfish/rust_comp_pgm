/**
 * 最大でかかる計算数
 * 最大2500個の辺の橋候補{
 *   最大2500個の辺を見て登録
 *   50個のノードを確認
 * }
 * 参照関連が大変すぎる。indexベースで考えた方がマシ。
 * 途中
 */
// use proconio::input;

// #[derive(Clone)]
// struct Node<'a>{
//     conn: Vec<&'a Node<'a>>,
//     parent: Option<Box<Node<'a>>>,
// }

// impl<'a> Node<'a>{
//     fn get_root(self)->Node<'a>{
//         if let Some(p_node) = self.parent {
//             p_node.get_root()
//         }else{
//             self
//         }
//     }
// }

// fn main() {
//     input! {
//         n: usize,
//         m: usize,
//         ab: [[usize; 2];m],
//     }
//     let mut init_nodes: Vec<Node> = Vec::new();
//     for _ in 0..n{
//         init_nodes.push(Node {conn: Vec::new(), parent: None });
//     }
//     // candidate of bridge
//     for i in 0..m{
//         // create node condition
//         let mut cu_nodes = init_nodes.clone();
//         for j in 0..m{
//             if i != j{
//                 cu_nodes[ab[j][0]-1].conn.push(&cu_nodes[ab[j][1]-1]);
//                 cu_nodes[ab[j][1]-1].conn.push(&cu_nodes[ab[j][0]-1]);
//             }
//         }
//         // set parent and
//         cu_nodes[0].parent = Some(Box::new(cu_nodes[0]));
//         for a_node in cu_nodes{
            
//         }
//     }
    
// }
use proconio::input;

struct Node{
    idx: usize,
    conn: Vec<usize>,
    parent: Option<usize>,
}

fn get_root_idx(node_idx: usize, nodes: Vec<Node>) -> usize{
    if let Some(p_idx) = nodes[node_idx].parent{
        if p_idx == nodes[node_idx].idx{
            p_idx
        }else{
            get_root_idx(p_idx, nodes)
        }
    }else{
        panic!("hey")
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [[usize; 2];m],
    }
}