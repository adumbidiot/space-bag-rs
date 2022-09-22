use anyhow::ensure;
use anyhow::Context;
use num_bigint::BigUint;
use std::fs::File;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;

#[derive(Debug, argh::FromArgs)]
#[argh(description = "a tool to take the \"space\" out of files")]
struct Options {
    #[argh(subcommand)]
    subcommand: Subcommand,
}

#[derive(Debug, argh::FromArgs)]
#[argh(subcommand)]
enum Subcommand {
    Pack(PackOptions),
    Unpack(UnpackOptions),
}

#[derive(Debug, argh::FromArgs)]
#[argh(
    subcommand,
    name = "pack",
    description = "remove the space from a file"
)]
struct PackOptions {
    #[argh(positional, description = "the file input")]
    input: PathBuf,

    #[argh(positional, description = "the file output")]
    output: PathBuf,
}

#[derive(Debug, argh::FromArgs)]
#[argh(
    subcommand,
    name = "unpack",
    description = "remove the space from a file"
)]
struct UnpackOptions {
    #[argh(positional, description = "the file input")]
    input: PathBuf,

    #[argh(positional, description = "the file output")]
    output: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let options: Options = argh::from_env();

    match options.subcommand {
        Subcommand::Pack(options) => {
            let mut input_file = File::open(&options.input)
                .map(BufReader::new)
                .with_context(|| format!("failed to open input `{}`", options.input.display()))?;
            let mut output_file = File::options()
                .create_new(true)
                .write(true)
                .open(&options.output)
                .map(BufWriter::new)
                .with_context(|| {
                    format!("failed to create output `{}`", options.output.display())
                })?;

            let mut buffer = Vec::new();
            input_file.read_to_end(&mut buffer)?;

            let mut num = BigUint::from_bytes_le(&buffer);
            println!("Num: {num}");
            
            let zero = BigUint::new(Vec::new());
            let mut partial = 0;
            while num > zero {
                partial += 1;
                if partial == 8 {
                    output_file
                        .write_all(&[u8::MAX])
                        .context("failed to write byte")?;
                    partial = 0;
                }

                debug_assert!(partial < 8);

                num -= 1_u8;
            }
            
            output_file.write_all(&[partial]).context("failed to write byte")?;

            output_file.flush().context("failed to flush")?;
            output_file.get_mut().sync_all().context("failed to sync")?;
        }
        Subcommand::Unpack(options) => {
            let input_file = File::open(&options.input)
                .map(BufReader::new)
                .with_context(|| format!("failed to open input `{}`", options.input.display()))?;
            let mut output_file = File::options()
                .create_new(true)
                .write(true)
                .open(&options.output)
                .map(BufWriter::new)
                .with_context(|| {
                    format!("failed to create output `{}`", options.output.display())
                })?;

            let mut num = BigUint::new(Vec::new());
            let mut has_space = false;
            for b in input_file.bytes() {
                ensure!(!has_space, "this file has some space in it");
                
                let b = b.context("failed to read byte")?;
                has_space = b.count_zeros() != 0;
                num += b.count_ones();
            }

            println!("Num: {num}");

            let buffer = num.to_bytes_le();
            output_file.write_all(&buffer).context("failed to write bytes")?;

            output_file.flush().context("failed to flush")?;
            output_file.get_mut().sync_all().context("failed to sync")?;
        }
    }

    Ok(())
}
