extern crate bitcoin;
extern crate secp256k1;
extern crate rand;

use rand::Rng;
use secp256k1::key::SecretKey;

use secp256k1::Secp256k1;

use bitcoin::network::constants::Network;
use bitcoin::blockdata::script::*;
use bitcoin::blockdata::opcodes::All;
use bitcoin::util::address::{Privkey, PublicKey};
use bitcoin::util::address::Address;

enum AnchorOffer {
    WILL_CREATE_ANCHOR,
    WONT_CREATE_ANCHOR,
}

enum Locktime {
    Seconds(u32),
    Blocks(u32)
}

struct OpenChannel {
    revocation_hash: [u8; 32],
    next_revocation_hash: [u8; 32],
    commit_key: PublicKey,
    final_key: PublicKey,
    anch: AnchorOffer,
    min_depth: u32,
    delay: Locktime,
    initial_fee_rate: u32
}

struct Node {
    address : Address,
    private: Privkey,
}

impl Node {
    fn generate()-> Self {
        let scp = Secp256k1::new();
        let mut rng = rand::thread_rng();
        let secret = SecretKey::new( &scp, &mut rng );
        let private = Privkey::from_key(Network::Testnet, secret, false );
        let address = private
            .to_address(&scp)
            .unwrap();

        Node {
            address:address,
            private: private
        }
    }

    fn open_channel(&self, otherNode: Node)-> Result<_> {

    }

}



fn main() {
    let node = Node::generate();
    println!("{:?}", node.address );

    // let builder = Builder::new();
    // let copy = builder
    //     .push_opcode(All::OP_CHECKSIG);
}
