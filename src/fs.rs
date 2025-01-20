use std::{
    fs::{File, Permissions},
    io::{self, Write},
    os::unix::fs::PermissionsExt,
};

pub fn write_binary(file_name: &str, bytes: &[u8]) -> io::Result<()> {
    let mut output = File::create(file_name)?;
    let permissions = Permissions::from_mode(0o755);

    output.write_all(bytes)?;
    output.set_permissions(permissions)?;

    Ok(())
}
