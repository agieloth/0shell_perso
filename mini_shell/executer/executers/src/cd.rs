/*
It allows users to move from the current working directory to another specified directory by providing either an absolute path or a relative path.
This command is essential for exploring different locations within the Linux environment and managing files efficiently.
*/
use types::command::Command;
use std::env;

pub fn cd(command: &Command) {
    // Déterminer le chemin de destination
    let path = if command.args.is_empty() {
        // Si aucun argument, aller au répertoire home
        match env::var("HOME").or_else(|_| env::var("USERPROFILE")) {
            Ok(home) => home,
            Err(_) => {
                eprintln!("cd: HOME directory not found");
                return;
            }
        }
    } else {
        // Utiliser le premier argument comme destination
        command.args[0].clone()
    };

    // Changer de répertoire
    match env::set_current_dir(&path) {
        Ok(_) => {
            // Succès - pas de message (comportement shell standard)
        }
        Err(e) => {
            eprintln!("cd: {}: {}", path, e);
        }
    }
}