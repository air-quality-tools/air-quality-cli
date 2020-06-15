use crate::file_sync::error::{SynchronizeRunnerError, SynchronizeRunnerErrorResult};
use ssh2::{Channel, Session};
use std::io::Read;
use std::net::TcpStream;
use std::path::PathBuf;

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

    pub fn sync_remote_to_local(
        &self,
        remote_password: String,
    ) -> SynchronizeRunnerErrorResult<()> {
        let session = self.authenticate(remote_password)?;

        fetch_metadata_remote(session.channel_session().unwrap(), &self.remote_dir_path);

        Ok(())
    }

    fn authenticate(&self, remote_password: String) -> SynchronizeRunnerErrorResult<Session> {
        // Connect to the local SSH server
        let tcp = TcpStream::connect(self.remote_host_and_port()).map_err(|err| {
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

fn fetch_metadata_remote(mut channel: Channel, remote_dir_path: &str) {
    let cmd = format!("cd {} && ls", remote_dir_path);
    channel.exec(&cmd).unwrap();
    let mut s = String::new();
    channel.read_to_string(&mut s).unwrap();
    println!("{}", s);
    channel.wait_close().unwrap();
    println!("{}", channel.exit_status().unwrap());
}

fn fetch_metadata_local(local_dir_path: &PathBuf) {}
