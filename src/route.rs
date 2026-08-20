use dioxus::prelude::*;

use crate::layouts::{MainLayout, PlayerLayout};
use crate::views::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(MainLayout)]
    #[route("/")]
    Home {},
    #[route("/config")]
    Config {},
    #[end_layout]
    #[layout(PlayerLayout)]
    #[route("/player")]
    Player {}
}
