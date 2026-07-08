use clap::Parser;
use gtk4::Application;
use gtk4::prelude::*;
use yawsf::Cli;
use yawsf::app::activate_host;

fn main() {
	let args = Cli::parse();

	let app = Application::builder()
		.application_id("com.bitbloxhub.yawsf")
		.build();

	app.connect_activate(move |app| {
		activate_host(app, args.clone());
	});

	app.run_with_args::<&str>(&[]);
}
