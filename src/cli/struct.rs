use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]

pub struct Args {
	pub key: String,
	pub value: String
}