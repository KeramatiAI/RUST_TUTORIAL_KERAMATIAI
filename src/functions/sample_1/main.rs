fn main() {
    let reward = calculate_reward(100);
    println!("Block reward: {}", reward);
}

// Parameter names: snake_case | Types are required
fn calculate_reward(block_height: u64) -> u64 {
    // Note: the last expression (without ;) is the return value
    if block_height < 210_000 {
        50
    } else {
        25
    }
}
