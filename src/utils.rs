use futures::future::Either;
use futures::io::{self, BufReader};
use futures::prelude::*;
use smol::Timer;
use std::future::Future;
use std::time::Duration;

pub async fn timeout<T>(dur: Duration, f: impl Future<Output = io::Result<T>>) -> io::Result<T> {
    futures::pin_mut!(f);
    match future::select(f, Timer::after(dur)).await {
        Either::Left((out, _)) => out,
        Either::Right(_) => Err(io::ErrorKind::TimedOut.into()),
    }
}
