//! Module de liaison FUSE entre le noyau Linux et le filesystem.
//!
//! Ce module utilise la bibliothèque `fuse-rs` pour implémenter la liaison FUSE.
//!

use crate::module::Module;
use fuse::Filesystem;
use log::{error, info};
use std::process::{Command, Stdio};

/// Représentation d'un filesystem FUSE.
pub struct Fuse {
    /// Nom du filesystem.
    name: String,
    /// Chemin du module noyau.
    module_path: String,
}

impl Fuse {
    /// Crée un nouveau filesystem FUSE.
    ///
    /// # Arguments
    ///
    /// * `name` - Nom du filesystem.
    /// * `module_path` - Chemin du module noyau.
    pub fn new(name: String, module_path: String) -> Self {
        Self { name, module_path }
    }

    /// Monte le filesystem FUSE.
    ///
    /// # Arguments
    ///
    /// * `mount_point` - Point de montage du filesystem.
    pub fn mount(&self, mount_point: String) -> Result<(), String> {
        // Crée un nouveau processus pour monter le filesystem.
        let mut command = Command::new("modprobe")
            .arg("-v")
            .arg(&self.module_path)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?;

        // Attend la fin du processus.
        command.wait()?;

        // Crée un nouveau processus pour monter le filesystem.
        let mut command = Command::new("mount")
            .arg("-t")
            .arg("fuse")
            .arg(&self.name)
            .arg(mount_point)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?;

        // Attend la fin du processus.
        command.wait()?;

        Ok(())
    }

    /// Désmonte le filesystem FUSE.
    ///
    /// # Arguments
    ///
    /// * `mount_point` - Point de montage du filesystem.
    pub fn unmount(&self, mount_point: String) -> Result<(), String> {
        // Crée un nouveau processus pour désmonter le filesystem.
        let mut command = Command::new("umount")
            .arg(mount_point)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?;

        // Attend la fin du processus.
        command.wait()?;

        Ok(())
    }
}

impl Filesystem for Fuse {
    /// Nom du filesystem.
    fn name(&self) -> &str {
        &self.name
    }

    /// Chemin du module noyau.
    fn module_path(&self) -> &str {
        &self.module_path
    }
}

// Définition des erreurs possibles.
#[derive(Debug)]
pub enum Error {
    // Erreur lors de la montée du filesystem.
    MountError,
    // Erreur lors de la désmonétée du filesystem.
    UnmountError,
}

// Implémentation des erreurs.
impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::MountError => write!(f, "Erreur lors de la montée du filesystem"),
            Error::UnmountError => write!(f, "Erreur lors de la désmonétée du filesystem"),
        }
    }
}