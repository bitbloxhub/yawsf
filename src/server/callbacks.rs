use reqwest::Client;
use url::Url;

use crate::Cli;

#[derive(Clone)]
pub(crate) struct ShellClient {
	client: Client,
	base_url: Url,
	host_api: Url,
	token: String,
}

impl ShellClient {
	pub(crate) fn new(client: Client, args: &Cli, host_api: Url) -> Self {
		Self {
			client,
			base_url: args.base_url.clone(),
			host_api,
			token: args.token.clone(),
		}
	}

	pub(crate) async fn post_start(&self) -> anyhow::Result<()> {
		let start_url = self.base_url.join("_start")?;

		self.client
			.post(start_url)
			.json(&serde_json::json!({
				"protocol": "yawsf-v1",
				"host_api": self.host_api.as_str(),
				"token": self.token,
			}))
			.send()
			.await?
			.error_for_status()?;

		Ok(())
	}

	pub(crate) async fn post_quit(&self) -> anyhow::Result<()> {
		let quit_url = self.base_url.join("_quit")?;

		self.client
			.post(quit_url)
			.send()
			.await?
			.error_for_status()?;

		Ok(())
	}
}
