use super::*;

pub fn do_mkdirat(fs_path: &FsPath, mode: usize) -> Result<()> {
    debug!("mkdirat: fs_path: {:?}, mode: {:#o}", fs_path, mode);

    let path = fs_path.to_abs_path()?;
    let (dir_path, file_name) = split_path(&path);
    let inode = {
        let current = current!();
        let fs = current.fs().lock().unwrap();
        fs.lookup_inode(dir_path)?
    };
    if inode.find(file_name).is_ok() {
        return_errno!(EEXIST, "");
    }
    if !inode.allow_write()? {
        return_errno!(EPERM, "dir cannot be written");
    }
    inode.create(file_name, FileType::Dir, mode as u32)?;
    Ok(())
}
