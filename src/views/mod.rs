use maud::PreEscaped;

pub fn sur(custom_html: String) -> String {
	html_quick_doctype![
		html {
			head {
				meta charset="utf-8" /
				meta name="description" content="Hybrida's Website" /
				link rel="icon" type="image/png" href="/files/images/favicon16x16.png"
				link rel="icon" type="image/png" href="/files/images/favicon24x24.png"
				link rel="icon" type="image/png" href="/files/images/favicon32x32.png"
				link rel="icon" type="image/png" href="/files/images/favicon64x64.png"
				link rel="author" href="https://github.com/BourgondAries" /
				title { "Hybrida" }

				script src="/files/jquery/dist/jquery.min.js" {}

				script src="/files/bootstrap/dist/js/bootstrap.min.js" {}
				link rel="stylesheet" type="text/css" href="/files/bootstrap/dist/css/bootstrap.min.css" /
				link rel="stylesheet" type="text/css" href="/files/bootstrap/dist/css/bootstrap-theme.min.css" /

				link rel="stylesheet" type="text/css" href="/files/styles/fonts.css" /
				link rel="stylesheet" type="text/css" href="/files/styles/main.css" /
				link rel="stylesheet" type="text/css" href="/files/styles/media.css" /
			}
			body {
				nav class="navbar navbar-default navbar-fixed-top" {
					div class="container-fluid nav-container" {
						div class="navbar-header" {
							button type="button" class="navbar-toggle collapsed" data-toggle="collapse" data-target="#navigation_menu" {
								span class="sr-only" { "Toggle navigation" }
								span class="icon-bar" { }
								span class="icon-bar" { }
								span class="icon-bar" { }
							}
							a class="navbar-brand" id="home" href="/" {
								img src="/files/images/griff_drop_shadow.png" id="logo" alt="Griff logo" /
							}
						}
						div class="collapse navbar-collapse" id="navigation_menu" {
							ul class="nav navbar-nav" {
								li { a href="/index" { "Nytt Innlegg" } }
								li { a href="/bedrift" { "Bedrift" } }
								li { a href="/about" { "Om Hybrida" } }
								li { a href="/ringen" { "I&IKT-Ringen" } }
							}
							form class="navbar-form navbar-right" role="search" action="/search" method="GET" {
								div class="form-group" {
									input id="navbar-search" type="text" class="form-control" name="term" placeholder="Søk" /
								}
							}
							ul class="nav navbar-nav navbar-right" {
								li class="dropdown" {
									a class="dropdown-toggle" role="button" data-toggle="dropdown" href="#" { "UsernameXHD" }
									ul class="dropdown-menu" role="menu" {
										li { a href="userprof" { "Profil" }}
										li { a href="rfid" { "RFID Prikking" }}
										li { a href="bedrift" { "Bedrifter" }}
										li { a href="logging" { "Logg ut" }}
										li { a href="adminlogout" { "Logg ut" }}
									}
								}
							}
						}
					}
				}
				^PreEscaped(custom_html)
			}
		}
	]
}

gens! {

render() {
	h1 { "Hybrid web fråmewørk æÅØÆ" }
}

render2(user: &str, link: &str) {
	h1 {
		"Welcome, " ^user "!"
	}
	h2 {
		"Click " a href=^link { "here" } " to go back"
	}
}

}
