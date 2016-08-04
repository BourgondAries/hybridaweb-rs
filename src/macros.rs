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

macro_rules! gen {
	($i:ident ($($tn:ident : $tp:ty),*) $($r:tt)*) => (
		pub fn $i($($tn : $tp),*) -> String {
			html_quick! {
				$($r)*
			}
		}
	);
}

macro_rules! gens {
	($($i:ident ($($tn:ident : $tp:ty),*) { $($r:tt)* })*) => (
		$(
			gen!($i ($($tn : $tp),*) $($r)*);
		)*
	);
}
