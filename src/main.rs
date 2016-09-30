extern crate bitcoin;
extern crate secp256k1;
extern crate rand;

use rand::Rng;
use secp256k1::key::SecretKey;

use secp256k1::Secp256k1;

use bitcoin::network::constants::Network;
use bitcoin::blockdata::script::*;
use bitcoin::blockdata::opcodes::All;
use bitcoin::util::address::Privkey;

fn main() {

    let scp = Secp256k1::new();

    let mut rng = rand::thread_rng();
    
    let secret = SecretKey::new( &scp, &mut rng );

    let private = Privkey::from_key(Network::Testnet, secret, false );

    let address = private.to_address(&scp);

    println!("{:?}", address );

    // let builder = Builder::new();
    // let copy = builder
    //     .push_opcode(All::OP_CHECKSIG);



}
