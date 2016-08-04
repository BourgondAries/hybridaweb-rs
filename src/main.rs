#![cfg_attr(feature = "dev", allow(unstable_features))]
#![cfg_attr(feature = "dev", feature(plugin))]
#![cfg_attr(feature = "dev", plugin(clippy))]
#![feature(plugin)]
#![plugin(maud_macros)]

extern crate iron;
extern crate maud;
#[macro_use(hybrid)]
extern crate hybridweb;
#[macro_use(router)]
extern crate router;
#[macro_use]
extern crate slog;

mod views;

use hybridweb::prelude::*;
use views::*;

fn main() {

	let (hybrid, surround) = hybrid! {

		get "/", homepage: (req, elm) => {
			debug![elm.log, "Got request", "req" => format!("{:?}", req)];
			Reply::Html(render());
			Reply::Html(quick())
		},

		get "/user/:uid", userpage: (req, elm) => {
			Reply::Html(
				render2(req.ext::<Router>().find("uid").unwrap_or("nobody"),
					elm.rev.homepage)
			)
		},

	};

	surround.surround_with(surrounder);
	Iron::new(hybrid).http("localhost:3000").unwrap();

}
