mod block;
mod chain;

fn main() {
    env_logger::init();
    let mut blockchain = chain::Blockchain::new(20);
    for data in ["what", "a", "wonderful", "world"] {
        blockchain.add_to_pool(data.as_bytes().to_vec());
    }
    blockchain.mine();
}
