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
	// Another problem is that the return code also needs to do something...
	// So, I'll just use a small macro, but should ideally use Reply::Redirect or something
	// That's at least unambiguous.
	// Can I create middleware that will surround the html text?
	// Most of the time we respond using a simple Ok, but there are pages that
	// return html with other codes like 404 or 403, these need to be handled.
	// One way of handling these is to write
	// rep![render()]
	// rep![render(), Ok]
	// ... with increasing levels of information I think this is a good idea
	// We still don't have a surrounder though. What can be done?
	// We can also assume that everything is a string, and we check for
	// a '<' in the body of that string
	// That's too strange. Let's just use something better..
	// I like the Reply:: paradigm, but I'm not sure how to report status codes there
	// Maybe these can be assumed, but how are edge cases handled?
	// Maybe just surround manually?
	// rep![sur(render()), Ok]. It's not too bad and allows customization.

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
