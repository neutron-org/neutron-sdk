use log::info;
use std::ffi::OsStr;
use std::path::PathBuf;
use std::process::{Command, Stdio};

fn run_git(args: impl IntoIterator<Item = impl AsRef<OsStr>>) -> Result<(), String> {
    let stdout = Stdio::inherit();

    let exit_status = Command::new("git")
        .args(args)
        .stdout(stdout)
        .status()
        .expect("git exit status missing");

    if !exit_status.success() {
        return Err(format!(
            "git exited with error code: {:?}",
            exit_status.code()
        ));
    }

    Ok(())
}

pub fn clone_repo(repo: &str, dir: &str, rev: &str) {
    let full_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(dir);

    info!("Cloning {} repo...", dir.split('/').nth(2).unwrap());

    run_git(["clone", repo, full_path.to_str().unwrap()]).expect("failed to clone");
    run_git(["-C", full_path.to_str().unwrap(), "checkout", rev]).expect("failed to checkout");
}
