mod dashboard_terminal;
mod device;
mod runner;
mod shared;

use dashboard_terminal::start_gui;
use dirs;
use flexi_logger::{opt_format, Age, Cleanup, Criterion, Duplicate, LogTarget, Naming};
use runner::start_data_generator;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(
    name = "air-quality-cli",
    about = "Tools for checking air quality. Supporting Airthings products, but not affiliated with the Airthings company. "
)]
struct Opt {
    #[structopt(long)]
    debug: bool,
    #[structopt(subcommand)]
    command: Command,
}

#[derive(StructOpt)]
enum Command {
    Runner(RunnerOpt),
    Gui(GuiOpt),
}

#[derive(Debug, StructOpt)]
struct RunnerOpt {
    #[structopt(short = "d", long = "data-dir", parse(from_os_str))]
    data_dir_path: Option<PathBuf>,
    #[structopt(short = "s", long = "serial-number")]
    serial_number: u32,
}

#[derive(Debug, StructOpt)]
struct GuiOpt {
    #[structopt(short = "d", long = "data-dir", parse(from_os_str))]
    data_dir_path: Option<PathBuf>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::from_args();

    crate_app_dirs()?;
    set_up_logger(opt.debug, get_log_dir().expect("Log dir"));

    match opt.command {
        Command::Runner(command_opt) => runner_subcommand(command_opt)?,
        Command::Gui(command_opt) => gui_subcommand(command_opt),
    };

    return Ok(());
}

const BASE_DEFAULT_DIR_NAME: &str = ".air-quality";

fn crate_app_dirs() -> Result<(), Box<dyn Error>> {
    dirs::home_dir()
        .map(|path| {
            let base_path = path.join(BASE_DEFAULT_DIR_NAME);
            fs::create_dir_all(&base_path).expect(
                format!(
                    "Could not create default directory: {}",
                    base_path.to_string_lossy()
                )
                .as_str(),
            );

            ["data", "logs"].iter().for_each(|dir_name| {
                let path = base_path.join(dir_name);
                fs::create_dir_all(&path).expect(
                    format!("Could not create directory: {}", path.to_string_lossy()).as_str(),
                );
            })
        })
        .expect("Could not get home dir");

    Ok(())
}

fn get_app_dir() -> Option<PathBuf> {
    dirs::home_dir().map(|path| path.join(BASE_DEFAULT_DIR_NAME))
}

fn get_log_dir() -> Option<PathBuf> {
    get_app_dir().map(|path| path.join("logs"))
}

fn set_up_logger(debug: bool, log_dir: PathBuf) {
    let error_log_level = if debug { "trace" } else { "error" };
    let duplicate_to_std_err_log_level = if debug {
        Duplicate::All
    } else {
        Duplicate::Error
    };

    flexi_logger::Logger::with_str(error_log_level)
        .log_to_file()
        .directory(log_dir)
        .duplicate_to_stderr(duplicate_to_std_err_log_level)
        .rotate(
            Criterion::Age(Age::Day),
            Naming::Timestamps,
            Cleanup::KeepLogFiles(3),
        )
        .format(opt_format)
        .start()
        .unwrap();
}

fn get_data_path(path: Option<PathBuf>) -> PathBuf {
    path.or_else(|| dirs::home_dir().map(|path| path.join(BASE_DEFAULT_DIR_NAME).join("data")))
        .unwrap_or_else(|| Path::new(".").to_path_buf())
}

fn runner_subcommand(opt: RunnerOpt) -> Result<(), Box<dyn Error>> {
    let data_dir_path = get_data_path(opt.data_dir_path);
    start_data_generator(data_dir_path, opt.serial_number)?;

    Ok(())
}

fn gui_subcommand(opt: GuiOpt) {
    let data_dir_path = get_data_path(opt.data_dir_path);

    let gui_result = start_gui(data_dir_path);

    if gui_result.is_err() {
        eprintln!("The terminal GUI failed to start/run");
    }
}
