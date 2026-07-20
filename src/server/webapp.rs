use std::{ffi::OsString, io, time::Duration};

use anyhow::{Context, Result, bail};
use tokio::{
	process::{Child, Command},
	time::timeout,
};

pub(crate) struct WebappProcess {
	child: Child,
	process_group: i32,
}

impl WebappProcess {
	pub(crate) fn spawn(command: &[OsString]) -> Result<Self> {
		let Some((program, args)) = command.split_first() else {
			bail!("webapp command is empty");
		};

		let child = Command::new(program)
			.args(args)
			.process_group(0)
			.kill_on_drop(true)
			.spawn()
			.context("failed to spawn webapp command")?;
		let process_group = child.id().expect("spawned webapp has PID") as i32;

		Ok(Self {
			child,
			process_group,
		})
	}
	pub(crate) async fn wait(&mut self) -> io::Result<std::process::ExitStatus> {
		self.child.wait().await
	}

	pub(crate) async fn terminate(&mut self) {
		let already_exited = self.child.try_wait().ok().flatten().is_some();

		unsafe {
			libc::kill(-self.process_group, libc::SIGTERM);
		}

		if already_exited {
			return;
		}

		if timeout(Duration::from_secs(5), self.child.wait())
			.await
			.is_err()
		{
			unsafe {
				libc::kill(-self.process_group, libc::SIGKILL);
			}
			let _ = self.child.wait().await;
		}
	}
}
