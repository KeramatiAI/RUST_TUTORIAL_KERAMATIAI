fn main(){
    let merkle_leaves: [u8; 4] = [1, 2, 3, 4]; // 4 elements of type u8 — fixed length!
    println!("Merkle Leaves:{:?}",merkle_leaves[0]);
    println!("Merkle Leaves:{:?}",merkle_leaves[1]);
    println!("Merkle Leaves:{:?}",merkle_leaves[2]);
    println!("Merkle Leaves:{:?}",merkle_leaves[3]);
    // println!("Merkle Leaves:{:?}",merkle_leaves[4]);//index out of bounds: the length is 4 but the index is 4
    println!("Merkle Leaves:{:?}",merkle_leaves);
}