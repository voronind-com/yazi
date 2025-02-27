use std::ffi::{OsStr, OsString};

use yazi_plugin::CLIPBOARD;
use yazi_shared::event::Cmd;

use crate::tab::Tab;

struct Opt {
	type_: String,
}

impl From<Cmd> for Opt {
	fn from(mut c: Cmd) -> Self { Self { type_: c.take_first_str().unwrap_or_default() } }
}

impl Tab {
	#[yazi_codegen::command]
	pub fn copy(&mut self, opt: Opt) {
		if !self.try_escape_visual() {
			return;
		}

		let mut s = OsString::new();
		let mut it = self.selected_or_hovered(true).peekable();
		while let Some(u) = it.next() {
			s.push(match opt.type_.as_str() {
				"path" => u.as_os_str(),
				"dirname" => u.parent().map_or(OsStr::new(""), |p| p.as_os_str()),
				"filename" => u.name(),
				"name_without_ext" => u.file_stem().unwrap_or(OsStr::new("")),
				_ => return,
			});
			if it.peek().is_some() {
				s.push("\n");
			}
		}

		// Copy the CWD path regardless even if the directory is empty
		if s.is_empty() && opt.type_ == "dirname" {
			s.push(self.cwd());
		}

		futures::executor::block_on(CLIPBOARD.set(s));
	}
}
