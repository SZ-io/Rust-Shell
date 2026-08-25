use crate::errors::CrateResult;
use std::fs;
use tokio::process::Command as AsyncCommand;

pub async fn exec_external(program: &str, args: &[String]) -> CrateResult<()> {
    let mut child = AsyncCommand::new(program).args(args).spawn()?;

    child.wait().await?;
    Ok(())
}

pub fn pwd() -> CrateResult<String> {
    let current_dir = std::env::current_dir()?;
    Ok(current_dir.display().to_string())
}

pub fn ls() -> CrateResult<()> {
    let entries = fs::read_dir(".")?;

    for entry in entries {
        let entry = entry?;
        println!("{}", entry.file_name().to_string_lossy());
    }
    Ok(())
}

pub fn cd(path: &str) -> CrateResult<()> {
    std::env::set_current_dir(path)?;
    Ok(())
}

pub fn touch(path: &str) -> CrateResult<()> {
    fs::File::create(path)?;
    Ok(())
}

pub fn rm(path: &str) -> CrateResult<()> {
    fs::remove_file(path)?;
    Ok(())
}

pub fn cat(path: &str) -> CrateResult<String> {
    let pwd = pwd()?;
    let joined_path = std::path::Path::new(&pwd).join(path);
    let content = fs::read_to_string(joined_path)?;
    Ok(content)
}

pub fn mkdir(path: &str) -> CrateResult<()> {
    fs::create_dir(path)?;
    Ok(())
}

pub fn rmdir(path: &str) -> CrateResult<()> {
    fs::remove_dir(path)?;
    Ok(())
}
