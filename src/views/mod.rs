pub fn render() -> String {
	let mut string = String::new();
	html! {
		string,
		html {
			head {
				meta charset="utf-8" /
			}
			body {
				h1 {
					"This is a simple website using the " em { "hybrid" } " framework!"
				}
			}
		}
	};
	string
}

pub fn render2(user: &str, link: &str) -> String {
	let mut string = String::new();
	html! {
		string,
		html {
			head {
				meta charset="utf-8" /
			}
			body {
				h1 {
					"Welcome, " ^user "!"
				}
				h2 {
					"Click " a href=^link { "here" } " to go back"
				}
			}
		}
	};
	string
}
