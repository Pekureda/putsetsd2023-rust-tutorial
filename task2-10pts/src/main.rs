
//Notice how a similarly-looking program in C++ would have no problem running.
// I'm referring to the concept, not the program semantics.
fn modify_vector_values(vector: Vec<i32>) {//modify to take mutable reference
    for  i in 0.. vector.len() {
        vector[i] *= 2; // Multiply each value by 2
    }
}

fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    println!("Original vector: {:?}", numbers);

    modify_vector_values( numbers);//currently we are transfering ownership of the variable to the function
    //better approach is to borrow reference
    //sometimes its needed to borrow mutable reference

    println!("Modified vector: {:?}", numbers);
}