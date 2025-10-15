use clap::{Parser, Subcommand};
use manage_system::handles;

#[derive(Parser)]
#[command{
    version = "1.0",
    about = "Manage system",
    long_about = "Manage system for student information"
}]
struct Cli{
    #[command(subcommand)]
    commands: Option<Commands>
}

#[derive(Subcommand)]
enum Commands{
    /// List all students
    List,
    /// Add a student
    Add {
        ///The id of the student
        #[arg(short, long)]
        id: String,

        ///The name of the student
        #[arg(short, long)]
        name: String,

        ///The score of the student
        #[arg(short, long)]
        score: i32,
    },
    ///Delete a student
    Delete {
        ///The id of the student
        #[arg(short, long)]
        id: String,
    },
    ///Edit a student
    Edit {
        ///The id of the student
        #[arg(short, long)]
        id: String,
    },

}
fn main() {
    let cli = Cli::parse();
    match &cli.commands {
        Some(Commands::List) => handles::handle_list().unwrap(),
        Some(Commands::Add {
                 id,
                 name,
                 score,
             }) => handles::handle_add(id, name, *score).unwrap(),
        Some(Commands::Delete{
                id,
             }) => handles::handle_delete(id).unwrap(),
         Some(Commands::Edit{
                  id,
              }) => handles::handle_edit(id).unwrap(),
        _ =>println!("No command provided or command not recognized."),
    }
}