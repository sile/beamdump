use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[clap(version)]
enum Args {
    AtomTable { beam_file: PathBuf },
    LiteralTable { beam_file: PathBuf },
    ImportTable { beam_file: PathBuf },
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    match args {
        Args::AtomTable { beam_file } => {
            dump_atom_table(beam_file)?;
        }
        Args::LiteralTable { beam_file } => {
            dump_literal_table(beam_file)?;
        }
        Args::ImportTable { beam_file } => {
            dump_import_table(beam_file)?;
        }
    }
    Ok(())
}

fn dump_atom_table(path: PathBuf) -> anyhow::Result<()> {
    let beam = beam_file::StandardBeamFile::from_file(path)?;
    for (i, atom) in get_atom_chunk(&beam)?.atoms.iter().enumerate() {
        println!("[{}] {}", i, atom.name);
    }
    Ok(())
}

fn dump_literal_table(path: PathBuf) -> anyhow::Result<()> {
    let beam = beam_file::StandardBeamFile::from_file(path)?;
    for chunk in beam.chunks {
        if let beam_file::chunk::StandardChunk::LitT(chunk) = chunk {
            for (i, literal) in chunk.literals.into_iter().enumerate() {
                let term = eetf::Term::decode(literal.as_slice())?;
                println!("[{}] {}", i, term);
            }
            return Ok(());
        }
    }
    anyhow::bail!("No such chunk: 'LitT'");
}

fn dump_import_table(path: PathBuf) -> anyhow::Result<()> {
    let beam = beam_file::StandardBeamFile::from_file(path)?;
    let atom_chunk = get_atom_chunk(&beam)?;
    for chunk in &beam.chunks {
        if let beam_file::chunk::StandardChunk::ImpT(chunk) = chunk {
            for (i, import) in chunk.imports.iter().enumerate() {
                let module = get_atom(&atom_chunk, import.module as usize)?;
                let function = get_atom(&atom_chunk, import.function as usize)?;
                println!("[{}] {}:{}/{}", i, module, function, import.arity);
            }
            return Ok(());
        }
    }
    anyhow::bail!("No such chunk: 'ImpT'");
}

fn get_atom(chunk: &beam_file::chunk::AtomChunk, num: usize) -> anyhow::Result<&str> {
    let index = num
        .checked_sub(1)
        .ok_or_else(|| anyhow::anyhow!("Invalid atom number: {num}"))?;
    chunk
        .atoms
        .get(index)
        .map(|x| x.name.as_str())
        .ok_or_else(|| {
            anyhow::anyhow!(
                "Out of the atom table range: table_size={}, index={}",
                chunk.atoms.len(),
                index
            )
        })
}

fn get_atom_chunk(
    beam: &beam_file::StandardBeamFile,
) -> anyhow::Result<&beam_file::chunk::AtomChunk> {
    for chunk in &beam.chunks {
        if let beam_file::chunk::StandardChunk::Atom(chunk) = chunk {
            return Ok(chunk);
        }
    }
    anyhow::bail!("Neither 'Atom' nor 'AtU8' chunk is found");
}
