Span is a networking library that currently implements advanced consensus (based on research papers).

It is not complete yet, and not ready for experimental use; however, this is what the consensus API looks like as it stands:

```
use span::*;
use serde::*;

#[data("62dc1b7c-6849-43a9-9f47-ecfc8ee1da0b")]
#[derive(Serialize, Deserialize)]
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

    	let mut set = Dataset::create(&mut cluster, b"these bytes can be used to identify a channel").await.expect("failed to create or find dataset");

	loop {
		if matches!(set.status().state, State::Leader) {
			set.send(&Command::Example("hello, span!".into()));
		}

		while let Ok(Some(Command::Example(string))) = set.recv().await {
			assert_eq!(string, "hello, span!")
		}
	}
}

```