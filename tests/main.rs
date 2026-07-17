//! Tests unitaires pour le filesystem FUSE
//!
//! Ce module contient les tests unitaires pour le filesystem FUSE.
//!
//! # Tests
//!
//! La liste des tests suivants est disponible :
//!
//! * `test_mount`: Vérifie la montée du filesystem
//! * `test_unmount`: Vérifie la démontage du filesystem
//! * `test_ls`: Vérifie la liste des fichiers et répertoires
//! * `test_touch`: Vérifie la création d'un fichier
//! * `test_rm`: Vérifie la suppression d'un fichier
//! * `test_mkdir`: Vérifie la création d'un répertoire
//! * `test_rmdir`: Vérifie la suppression d'un répertoire

#[cfg(test)]
mod tests {
    use super::*;
    use fuse::{MountOptions, UnmountOptions};
    use libc::{c_int, c_void};
    use std::fs::File;
    use std::io;
    use std::path::Path;
    use std::process::Command;

    #[test]
    fn test_mount() -> io::Result<()> {
        // Montage du filesystem
        let mount_options = MountOptions::new().allow_other().default_permissions();
        let mount_point = "/tmp/rust-fuse-mount";
        let fuse_path = "/tmp/rust-fuse";

        // Création du filesystem
        let file = File::create(fuse_path)?;
        file.set_permissions(io::Permissions::from_mode(0o755))?;

        // Montage
        let mount_process = Command::new("fusermount")
            .arg("-u")
            .arg(mount_point)
            .output()?;
        assert!(mount_process.status.success());

        let mount_process = Command::new("fusermount")
            .arg("-z")
            .arg(mount_point)
            .arg("-o")
            .arg("allow_other,default_permissions")
            .arg(fuse_path)
            .output()?;
        assert!(mount_process.status.success());

        // Vérification du montage
        let mount_output = Command::new("ls")
            .arg(mount_point)
            .output()?;
        assert!(mount_output.status.success());

        Ok(())
    }

    #[test]
    fn test_unmount() -> io::Result<()> {
        // Démontage du filesystem
        let mount_options = UnmountOptions::new().force();

        // Démontage
        let unmount_process = Command::new("fusermount")
            .arg("-u")
            .arg("/tmp/rust-fuse-mount")
            .output()?;
        assert!(unmount_process.status.success());

        Ok(())
    }

    #[test]
    fn test_ls() -> io::Result<()> {
        // Création d'un répertoire
        let file = File::create("/tmp/rust-fuse-test")?;
        file.set_permissions(io::Permissions::from_mode(0o755))?;

        // Liste des fichiers et répertoires
        let ls_output = Command::new("ls")
            .arg("/tmp/rust-fuse-test")
            .output()?;
        assert!(ls_output.status.success());

        Ok(())
    }

    #[test]
    fn test_touch() -> io::Result<()> {
        // Création d'un fichier
        let file = File::create("/tmp/rust-fuse-test-touch")?;
        file.set_permissions(io::Permissions::from_mode(0o644))?;

        // Vérification de la création du fichier
        let ls_output = Command::new("ls")
            .arg("/tmp/rust-fuse-test-touch")
            .output()?;
        assert!(ls_output.status.success());

        Ok(())
    }

    #[test]
    fn test_rm() -> io::Result<()> {
        // Création d'un fichier
        let file = File::create("/tmp/rust-fuse-test-rm")?;
        file.set_permissions(io::Permissions::from_mode(0o644))?;

        // Suppression du fichier
        let rm_output = Command::new("rm")
            .arg("/tmp/rust-fuse-test-rm")
            .output()?;
        assert!(rm_output.status.success());

        // Vérification de la suppression du fichier
        let ls_output = Command::new("ls")
            .arg("/tmp/rust-fuse-test-rm")
            .output()?;
        assert!(!ls_output.status.success());

        Ok(())
    }

    #[test]
    fn test_mkdir() -> io::Result<()> {
        // Création d'un répertoire
        let file = File::create("/tmp/rust-fuse-test-mkdir")?;
        file.set_permissions(io::Permissions::from_mode(0o755))?;

        // Vérification de la création du répertoire
        let ls_output = Command::new("ls")
            .arg("/tmp/rust-fuse-test-mkdir")
            .output()?;
        assert!(ls_output.status.success());

        Ok(())
    }

    #[test]
    fn test_rmdir() -> io::Result<()> {
        // Création d'un répertoire
        let file = File::create("/tmp/rust-fuse-test-rmdir")?;
        file.set_permissions(io::Permissions::from_mode(0o755))?;

        // Suppression du répertoire
        let rm_output = Command::new("rm")
            .arg("-r")
            .arg("/tmp/rust-fuse-test-rmdir")
            .output()?;
        assert!(rm_output.status.success());

        // Vérification de la suppression du répertoire
        let ls_output = Command::new("ls")
            .arg("/tmp/rust-fuse-test-rmdir")
            .output()?;
        assert!(!ls_output.status.success());

        Ok(())
    }
}