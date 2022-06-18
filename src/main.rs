extern crate diesel;
extern crate notes;

use clap::{Parser, Subcommand};
use colored::Colorize;
use notes::{create_note, delete_note, establish_connection, list_notes, edit_note};

#[derive(Parser)]
#[clap(
    author = "kndwin",
    version,
    about = "Notes",
    long_about = "A simple command  note app"
)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    commands: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[clap(
        name = "list", 
        about = "List all notes", 
        visible_aliases = &["ls", "l"]
    )]
    List,
    #[clap(
        name = "create", 
        about = "Create a new note", 
        visible_aliases = &["a", "+", "create", "new"]
    )]
    Add {
        #[clap(name = "body")]
        body: String,
        #[clap(name = "title")]
        title: String,
    },
    #[clap(
        name = "delete", 
        about = "Delete a note",
        visible_aliases = &["d", "-", "delete", "rm"]
    )]
    Delete {
        #[clap(name = "id")]
        id: i32,
    },
    #[clap(
        name = "edit", 
        about = "Edit a note",
        visible_aliases = &["e", "edit"]
    )]
    Edit {
        #[clap(name = "id")]
        id: i32,
        #[clap(name = "title")]
        title: String,
        #[clap(name = "body")]
        body: String,
    },
}

fn main() {
    let cli = Cli::parse();

    let connection = establish_connection();

    match &cli.commands {
        Commands::List => {
            let notes = list_notes(&connection);
            if notes.is_empty() {
                println!("{}", "No notes found".bright_red().bold());
            } else {
                for note in notes {
                    println!(
                        "{}: {:?} - {:?}",
                        note.id.to_string().bright_green().bold(),
                        note.title,
                        note.body
                    );
                }
            }
        }
        Commands::Add { body, title } => {
            println!("Adding note: {:?}", title);
            create_note(&connection, title, body);
        }
        Commands::Delete { id } => {
            println!("Deleting note with id: {:?}", id);
            delete_note(&connection, id);
        }
        Commands::Edit { id, title, body } => {
            println!("Editing note with id: {:?}", id);
            edit_note(&connection, id, title, body);
        }
    }
}
