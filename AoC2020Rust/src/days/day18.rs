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
            match c {
                '(' => {
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
                    // Keep each layer at the same priority
                    if current_priority as usize <= layer_starts.len()-1 {
                        current_priority += 1;
                    }
                    current_operation = "+";
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
                                    layer_starts.push(current_id);
                                } else {
                                    layer_starts.push(potential_parent);
                                }
                            }
                        }
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

                        current_parent = current_id;
                        current_id += 1;
                    } else {
                        println!("Expected num, found {}", c);
                    }
                }
            }
        }

        for p_offset in 0..=max_priority {
            for i in 0..node_map.len() {
                let node = node_map[&((node_map.len()-i-1) as i64)];
                if node.priority == max_priority - p_offset && node.id != trunk_id {
                    let mut parent_node = node_map[&node.parent];
                    match node.operation {
                        "+" => parent_node.value += node.value,
                        "*" => parent_node.value *= node.value,
                        _=>()
                    }
                    node_map.insert(parent_node.id, parent_node);
                }
            }
        }
        let result_of_line = node_map[&trunk_id].value;
        part_two_sum_of_line_results += result_of_line;
    }
    // Part two sum of line results is 171259538712010
    println!("Part two sum of line results is {}", part_two_sum_of_line_results);
}