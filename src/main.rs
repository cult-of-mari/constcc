use self::elf::Elf;

mod elf;
mod elf_header;
mod fs;
mod instruction;
mod io;
mod program_header;
mod section_header;

const ELF: Elf = Elf::new();

fn main() -> std::io::Result<()> {
    println!("{ELF:#?}");

    fs::write_binary("main.elf", ELF.as_bytes())?;

    Ok(())
}
