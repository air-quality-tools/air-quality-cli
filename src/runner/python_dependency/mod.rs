use std::fs::File;
use std::io::Write;
use tempfile::NamedTempFile;

pub fn create_python_file() -> std::io::Result<NamedTempFile> {
    let python_dependency = include_str!("read_waveplus.py");
    let mut python_temp = tempfile::Builder::new()
        .prefix("read_waveplus.")
        .suffix(".py")
        .tempfile()
        .unwrap();
    // let mut file = File::create(PYTHON_FILENAME)?;
    python_temp.write_all(python_dependency.as_ref())?;
    return Ok(python_temp);
}
