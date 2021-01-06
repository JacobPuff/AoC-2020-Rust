use std::collections::HashMap;
use crate::days;

#[derive(Clone, Copy, Debug)]
struct Node {
    id: i64,
    parent: i64,
    priority: i64,
    value: i64,
    operation: &'static str
}

// impl Copy for Node {
//     fn Copy(&self, other: &Self) -> Node {
//         let copied_node: Node = Node {
//             id: self.id,
//             parent: self.parent,
//             children: self.children.to_vec(),
//             value: self.value,
//             operation: self.operation
//         };
//         return copied_node
//     }
// }

pub fn day18() {
    part_one();
    part_two();
}

fn part_one() {
    let data: Vec<String> = days::tools::open_file_from_path("src/days/day18data.txt");
    let mut part_one_sum_of_line_results = 0;
    for line in data {
        let mut order_of_priority: Vec<i64> = Vec::new();
        order_of_priority.push(0);
        let mut operation_at_priority: Vec<&str> = Vec::new();
        operation_at_priority.push("+");
        let mut current_priority_idx: usize = 0;
        for c in line.chars() {
            match c {
                '(' => {
                    current_priority_idx += 1;
                    order_of_priority.push(0);
                    operation_at_priority.push("+");
                },
                ')' => {
                    current_priority_idx -= 1;
                    // Remove the operation for the current set of parenthesis
                    operation_at_priority.pop();
                    let operation = operation_at_priority[current_priority_idx];
                    if operation == "+" {
                        order_of_priority[current_priority_idx] += order_of_priority.pop().unwrap();
                    } else if operation == "*" {
                        order_of_priority[current_priority_idx] *= order_of_priority.pop().unwrap();
                    }
                },
                ' ' => (),
                '+' => operation_at_priority[current_priority_idx] = "+",
                '*' => operation_at_priority[current_priority_idx] = "*",
                _ => {
                    if let Some(num) = c.to_digit(10) {
                        if operation_at_priority[current_priority_idx] == "+" {
                            order_of_priority[current_priority_idx] += num as i64
                        } else if operation_at_priority[current_priority_idx] == "*" {
                            order_of_priority[current_priority_idx] *= num as i64
                        }
                    } else {
                        println!("Expected num, found {}", c);
                    }
                }
            }
        }
        
        let result_of_line = order_of_priority[0];
        assert_eq!(result_of_line, order_of_priority[current_priority_idx]);
        part_one_sum_of_line_results += result_of_line;
    }
    println!("Part one sum of line results is {}", part_one_sum_of_line_results);
}


fn part_two() {
    let data: Vec<String> = days::tools::open_file_from_path("src/days/day18data.txt");
    let mut part_two_sum_of_line_results = 0;
    for line in data {
        let mut node_map: HashMap<i64, Node> = HashMap::new();
        let trunk_id: i64 = 0;
        let trunk_node = Node {
            id: trunk_id,
            parent: -1,
            priority: -1,
            value: 0,
            operation: "+"
        };
        node_map.insert(trunk_id, trunk_node);
        let mut layer_starts = Vec::new();
        layer_starts.push(trunk_id);
        let mut current_id = 1;
        let mut current_parent = 0;
        let mut current_operation = "+";
        let mut current_priority = 1;
        let mut max_priority = 0;
        for c in line.chars() {
            // if c != ' ' {
            //     println!("Current Parent is {} c is {}", current_parent, c);
            // }
            match c {
                '(' => {
                    // 2+(3+4)*(5+6)
                    // Always add 2, because the first num in the first paren has higher priority.
                    current_priority += 2;
                    layer_starts.push(current_id);
                    layer_starts.push(current_id);
                },
                ')' => {
                    layer_starts.pop().unwrap();
                    current_parent = layer_starts.pop().unwrap();
                    current_priority -= 2;
                },
                ' ' => (),
                '+' => {
                    // Try to keep these at the same priority.
                    if current_priority as usize <= layer_starts.len()-1 {
                        current_priority += 1;
                    }
                    current_operation = "+";

                    // These stopped being about the + operation, and became a general list.

                    // Priority is only set back by one, and not set back to the start. CHECKED
                    // How can we check that this has only be raised once? CHECKED
                    // Can we guarantee that a node exists, or should we just push priority? CHECKED
                    // In input 1 + (2 * 3) + (4 * (5 + 6)):
                    //   Priority matches up between 3 and 4. CHECKED this is actually ok.
                    //   Parent of 4 is 3, but should be 2. When the parent is set to 2,
                    //       once 4 is reached the parent is set to current id, which is 3. CHECKED


                    // TODO
                    // Walk through the program with 1 + (2 * 3) + (4 * (5 + 6)) as input to figure
                    //     out why double parens break things. CHECKED
                    // Fix double parens. CHECKED not actually the problem.
                    // Fix parens next to eachother. CHECKED they now have correct parents.
                    // Fix order of operations. 4 should not add to 2 before 3 is multiplied. CHECKED
                    // In input ((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2:
                    //   The last 3 nodes are not processed. CHECKED
                    // Walk through program to solve above.
                },
                '*' => {
                    current_operation = "*";
                },
                _ => {
                    if let Some(num) = c.to_digit(10) {
                        let mut beginning_of_paren = false;
                        let mut beginning_of_paren_offset: i64 = 0;
                        if layer_starts[layer_starts.len()-1] == current_id {
                            beginning_of_paren = true;
                            if layer_starts.len() < 2 {
                                break
                            }
                            for i in 1..layer_starts.len()/2 + 1 {
                                if layer_starts[layer_starts.len()-(i*2)] == current_id {
                                    current_priority -= 2;
                                    beginning_of_paren_offset += 2;
                                } else {
                                    break
                                }
                            }
                        }
                        if current_operation == "*" {
                            if beginning_of_paren {
                                current_parent = layer_starts[layer_starts.len() - beginning_of_paren_offset as usize-1];
                            } else if layer_starts.len() != 0 {
                                let potential_parent = layer_starts.pop().unwrap();
                                if potential_parent != current_id {
                                    current_parent = potential_parent;
                                    // println!("{:?}", node_map[&current_parent]);
                                    // current_priority = node_map[&current_parent].priority;
                                    // println!("{}", current_parent);
                                    layer_starts.push(current_id);
                                } else {
                                    layer_starts.push(potential_parent);
                                }
                            }
                        }
                        // println!("{} CPy", current_priority);
                        if current_priority > max_priority {
                            max_priority = current_priority;
                        }
                        let new_node: Node = Node{
                            id: current_id,
                            parent: current_parent,
                            priority: current_priority,
                            value: num as i64,
                            operation: current_operation
                        };
                        node_map.insert(current_id, new_node);
                        
                        if current_operation == "+" {
                            current_priority -= 1;
                        }
                        if beginning_of_paren {
                            current_priority += beginning_of_paren_offset;
                        }

                        // println!("{} CPy", current_priority);
                        current_parent = current_id;
                        current_id += 1;
                    } else {
                        println!("Expected num, found {}", c);
                    }
                }
            }
        }
        // println!("Yeet");
        // println!("{:?}", node_map);
        for p_offset in 0..=max_priority {
            // let mut new_nodes: Vec<Node> = Vec::new();
            // reversed_node_map.reverse();
            // let mut node_loop_vec = &mut node_map;
            // node_loop_vec.push(node_map[node_map.len()-1]);
            for i in 0..node_map.len() {
                let node = node_map[&((node_map.len()-i-1) as i64)];
                if node.priority == max_priority - p_offset && node.id != trunk_id {
                    // println!("current_node is {:?}", node);

                    // If 2 add into 1, and 1 is the same priority, than 1 wont be changed
                    // for the next operation CHECKED
                    let mut parent_node = node_map[&node.parent];
                    // println!("{:?}", node);
                    // println!("op: {}", node.operation);
                    // println!("pr: {}", parent_node.value);
                    match node.operation {
                        "+" => parent_node.value += node.value,
                        "*" => parent_node.value *= node.value,
                        _=>()
                    }
                    // println!("pr: {}\n", parent_node.value);

                    // node_map.remove(&parent_node.id);
                    node_map.insert(parent_node.id, parent_node);
                    // println!("{:?}", node_map);
                }
            }
            // new_nodes.reverse();
            // node_map = new_nodes;
        }
        // println!("{:?}", node_map);

        let result_of_line = node_map[&trunk_id].value;
        part_two_sum_of_line_results += result_of_line;
    }
    // Part two sum of line results is 171259538712010
    println!("Part two sum of line results is {}", part_two_sum_of_line_results);
}