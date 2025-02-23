#![allow(clippy::module_inception)]
#![allow(clippy::unit_arg)]

#[cfg(all(not(target_os = "macos"), not(target_os = "windows")))]
#[global_allocator]
static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

yazi_macro::mod_pub!(app completion confirm help input lives manager notify pick tasks which);

yazi_macro::mod_flat!(context executor logs panic root router signals term);

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	Panic::install();
	Logs::start()?;

	_ = fdlimit::raise_fd_limit();

	yazi_shared::init();

	yazi_config::init()?;

	yazi_adapter::init();

	yazi_boot::init();

	yazi_proxy::init();

	yazi_dds::init();

	yazi_plugin::init()?;

	yazi_core::init();

	yazi_dds::serve();
	app::App::serve().await
}
