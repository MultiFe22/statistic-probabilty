struct Node {
    //val: i32,
    left: i32,
    right: i32,
}

fn main() {
    
    use std::collections::HashSet;
    let mut v : Vec<Node> = Vec::new();
    for _n in 0..12 {
        let node = Node {
            //val: n,
            left: 0,
            right: 0,
        };
        v.push(node);
    }
    // define right and left in order to make the cycle

    for n in 0..12 {
        let aux = n as i32;
        v[n].left = (((aux - 1) % 12) + 12) % 12;
        v[n].right = (((aux + 1) % 12) + 12) % 12;
    }
        
    let mut number_of_steps = 0;
    
    for _n in 0..1000001 {        
        let mut current_node = 0;
        let mut visited: HashSet<i32> = HashSet::new();
        visited.insert(current_node);
        while visited.len() != 12 {
            if rand::random() {
                current_node = v[current_node as usize].left;
                visited.insert(current_node);
                number_of_steps += 1;
            } else {
                current_node = v[current_node as usize].right;
                visited.insert(current_node);
                number_of_steps += 1;
            }
        }
    }
    

    println!("{}", (number_of_steps as f64/1000000.0));
    

}