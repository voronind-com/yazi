use std::ops::Deref;

use mlua::{AnyUserData, UserData};

use super::SCOPE;

pub(super) struct Finder {
	inner: *const yazi_core::tab::Finder,
}

impl Deref for Finder {
	type Target = yazi_core::tab::Finder;

	fn deref(&self) -> &Self::Target { unsafe { &*self.inner } }
}

impl Finder {
	#[inline]
	pub(super) fn make(inner: &yazi_core::tab::Finder) -> mlua::Result<AnyUserData> {
		SCOPE.create_userdata(Self { inner })
	}
}

impl UserData for Finder {}
