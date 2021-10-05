mod block;
mod blockchain;

use blockchain::Blockchain;

fn main() {
    let mut blockchain = Blockchain::new(20);
    for data in ["what", "a", "wonderful", "world"] {
        blockchain.add_data(data.as_bytes().to_vec());
    }
    blockchain.mine();
}
