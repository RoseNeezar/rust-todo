use maud::{html, Markup, DOCTYPE};

fn header() -> Markup {
    html! {
        (DOCTYPE)
        title { "TODO" }
        script src="https://unpkg.com/htmx.org@1.9.5" {}
        script src="https://cdn.tailwindcss.com" {}
        link href="https://cdn.jsdelivr.net/npm/daisyui@4.4.17/dist/full.min.css" rel="stylesheet" type="text/css" {}
    }
}

pub fn index() -> Markup {
    html! {
        (header())

        div class="grid grid-cols-1 justify-items-center bg-green-900" {

            div class="border max-w-7xl" {

              div class="bgr-red-300 w-10 h-10" {
               h1{"pl"}
              }
            }
        }
    }
}
