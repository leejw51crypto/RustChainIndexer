mod proto;

use proto::tendermint::blockchain::{BlockRequest, BlockResponse};

fn main() {
    let m=BlockRequest::default();
    let m2= BlockResponse::default();
    println!("request {:?}",m);
    println!("response {:?}",m2);
}
