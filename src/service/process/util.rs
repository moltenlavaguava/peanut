use std::ffi::OsString;
use std::process::Stdio;

use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;
use tokio::sync::mpsc;

use super::enums::ChildMessage;

pub async fn stream_process(
    cmd: OsString,
    args: Vec<OsString>,
    output_stream: mpsc::Sender<ChildMessage>,
) {
    let mut child = Command::new(&cmd)
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .kill_on_drop(true)
        .spawn()
        .expect("Failed to spawn child process");

    let stdout = child.stdout.take().unwrap();
    let stderr = child.stderr.take().unwrap();

    let output_std = output_stream.clone();
    let output_err = output_stream.clone();

    let std_handle = tokio::spawn(async move {
        let mut reader = BufReader::new(stdout).lines();
        while let Ok(Some(line)) = reader.next_line().await {
            if output_std.send(ChildMessage::StdOut(line)).await.is_err() {
                return;
            }
        }
    });
    let err_handle = tokio::spawn(async move {
        let mut reader = BufReader::new(stderr).lines();
        while let Ok(Some(line)) = reader.next_line().await {
            if output_err.send(ChildMessage::StdErr(line)).await.is_err() {
                return;
            }
        }
    });

    let _ = std_handle.await;
    let _ = err_handle.await;
    let exit_code = child.wait().await.unwrap();
    let _ = output_stream.send(ChildMessage::Exit(exit_code)).await;
}
