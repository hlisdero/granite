use clap::{Parser, ValueEnum};
use granite2::PetriNet;
use log::info;

const ERR_SOURCE_FILE_NOT_FOUND: i32 = 1;
const ERR_OUTPUT_FOLDER_NOT_FOUND: i32 = 2;
const ERR_TRANSLATION: i32 = 3;
const ERR_OUTPUT_FILE_GENERATION: i32 = 4;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum OutputFormat {
    /// Petri Net Markup Language - <https://www.pnml.org/>
    Pnml,
    /// LoLA - A Low Level Petri Net Analyzer - A model checker by the Universität Rostock
    Lola,
    /// DOT (graph description language)
    Dot,
}

impl std::fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Dot => write!(f, "dot"),
            Self::Lola => write!(f, "lola"),
            Self::Pnml => write!(f, "pnml"),
        }
    }
}

/// Convert a Rust source code file into a Petri net and export
/// the resulting net in one of the supported formats.
#[derive(Parser)]
#[command(author, version, long_about = None)]
#[command(about = "Convert a Rust source code file into a Petri net \
    and export the resulting net in one of the supported formats.")]
struct CliArgs {
    /// The path to the Rust source code file to read.
    path: std::path::PathBuf,

    /// Filename for the resulting net.
    /// The output files contain this filename followed by an extension depending on the format.
    #[arg(long, default_value = "net")]
    filename: String,

    /// The path to a valid folder where the output files should be created.
    /// If not specified, the current working directory is used.
    #[arg(long, default_value = ".")]
    output_folder: std::path::PathBuf,

    /// The format for the output. Multiple formats can be specified.
    #[arg(long, value_enum)]
    format: Vec<OutputFormat>,

    /// Verbosity flag.
    #[clap(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
}

fn main() {
    let args = CliArgs::parse();
    // Initialize an `env_logger` with the clap verbosity flag entered by the user.
    env_logger::Builder::new()
        .filter_level(args.verbose.log_level_filter())
        .init();

    // Double check that the file exists before starting the compiler
    // to generate an error message independent of the rustc output.
    info!(
        "Checking that the source code file at {} exists...",
        args.path.to_string_lossy()
    );
    if !args.path.exists() {
        eprintln!(
            "Source code file at {} does not exist",
            &args.path.to_string_lossy()
        );
        std::process::exit(ERR_SOURCE_FILE_NOT_FOUND);
    };

    // Double check that the output folder exists before starting the compiler
    // to generate an error message as soon as possible.
    info!(
        "Checking that the output folder at {} exists...",
        args.output_folder.to_string_lossy()
    );
    if !args.output_folder.exists() {
        eprintln!(
            "Output folder at {} does not exist",
            &args.output_folder.to_string_lossy()
        );
        std::process::exit(ERR_OUTPUT_FOLDER_NOT_FOUND);
    };

    info!("Starting the translation...");
    let petri_net = match granite2::run(args.path) {
        Ok(petri_net) => petri_net,
        Err(err_str) => {
            eprintln!("{err_str}");
            std::process::exit(ERR_TRANSLATION);
        }
    };

    if let Err(err_str) = create_output_files(
        &petri_net,
        &args.filename,
        &args.output_folder,
        &args.format,
    ) {
        eprintln!("{err_str}");
        std::process::exit(ERR_OUTPUT_FILE_GENERATION);
    }
}

fn create_output_files(
    petri_net: &PetriNet,
    filename: &str,
    output_folder: &std::path::Path,
    format: &Vec<OutputFormat>,
) -> Result<(), std::io::Error> {
    for format in format {
        let mut filepath = output_folder.to_path_buf();
        filepath.push(filename);
        filepath.set_extension(format.to_string());

        info!("Creating output file {}...", filepath.to_string_lossy());
        let mut file = std::fs::File::create(filepath)?;
        match format {
            OutputFormat::Dot => petri_net.to_dot(&mut file)?,
            OutputFormat::Lola => petri_net.to_lola(&mut file)?,
            OutputFormat::Pnml => petri_net.to_pnml(&mut file)?,
        }
    }
    Ok(())
}
