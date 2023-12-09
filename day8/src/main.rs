use std::fs::read_to_string;
use std::collections::HashMap;

fn traverse(graph: &HashMap<String, (String, String)>, start_node: &str, instructions: &[char]) -> usize {
    let mut current_node = start_node;
    let mut call_count = 0;
    let mut inst_index = 0;

    loop {
        call_count += 1;

        if let Some((left, right)) = graph.get(current_node) {
            // if we reached ZZZ we are done
            if current_node == "ZZZ" {
                println!("Reached final node: {}", current_node);
                break;
            }
            // Determine the next node based on the current instruction
            current_node = match instructions[inst_index] {
                'L' => left,
                'R' => right,
                _ => {
                    println!("Invalid instruction at node: {}", current_node);
                    break;
                }
            };
            // Calculate the next instruction index, wrapping around if necessary
            inst_index = (inst_index + 1) % instructions.len();
        } else {
            println!("Reached final node: {}", current_node);
            break;
        }
    }

    call_count
}

fn main() -> std::io::Result<()> {
    let input_file = "input.txt";

    let mut graph: HashMap<String, (String,String)> = HashMap::new();

    println!("Input file: {:?}", input_file);

    // read file line by line
    let file_str = read_to_string(input_file)?;

    let (instructions_str, map_str) = file_str.split_once("\n\n").unwrap();

    let instructions: Vec<char> = instructions_str.chars().collect();

    for connection in map_str.lines() {
        let (node_str, choices) = connection.split_once(" = ").unwrap();
        let node = node_str.trim().chars().collect::<String>();
        let (left, right) = choices.split_once(",").unwrap();
        let left = left.chars().filter(|c| c.is_alphabetic()).collect::<String>();
        let right = right.chars().filter(|c| c.is_alphabetic()).collect::<String>();
        println!("Node: {} => {},{}", node, left, right);
        graph.insert(node, (left, right));
    }

    println!("Instruction Cycle Length: {:?}", instructions.len());

    let step_count: usize = traverse(&graph, "AAA", instructions.as_slice());

    println!("Total steps: {}", step_count-1);

    Ok(())
}
