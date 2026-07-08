use std::net::SocketAddr;

use clap::Parser;
use rand::distr::{Alphanumeric, SampleString};
use url::Url;

#[derive(Parser, Clone)]
pub struct Cli {
	/// Webapp backend, e.g. http://127.0.0.1:12551/
	#[arg(long, default_value = "http://127.0.0.1:12551/", value_parser = parse_base_url)]
	pub(crate) base_url: Url,

	/// Native host bind address
	#[arg(long, default_value = "127.0.0.1:12550")]
	pub(crate) bind: SocketAddr,

	/// API token
	#[arg(long, default_value_t = generate_token(), hide_default_value = true)]
	pub(crate) token: String,

	/// Supervised child command; supports shell-style quoting but does not invoke a shell.
	#[arg(long, value_name = "COMMAND")]
	pub(crate) webapp_command: Option<String>,
}

fn parse_base_url(s: &str) -> Result<Url, String> {
	let mut url = Url::parse(s).map_err(|err| err.to_string())?;

	match url.scheme() {
		"http" | "https" => {}
		scheme => return Err(format!("unsupported URL scheme: {scheme}")),
	}

	if url.cannot_be_a_base() {
		return Err("base URL must be hierarchical".to_string());
	}

	if !url.path().ends_with('/') {
		let path = format!("{}/", url.path());
		url.set_path(&path);
	}

	Ok(url)
}

fn generate_token() -> String {
	Alphanumeric.sample_string(&mut rand::rng(), 32)
}

impl Cli {
	pub(crate) fn parsed_webapp_command(&self) -> anyhow::Result<Option<Vec<std::ffi::OsString>>> {
		self.webapp_command
			.as_deref()
			.map(shell_words::split)
			.transpose()
			.map(|command| command.map(|args| args.into_iter().map(Into::into).collect()))
			.map_err(Into::into)
	}
}
