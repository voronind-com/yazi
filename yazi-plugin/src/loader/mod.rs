#![allow(clippy::module_inception)]

use mlua::Lua;

yazi_macro::mod_flat!(loader require);

pub(super) fn init() { LOADER.with(<_>::default); }

pub(super) fn install(lua: &Lua) -> mlua::Result<()> { Require::install(lua) }

pub(super) fn install_isolate(lua: &Lua) -> mlua::Result<()> { Require::install_isolate(lua) }
