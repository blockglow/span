use span::*;
use serde::*;

#[data("62dc1b7c-6849-43a9-9f47-ecfc8ee1da0b")]
#[derive(Serialize, Deserialize)]
pub enum Command {
	Example(String)
}

#[tokio::main]
async fn main() {
	let example::Args { consensus_port, peers } = example::Args::from_env();

	let bind = ([0, 0, 0, 0], consensus_port).into();

   	let mut cluster = Cluster::connect(bind, peers);

    	let mut set = Dataset::create(&mut cluster, b"these bytes can be used to identify a channel").await.expect("failed to create or find dataset");

	loop {
		if matches!(set.status().state, State::Leader) {
			set.send(&Command::Example("hello, span!".into())).await.unwrap();
		}

		while let Ok(Some(Command::Example(string))) = set.recv().await {
			assert_eq!(string, "hello, span!")
		}
	}
}
