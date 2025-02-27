use yazi_proxy::HIDER;
use yazi_shared::{event::{Cmd, Data}, fs::Url};

use crate::manager::Manager;

#[derive(Debug, Default)]
struct Opt {
	skip:        Option<usize>,
	force:       bool,
	only_if:     Option<Url>,
	upper_bound: bool,
}

impl From<Cmd> for Opt {
	fn from(mut c: Cmd) -> Self {
		Self {
			skip:        c.first().and_then(Data::as_usize),
			force:       c.bool("force"),
			only_if:     c.take("only-if").and_then(Data::into_url),
			upper_bound: c.bool("upper-bound"),
		}
	}
}
impl From<bool> for Opt {
	fn from(force: bool) -> Self { Self { force, ..Default::default() } }
}

impl Manager {
	#[yazi_codegen::command]
	pub fn peek(&mut self, opt: Opt) {
		let Some(hovered) = self.hovered().cloned() else {
			return self.active_mut().preview.reset();
		};
		if HIDER.try_acquire().is_err() {
			return self.active_mut().preview.reset_image();
		}

		let mime = self.mimetype.get_owned(&hovered.url).unwrap_or_default();
		let folder = self.active().hovered_folder().map(|f| (f.offset, f.cha));

		if !self.active().preview.same_url(&hovered.url) {
			self.active_mut().preview.skip = folder.map(|f| f.0).unwrap_or_default();
		}
		if !self.active().preview.same_file(&hovered, &mime) {
			self.active_mut().preview.reset();
		}

		if matches!(opt.only_if, Some(u) if u != hovered.url) {
			return;
		}

		if let Some(skip) = opt.skip {
			let preview = &mut self.active_mut().preview;
			if opt.upper_bound {
				preview.skip = preview.skip.min(skip);
			} else {
				preview.skip = skip;
			}
		}

		if hovered.is_dir() {
			self.active_mut().preview.go_folder(hovered, folder.map(|f| f.1), opt.force);
		} else {
			self.active_mut().preview.go(hovered, mime.into(), opt.force);
		}
	}
}
