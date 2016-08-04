use maud::PreEscaped;

macro_rules! html_quick {
	($($t:tt)*) => ({
		let mut temporary = String::new();
		let _ = html! {
			temporary,
			$($t)*
		};
		temporary
	});
}

pub fn surrounder(x: String) -> String {
	html_quick! {
		html {
			head {
				meta charset="utf-8" /
			}
			body {
				^PreEscaped(x)
			}
		}
	}
}

pub fn render() -> String {
	html_quick! {
		h1 { "Hybrid web framework" }
	}
}

pub fn render2(user: &str, link: &str) -> String {
	html_quick! {
		h1 {
			"Welcome, " ^user "!"
		}
		h2 {
			"Click " a href=^link { "here" } " to go back"
		}
	}
}
