extern crate bitcoin;
extern crate secp256k1;
extern crate rand;

use rand::Rng;
use secp256k1::key::{PublicKey, SecretKey};

use secp256k1::Secp256k1;

use bitcoin::network::constants::Network;
use bitcoin::blockdata::script::*;
use bitcoin::blockdata::opcodes::All;
use bitcoin::util::address::{Privkey};
use bitcoin::util::address::Address;

#[derive(Debug)]
enum AnchorOffer {
    WILL_CREATE_ANCHOR,
    WONT_CREATE_ANCHOR,
}

#[derive(Debug)]
enum Locktime {
    Seconds(u32),
    Blocks(u32)
}

#[derive(Debug)]
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
    secret: SecretKey,
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
            secret: secret,
            private: private
        }
    }

    fn create_public_key(&self)-> PublicKey{
        let scp = Secp256k1::new();
        PublicKey::from_secret_key(&scp, &self.secret).unwrap()
    }

    fn open_channel(&self, min_depth :u32)-> OpenChannel {
        let mut rng = rand::thread_rng();
        OpenChannel {
            revocation_hash:rng.gen::<[u8; 32]>(),
            next_revocation_hash: rng.gen::<[u8; 32]>(),
            commit_key: self.create_public_key(),
            final_key: self.create_public_key(),
            min_depth: min_depth,
            delay: Locktime::Blocks(0),
            anch: AnchorOffer::WILL_CREATE_ANCHOR,
            initial_fee_rate: 0,
        }
    }

}



fn main() {
    let node = Node::generate();
    println!("{:?}", node.open_channel(0) );

    // let builder = Builder::new();
    // let copy = builder
    //     .push_opcode(All::OP_CHECKSIG);
}
