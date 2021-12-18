use std::io::Read;

#[derive(Debug, Clone, Copy, PartialEq)]
enum NodeData {
    Num(u8),
    LBracket,
    RBracket,
}

#[derive(Debug)]
struct Node {
    next: Option<usize>,
    prev: Option<usize>,
    data: NodeData,
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let nums: Vec<_> = input.lines().enumerate().map(|(_n, line)| {
        let mut nodes: Vec<NodeData> = Vec::new();

        for c in line.chars() {
            match c {
                '[' => {
                    nodes.push(NodeData::LBracket);
                }

                ']' => {
                    nodes.push(NodeData::RBracket);
                }

                ',' => {}

                digit => {
                    nodes.push(NodeData::Num(digit.to_digit(10).unwrap() as u8));
                }
            }
        }

        nodes
    }).collect();

    println!("{}", nums.iter().enumerate().flat_map(|(i, a)| {
        nums.iter().skip(i + 1).map(|b| {
            let mut nodes: Vec<Node> = Vec::new();

            let mut add_node = |data: NodeData| {
                let prev_node = if nodes.len() == 0 {
                    None
                } else {
                    let prev = nodes.len() - 1;
                    nodes[prev].next = Some(nodes.len());
                    Some(prev)
                };

                nodes.push(Node {
                    prev: prev_node,
                    next: None,
                    data: data,
                });
            };

            add_node(NodeData::LBracket);
            for &data in a.iter().chain(b) {
                add_node(data);
            }
            add_node(NodeData::RBracket);

            'outer: loop {
                let mut prev_node = 0;
                let mut depth = 0;

                while let Some(curr_node) = nodes[prev_node].next {
                    match (nodes[prev_node].data, nodes[curr_node].data) {
                        (NodeData::LBracket, _) => {
                            depth += 1;
                        }

                        (NodeData::RBracket, _) => {
                            depth -= 1;
                        }

                        (NodeData::Num(a), NodeData::Num(b)) => {
                            if depth >= 5 {
                                let mut node = nodes[prev_node].prev;

                                while let Some(curr) = node {
                                    match &mut nodes[curr].data {
                                        NodeData::Num(num) => {
                                            *num += a;
                                            break;
                                        }

                                        _ => {
                                            node = nodes[curr].prev;
                                        }
                                    }
                                }

                                node = nodes[curr_node].next;

                                while let Some(curr) = node {
                                    match &mut nodes[curr].data {
                                        NodeData::Num(num) => {
                                            *num += b;
                                            break;
                                        }

                                        _ => {
                                            node = nodes[curr].next;
                                        }
                                    }
                                }

                                nodes[curr_node].data = NodeData::Num(0);
                                let node = (0..2).fold(curr_node, |node, _| nodes[node].next.unwrap());
                                nodes[node].prev = Some(curr_node);
                                nodes[curr_node].next = Some(node);
                                let node = (0..2).fold(prev_node, |node, _| nodes[node].prev.unwrap());
                                nodes[node].next = Some(curr_node);
                                nodes[curr_node].prev = Some(node);
                                prev_node = node;
                                depth -= if nodes[prev_node].data == NodeData::LBracket {
                                    2
                                } else {
                                    1
                                };
                                continue;
                            }
                        }

                        _ => {}
                    }

                    prev_node = curr_node;
                }

                let mut node = Some(0);
                while let Some(curr) = node {
                    if let NodeData::Num(num) = nodes[curr].data {
                        if num >= 10 {
                            let prev = nodes[curr].prev.unwrap();
                            let next = nodes[curr].next.unwrap();

                            let lbracket = nodes.len();
                            nodes.push(Node {
                                prev: Some(prev),
                                next: None,
                                data: NodeData::LBracket,
                            });

                            let lnum = nodes.len();
                            nodes.push(Node {
                                prev: Some(lbracket),
                                next: Some(curr),
                                data: NodeData::Num(num / 2),
                            });

                            let rbracket = nodes.len();
                            nodes.push(Node {
                                prev: Some(curr),
                                next: Some(next),
                                data: NodeData::RBracket,
                            });

                            nodes[prev].next = Some(lbracket);
                            nodes[next].prev = Some(rbracket);
                            nodes[lbracket].next = Some(lnum);
                            nodes[curr].prev = Some(lnum);
                            nodes[curr].next = Some(rbracket);
                            nodes[curr].data = NodeData::Num((num - 1) / 2 + 1);

                            continue 'outer;
                        }
                    }

                    node = nodes[curr].next;
                }

                break;
            }

            let mut prev: Option<usize> = None;
            let mut curr = Some(0);
            let mut stack = Vec::new();
            let mut val = 0;
            let mut depth = 0;

            while let Some(node) = curr {
                if let Some(prev) = prev {
                    match (nodes[prev].data, nodes[node].data) {
                        (NodeData::Num(_) | NodeData::RBracket, NodeData::Num(_)| NodeData::LBracket) => {
                            stack.push((depth, val));
                        }

                        _ => {}
                    }
                }

                match nodes[node].data {
                    NodeData::Num(num) => {
                        val = num as u16;
                    }

                    NodeData::LBracket => {
                        depth += 1;
                    }

                    NodeData::RBracket => {
                        depth -= 1;
                    }
                }

                if let Some(&(stack_depth, stack_val)) = stack.last() {
                    if depth < stack_depth {
                        stack.pop();
                        val = stack_val * 3 + val * 2;
                    }
                }

                prev = curr;
                curr = nodes[node].next;
            }

            val
        })
    }).max().unwrap());
}
