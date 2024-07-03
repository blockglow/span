use std::{collections::HashMap, env, mem, net::SocketAddr, str::FromStr};


pub struct Args {
	pub consensus_port: u16,
	pub peers: Vec<SocketAddr>,
    }
    
    impl Args {
	fn finish_arg(buffer: &mut Vec<String>, args: &mut HashMap<String, String>) {
	    let mut data = vec![];
	    mem::swap(buffer, &mut data);
	    let name = data.remove(0).trim_start_matches("-").to_string();
	    let value_count = data.len();
	    let value = data
		.into_iter()
		.enumerate()
		.map(|(i, x)| x + if i < value_count - 1 { " " } else { "" })
		.collect();
	    args.insert(name, value);
	}
    
	fn parse_args_recurse(
	    args: &mut HashMap<String, String>,
	    buffer: &mut Vec<String>,
	    rest: &mut impl Iterator<Item = String>,
	) {
	    if !buffer[0].starts_with("--") {
		panic!("argument name is missing double dash")
	    }
	    let Some(sub_arg) = rest.next() else {
		Self::finish_arg(buffer, args);
		return;
	    };
    
	    if sub_arg.starts_with("--") {
		Self::finish_arg(buffer, args);
	    }
    
	    buffer.push(sub_arg);
	    Self::parse_args_recurse(args, buffer, rest);
	}
	fn parse_args() -> HashMap<String, String> {
	    let mut args = HashMap::<String, String>::new();
	    let mut arg_iter = env::args().skip(1);
	    let Some(arg) = arg_iter.next() else {
		return HashMap::default();
	    };
	    let mut buffer = vec![arg];
	    Self::parse_args_recurse(&mut args, &mut buffer, &mut arg_iter);
	    args
	}
	pub fn from_env() -> Self {
	    let args = Self::parse_args();
    
    
	    let consensus_port = args
		.get("consensus_port")
		.expect("missing consensus_port arg")
		.parse()
		.expect("invalid consensus_port arg");
    
	    let peers = args  .get("peers")
		    .expect("missing peers array arg")
		.split(" ")
		.map(|addr| SocketAddr::from_str(addr).expect("invalid peer address"))
		.collect::<Vec<_>>();
    
	    if peers.len() < 2 {
		panic!("expected at least two peers to form a cluster");
	    }
    
	    Args {
		consensus_port,
		peers,
	    }
	}
    }