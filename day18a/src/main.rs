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

    let mut nodes: Vec<Node> = Vec::new();

    let chunks: Vec<_> = input.lines().enumerate().map(|(_n, line)| {
        let mut prev_node: Option<usize> = None;
        let mut first_node = None;

        let mut add_node = |data: NodeData| {
            let curr_node = Some(nodes.len());

            if let Some(prev_node) = prev_node {
                nodes[prev_node].next = curr_node;
            }

            if first_node.is_none() {
                first_node = curr_node;
            }

            nodes.push(Node {
                prev: prev_node,
                next: None,
                data: data,
            });

            prev_node = curr_node;
        };

        for c in line.chars() {
            match c {
                '[' => {
                    add_node(NodeData::LBracket);
                }

                ']' => {
                    add_node(NodeData::RBracket);
                }

                ',' => {}

                digit => {
                    add_node(NodeData::Num(digit.to_digit(10).unwrap() as u8));
                }
            }
        }

        (first_node.unwrap(), prev_node.unwrap())
    }).collect();

    let sum_start = chunks.into_iter().reduce(|(a_first, a_last), (b_first, b_last)| {
        nodes[a_last].next = Some(b_first);
        nodes[b_first].prev = Some(a_last);

        nodes[a_first].prev = Some(nodes.len());
        let first = nodes.len();
        nodes.push(Node {
            next: Some(a_first),
            prev: None,
            data: NodeData::LBracket,
        });

        nodes[b_last].next = Some(nodes.len());
        let last = nodes.len();
        nodes.push(Node {
            next: None,
            prev: Some(b_last),
            data: NodeData::RBracket,
        });

        'outer: loop {
            let mut prev_node = first;
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

            let mut node = Some(first);
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

        (first, last)
    }).unwrap().0;

    let mut prev: Option<usize> = None;
    let mut curr = Some(sum_start);
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

    println!("{}", val);
}
