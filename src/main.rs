#![cfg_attr(feature = "dev", allow(unstable_features))]
#![cfg_attr(feature = "dev", feature(plugin))]
#![cfg_attr(feature = "dev", plugin(clippy))]
#![feature(plugin)]
#![plugin(maud_macros)]

#[macro_use]
extern crate hado;
extern crate iron;
extern crate maud;
#[macro_use(hybrid)]
extern crate hybridweb;
#[macro_use(router)]
extern crate router;
#[macro_use]
extern crate slog;

#[macro_use]
mod macros;
mod views;

use hybridweb::prelude::*;
use views::*;

type Html = String;
fn main() {

	// Ideally, we want to use String-returning renderers (or super-light HTML objects)
	// This allows us to compose renders. The surrounding renderer will always run for
	// the top-level renderer
	// We can return a tuple: (render, modifiers)
	// The render will pass through a surrounder, but the surrounder should also have
	// access to the login state and various other resources.

	let (hybrid, surround) = hybrid! {

		get "/", homepage: (req, elm) => {
			debug![elm.log, "Got request", "req" => format!("{:?}", req)];
			render();
		},

		get "/user/:uid", userpage: (req, elm) => {
			render2(req.ext::<Router>().find("uid").unwrap_or("nobody"),
				elm.rev.homepage)
		},

		post "/user/:new", newuser: (_, elm) => {
			red![Found, String::from(elm.rev.userpage) + "/ok"]
		},

	};

	surround.surround_with(surrounder);
	Iron::new(hybrid).http("localhost:3000").unwrap();

}
