use crate::file_sync::types::error::SynchronizeRunnerErrorResult;
use sha1::digest::generic_array::GenericArray;
use sha1::{Digest, Sha1};
use std::fs::{DirEntry, File};
use std::io::{BufReader, Read};
use std::path::PathBuf;

// type DateTimeUtc = DateTime<Utc>;

#[derive(Debug, Clone)]
pub struct FileMetadata {
    checksum: HasherType,
    file_name: String,
    // timestamp_created: DateTimeUtc,
    // timestamp_modified: DateTimeUtc,
}

pub struct FileMetadataBuilder {
    pub checksum: HasherType,
    pub file_name: String,
    // pub timestamp_created: DateTimeUtc,
    // pub timestamp_modified: DateTimeUtc,
}

impl From<FileMetadataBuilder> for FileMetadata {
    fn from(builder: FileMetadataBuilder) -> Self {
        Self::new(builder)
    }
}

impl FileMetadata {
    pub fn new(builder: FileMetadataBuilder) -> Self {
        let FileMetadataBuilder {
            checksum,
            file_name,
            // timestamp_created,
            // timestamp_modified,
        } = builder;

        Self {
            file_name,
            checksum,
            // timestamp_modified,
            // timestamp_created,
        }
    }

    pub fn from_dir_entry(dir_entry: DirEntry) -> SynchronizeRunnerErrorResult<FileMetadata> {
        // let timestamp_created = {
        //     let system_time = dir_entry.metadata()?.created()?;
        //     chrono::DateTime::from(system_time)
        // };
        // let timestamp_modified = {
        //     let system_time = dir_entry.metadata()?.modified()?;
        //     chrono::DateTime::from(system_time)
        // };

        let checksum = generate_checksum_from_file(&dir_entry.path())?;

        Ok(FileMetadataBuilder {
            file_name: dir_entry.file_name().into_string()?,
            checksum,
            // timestamp_created,
            // timestamp_modified,
        }
        .into())
    }

    pub fn from_content(
        content: &str,
        file_name: String,
    ) -> SynchronizeRunnerErrorResult<FileMetadata> {
        let checksum = generate_checksum_from_string(content)?;

        Ok(FileMetadataBuilder {
            file_name,
            checksum,
        }
        .into())
    }
}

impl FileMetadata {
    pub fn equal_checksum(&self, other: &FileMetadata) -> bool {
        self.checksum == other.checksum
    }

    pub fn file_name(&self) -> &str {
        &self.file_name
    }
}

type HasherType = GenericArray<u8, <Sha1 as Digest>::OutputSize>;

fn sha1_digest<R: Read>(
    mut reader: R,
) -> SynchronizeRunnerErrorResult<GenericArray<u8, <Sha1 as Digest>::OutputSize>> {
    let mut hasher = Sha1::new();
    let mut buffer = [0; 1024];

    loop {
        let count = reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        hasher.update(&buffer[..count]);
    }

    Ok(hasher.finalize())
}

fn generate_checksum_from_file(path: &PathBuf) -> SynchronizeRunnerErrorResult<HasherType> {
    let input = File::open(path)?;
    let reader = BufReader::new(input);
    let digest = sha1_digest(reader)?;
    Ok(digest)
}

fn generate_checksum_from_string(input: &str) -> SynchronizeRunnerErrorResult<HasherType> {
    let reader = BufReader::new(input.as_bytes());
    let digest = sha1_digest(reader)?;
    Ok(digest)
}

pub fn reduce_remote_metadata_list_to_modified_or_not_exist_local(
    remote_metadata_list: &[FileMetadata],
    local_metadata_list: &[FileMetadata],
) -> Vec<FileMetadata> {
    let mut changed_or_new: Vec<FileMetadata> = Vec::new();
    remote_metadata_list.iter().for_each(|remote_metadata| {
        let local_metadata = {
            local_metadata_list
                .iter()
                .find(|local_metadata| local_metadata.file_name == remote_metadata.file_name)
        };
        if let Some(local_metadata) = local_metadata {
            if remote_metadata.clone().equal_checksum(local_metadata) == false {
                // Only when the checksums are different
                changed_or_new.push(remote_metadata.to_owned());
            }
        } else {
            changed_or_new.push(remote_metadata.to_owned());
        }
    });

    changed_or_new
}
