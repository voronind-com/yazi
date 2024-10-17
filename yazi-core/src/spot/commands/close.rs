use yazi_macro::render;
use yazi_shared::event::Cmd;

use crate::spot::Spot;

struct Opt {
	submit: bool,
}

impl From<Cmd> for Opt {
	fn from(c: Cmd) -> Self { Self { submit: c.bool("submit") } }
}
impl From<bool> for Opt {
	fn from(submit: bool) -> Self { Self { submit } }
}

impl Spot {
	#[yazi_codegen::command]
	pub fn close(&mut self, opt: Opt) {
		self.visible = false;
		render!();
	}
}
