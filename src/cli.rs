use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Action {
	/// Write tasks into the journal file.
	Add {
		/// The task description text.
		#[structopt()]
		text: String },
	/// Remove an entry(finished task) from the journal file by position(unsize int) 
	Done {
		#[structopt()]
		position: usize },
	/// List all tasks in the journal file.
	List,
}

#[derive(Debug, StructOpt)]
#[structopt(
	name = "Rusty Journal",
	about = "A command line to-do app written in Rust"
)]
pub struct CommandLineArgs {
	#[structopt(subcommand)]
	pub action: Action,
	/// Use a different journal file to store the task entries
  #[structopt(parse(from_os_str), short, long)]
	pub journal_file: Option<PathBuf>,
}