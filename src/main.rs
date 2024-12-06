#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

mod pages;

use pages::cookies::Cookies;
use pages::qcm::Qcm;

const _TAILWIND_URL: &str = manganis::mg!(file("public/tailwind.css"));

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/edit_cookie_preferences")]
    Cookies {},
    #[route("/qcm")]
    Qcm {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");
    launch(App);
}

fn App() -> Element {
    rsx! {
        link {
            rel: "stylesheet",
            href: "https://fonts.googleapis.com/css?family=Press Start 2P"
        }

        div {
          class: "h-[16px] bg-[#70BFFF] w-full"
        }

        Router::<Route> {}
    }
}

#[component]
fn Blog(id: i32) -> Element {
    rsx! {
        Link { to: Route::Home {}, "Go to counter" }
        "Blog post {id}"
    }
}

#[component]
fn Home() -> Element {
    rsx! {
        header {
          class: "justify-end px-6 hidden sm:flex h-[92px]",

          nav {
            class: "flex items-center gap-6",

            Link {
              class: "opacity-60 hover:opacity-100",
              to: Route::Home {},
              "Indexation"
            }
            Link {
              class: "px-4 py-2 border-2 border-[#70BFFF] text-[#70BFFF] hover:bg-[#70BFFF] hover:text-white",
              to: Route::Qcm {},
              "Faire le défi"
            }
          }
        }

        div {
          class: "px-6 relative container mx-auto h-[calc(100dvh-16px)] sm:h-[calc(100dvh-92px-16px)] flex items-center",

          div {
            class: "flex flex-col gap-4 w-full mt-[-160px]",
            
            h1 {
              class: "sm:text-xl font-bold text-[#000E1A]", 
              "Et si l'Océan était un corps humain ?"
            }
  
            Link {
              class: "bg-[#70BFFF] px-3 py-1.5 text-white w-fit",
              to: Route::Qcm {},
              "Commencer le défi !"
            }
          }
        }

        div {
          class: "flex flex-col items-center justify-center absolute bottom-0 inset-x-0",

          p {
            class: "text-center w-full mb-6 opacity-50",
            "voir pourquoi"
          }

          svg {
            width: 26,
            height: 15,
            view_box: "0 0 26 15",
            fill: "none",
            xmlns: "http://www.w3.org/2000/svg",
            class: "opacity-50",

            path {
              d: "M26 0L22.2857 -1.62358e-07L22.2857 3.71431L26 3.71431L26 0ZM18.5714 7.42862L22.2857 7.42862L22.2857 3.71431L18.5714 3.71431L18.5714 7.42862ZM14.8571 11.1429L18.5714 11.1429L18.5714 7.42862L14.8571 7.42862L14.8571 11.1429ZM11.1428 11.1429L11.1428 14.8572L14.8571 14.8572L14.8571 11.1429L11.1428 11.1429ZM7.42845 7.42862L11.1428 7.42862L11.1428 11.1429L7.42845 11.1429L7.42845 7.42862ZM7.42845 7.42862L7.42845 3.71431L3.71414 3.71431L3.71414 7.42862L7.42845 7.42862ZM-0.000165939 -1.1365e-06L3.71414 -9.74146e-07L3.71414 3.71431L-0.000166102 3.71431L-0.000165939 -1.1365e-06Z",
              fill: "black"
            }
          }

          svg {
            view_box: "0 0 2766 240",
            fill: "none",
            xmlns: "http://www.w3.org/2000/svg",

            path {
              opacity: "0.2",
              d: "M0 240V80C397.036 83.4667 794.072 86.9333 1087.67 90C1381.27 93.0667 1571.43 95.7333 1833.92 94C2096.4 92.2667 2431.2 86.1333 2766 80V240H0Z",
              fill: "#37A5FF"
            }
            path {
              opacity: "0.4",
              d: "M0 240V160C385.511 161.733 771.022 163.467 1028.61 164C1286.19 164.533 1415.85 163.867 1684.09 163C1952.33 162.133 2359.17 161.067 2766 160V240H0Z",
              fill: "#37A5FF"
            }
          }
        }

        section {
          class: "bg-[#70BFFF] py-16 px-6",

          div {
            class: "flex flex-col md:flex-row gap-8 px-6 container mx-auto",

            div {
              class: "shrink-0 w-[400px] h-[300px] bg-white"
            }

            div {
              h2 {
                class: "text-xl font-[Arial] font-bold text-[#003866]",
                "Et si l'océan était un corps humain ?"
              }
              p {
                class: "font-[Arial] text-[#002D52] pt-4",
                "C'est la question que nous nous sommes posés et qu'on va vous poser aujourd'hui, en vous invitant a participer à ce jeu interactif qui va vous permettre de découvrir les différentes ressemblances que peuvent avoir le corps humain et l'océan."
              }
            }
          }
        }
    }
}
