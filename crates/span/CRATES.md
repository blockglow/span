Span is a networking library that currently implements advanced consensus (based on research papers).

It is not complete yet, and not ready for experimental use; however, this is what the consensus API looks like as it stands:

```
use span::*;

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
			set.send(Command::Example("hello, span!".into()));
		}

		while let Some(Command::Example(string)) = set.recv() {
			assert_eq!(string, "hello, span!")
		}
	}
}
```