use std::{ffi::OsString, io, time::Duration};

use anyhow::{Context, Result, bail};
use tokio::{
	process::{Child, Command},
	time::timeout,
};

pub(crate) struct WebappProcess {
	child: Child,
}

impl WebappProcess {
	pub(crate) fn spawn(command: &[OsString]) -> Result<Self> {
		let Some((program, args)) = command.split_first() else {
			bail!("webapp command is empty");
		};

		let child = Command::new(program)
			.args(args)
			.kill_on_drop(true)
			.spawn()
			.context("failed to spawn webapp command")?;

		Ok(Self { child })
	}

	pub(crate) async fn wait(&mut self) -> io::Result<std::process::ExitStatus> {
		self.child.wait().await
	}

	pub(crate) async fn terminate(&mut self) {
		let Some(pid) = self.child.id() else {
			return;
		};

		if self.child.try_wait().ok().flatten().is_some() {
			return;
		}

		unsafe {
			libc::kill(pid as i32, libc::SIGTERM);
		}

		if timeout(Duration::from_secs(5), self.child.wait())
			.await
			.is_err()
		{
			let _ = self.child.kill().await;
		}
	}
}
