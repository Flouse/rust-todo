mod task;
mod cli;
use anyhow;
use structopt::StructOpt;
use cli::{Action::*, CommandLineArgs};
use task::Task;
use std::path::PathBuf;

fn find_default_journal_file() -> Option<PathBuf> {
  home::home_dir().map(|mut path| {
    path.push(".rusty-journal.json");
    path
  })
}

fn main() -> anyhow::Result<()> {
  // println!("{:#?}", cli::CommandLineArgs::from_args());
  let CommandLineArgs {
    action, journal_file,
  } = CommandLineArgs::from_args();

  // unpack the journal file
  let journal_file = journal_file
    .or_else(find_default_journal_file)
    .expect("faild to find journal file");

  // perform the action
  match action {
      Add { text } => task::add_task(journal_file, Task::new(text)),
      List => task::list_tasks(journal_file),
      Done { position } => task::complete_task(journal_file, position),
  }?;

  Ok(())
}
