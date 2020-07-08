use crate::file_sync::file_service::{
    fetch_metadata_local, fetch_metadata_remote, sync_remote_files_to_local,
};
use crate::file_sync::types::error::{SynchronizeRunnerError, SynchronizeRunnerErrorResult};
use crate::file_sync::types::metadata::reduce_remote_metadata_list_to_modified_or_not_exist_local;
use chrono::Local;
use ssh2::Session;
use std::net::TcpStream;
use std::path::PathBuf;
use std::thread::sleep;
use std::time::Duration;

pub struct SynchronizeRunner {
    local_dir_path: PathBuf,
    remote_dir_path: String,
    remote_host: String,
    remote_port: u32,
    remote_username: String,
}

pub struct SynchronizeRunnerBuilder {
    pub local_dir_path: PathBuf,
    pub remote_dir_path: String,
    pub remote_host: String,
    pub remote_port: u32,
    pub remote_username: String,
}

impl From<SynchronizeRunnerBuilder> for SynchronizeRunner {
    fn from(builder: SynchronizeRunnerBuilder) -> Self {
        Self::new(builder)
    }
}

impl SynchronizeRunner {
    pub fn new(builder: SynchronizeRunnerBuilder) -> Self {
        let SynchronizeRunnerBuilder {
            remote_dir_path,
            local_dir_path,
            remote_host,
            remote_port,
            remote_username,
        } = builder;

        Self {
            local_dir_path,
            remote_dir_path,
            remote_host,
            remote_port,
            remote_username,
        }
    }

    fn remote_host_and_port(&self) -> String {
        format!("{}:{}", self.remote_host, self.remote_port)
    }

    pub fn sync_remote_to_local_loop(
        &self,
        remote_password: String,
        sleep_duration: Duration,
    ) -> SynchronizeRunnerErrorResult<()> {
        let session = self.authenticate(remote_password)?;

        loop {
            println!(
                "[{}] Syncing remote files to local, with loop duration: {}s",
                chrono::Utc::now()
                    .with_timezone(&Local)
                    .format("%Y-%m-%d %H:%M"),
                sleep_duration.as_secs()
            );
            self.sync_remote_to_local_once(&session)?;
            sleep(sleep_duration);
        }
    }

    fn sync_remote_to_local_once(&self, session: &Session) -> SynchronizeRunnerErrorResult<()> {
        let local_metadata = fetch_metadata_local(&self.local_dir_path)?;
        let remote_metadata = fetch_metadata_remote(&session, &self.remote_dir_path)?;

        let new_or_changed_list: Vec<String> =
            reduce_remote_metadata_list_to_modified_or_not_exist_local(
                &remote_metadata,
                &local_metadata,
            )
            .iter()
            .map(|metadata| metadata.file_name().to_owned())
            .collect();

        sync_remote_files_to_local(
            &session,
            &self.remote_dir_path,
            &self.local_dir_path,
            &new_or_changed_list,
        )?;

        Ok(())
    }

    pub fn authenticate(&self, remote_password: String) -> SynchronizeRunnerErrorResult<Session> {
        // Connect to the local SSH server
        let tcp = TcpStream::connect(self.remote_host_and_port()).map_err(|_err| {
            SynchronizeRunnerError::new(format!(
                "Could not connect to SSH server {}",
                self.remote_host_and_port()
            ))
        })?;

        let mut session = Session::new()?;
        session.set_tcp_stream(tcp);
        session.handshake()?;

        let maybe_session = session.userauth_password(&self.remote_username, &remote_password);

        drop(remote_password);
        if maybe_session.is_err() {
            return Err(SynchronizeRunnerError::new(format!(
                "Username ({}) and password combination failed",
                self.remote_username
            )));
        }

        if session.authenticated() == false {
            return Err(SynchronizeRunnerError::new(
                "Unable to authenticate with remote SSH server".to_owned(),
            ));
        }

        Ok(session)
    }
}
