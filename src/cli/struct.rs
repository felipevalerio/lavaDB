use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]

pub struct Args {
	#[command(subcommand)]
	pub cmd: Commands
}


#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
	GetAll,
	Put {
		key: String,
		value: String
	}
}