use crate::file_sync::types::error::SynchronizeRunnerErrorResult;
use crate::file_sync::types::metadata::FileMetadata;
use ssh2::{Channel, Session};
use std::fs::{read_dir, OpenOptions};
use std::io::{Read, Write};
use std::path::PathBuf;
use std::str::from_utf8;

pub fn fetch_metadata_remote(
    session: &Session,
    remote_dir_path: &str,
) -> SynchronizeRunnerErrorResult<Vec<FileMetadata>> {
    let filenames = {
        let channel = session.channel_session()?;
        fetch_remote_filenames(channel, remote_dir_path)?
    };

    let file_metadata = filenames
        .iter()
        .map(|filename| {
            let path = format!("{}/{}", remote_dir_path, filename);
            fetch_remote_file_metadata(&session, &PathBuf::from(path), filename.to_owned())
        })
        .flatten()
        .collect();

    Ok(file_metadata)
}

pub fn fetch_remote_filenames(
    mut channel: Channel,
    remote_dir_path: &str,
) -> SynchronizeRunnerErrorResult<Vec<String>> {
    let cmd = format!("cd {} && ls", remote_dir_path);
    channel.exec(&cmd)?;
    let files_str = {
        let mut files_str = String::new();
        channel.read_to_string(&mut files_str)?;
        files_str
    };
    channel.wait_close()?;
    Ok(files_str.lines().map(|s| s.to_owned()).collect())
}

pub fn download_remote_file(
    session: &Session,
    path: &PathBuf,
) -> SynchronizeRunnerErrorResult<Vec<u8>> {
    let (mut remote_file, _stat) = session.scp_recv(path)?;
    let mut content_buffer = Vec::new();
    remote_file.read_to_end(&mut content_buffer)?;
    Ok(content_buffer)
}

pub fn fetch_remote_file_metadata(
    session: &Session,
    path: &PathBuf,
    file_name: String,
) -> SynchronizeRunnerErrorResult<FileMetadata> {
    let (mut remote_file, _stat) = session.scp_recv(path)?;
    let mut content_buffer = Vec::new();
    let content = {
        remote_file.read_to_end(&mut content_buffer)?;
        let content = from_utf8(&content_buffer)?;
        content
    };
    FileMetadata::from_content(content, file_name)
}

pub fn fetch_metadata_local(
    local_dir_path: &PathBuf,
) -> SynchronizeRunnerErrorResult<Vec<FileMetadata>> {
    let files = read_dir(local_dir_path)?;

    let metadata = files
        .map(|file| {
            let dir_entry = file?;

            FileMetadata::from_dir_entry(dir_entry)
        })
        .flatten()
        .collect();

    Ok(metadata)
}

pub fn sync_remote_files_to_local(
    session: &Session,
    remote_dir_path: &str,
    local_dir_path: &PathBuf,
    file_names: &[String],
) -> SynchronizeRunnerErrorResult<()> {
    for file_name in file_names {
        let path = format!("{}/{}", remote_dir_path, file_name);
        let file_content = download_remote_file(session, &PathBuf::from(path))?;

        let local_path = local_dir_path.join(file_name);
        OpenOptions::new()
            .create(true)
            .write(true)
            .open(local_path)?
            .write_all(&file_content)?;
    }

    Ok(())
}
