use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[clap(version)]
enum Args {
    LiteralTable { beam_file: PathBuf },
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    match args {
        Args::LiteralTable { beam_file } => {
            dump_literal_table(beam_file)?;
        }
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
    anyhow::bail!("No 'LitT' chunk in the BEAM file.");
}
