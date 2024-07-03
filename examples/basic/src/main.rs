use span::prelude::*;

#
pub enum Command {
	Example(String)
}

#[tokio::main]
async fn main() {
	//this has to be changed manually to the socket addrs of peers for this example
	let peers = vec![
		//change these
		"0.0.0.0:0".parse().unwrap(),
		"0.0.0.0:0".parse().unwrap()
	];

    let mut cluster = Cluster::connect("0.0.0.0:0".parse().unwrap(), peers);

    let set = Dataset::create(&mut cluster, ).await.expect("failed to create or find dataset");

	loop {
		if matches!(set.status, Status::Leader) {
			set.send("hello, span!");
		}

		assert!()
	}
}
