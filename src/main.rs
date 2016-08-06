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

fn main() {

	let hybrid = hybrid! {

		(req, elm) |

		get "/", homepage => {
			debug![elm.log, "Got request", "req" => format!("{:?}", req)];
			rep![render()]
		},

		get "/user/:uid", userpage => {
			rep![render2(req.ext::<Router>().find("uid").unwrap_or("nobody"),
				elm.rev.homepage)]
		},

		post "/user/:new", newuser => {
			red![Found, String::from(elm.rev.userpage) + "/ok"]
		},

	};

	Iron::new(hybrid).http("localhost:3000").unwrap();

}
