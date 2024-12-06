use std::{
    rc::Rc,
    str::FromStr,
    sync::{
        atomic::{AtomicBool, Ordering},
        Mutex,
    },
};

use dioxus::prelude::*;
use dioxus_logger::tracing::info;
use getrandom::getrandom;
use web_sys::{wasm_bindgen::prelude::*, Window};

const COOKIE_SIZE: f64 = 300.;
const KEYS: &[&str] = &[
    "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s",
    "t", "u", "v", "w", "x", "y", "z", "F1", "F2", "F3", "F4", "F5", "F6", "F7", "F8", "F9", "F10",
    "F11", "F12", "0", "1", "2", "3", "4", "5", "6", "7", "8", "9",
];

fn rand() -> u32 {
    let mut buf = [0u8; 4];
    getrandom(&mut buf).unwrap();

    u32::from_le_bytes(buf)
}

#[component]
pub fn Cookies() -> Element {
    let window = web_sys::window().unwrap();
    let mut x = use_signal(|| {
        rand() as f64 % (window.inner_width().unwrap().as_f64().unwrap() - COOKIE_SIZE)
    });
    let mut y = use_signal(|| {
        rand() as f64 % (window.inner_height().unwrap().as_f64().unwrap() - COOKIE_SIZE)
    });
    let mut next_char = use_signal(|| KEYS[(rand() as usize) % KEYS.len()]);
    let mut mouse_is_hover = use_signal(|| false);
    let mut score = use_signal(|| 0);
    let mut is_menu_open = use_signal(|| false);
    let mut items = use_signal(|| {
        vec![
            ("CookiX", false, 10),
            ("DataBite", false, 20),
            ("Crumbly", false, 50),
            ("WebChip", false, 100),
            ("NetMorsel", false, 200),
            ("Désactivation des cookies de tracking", false, 1000),
        ]
    });

    rsx! {
        p {
            class: "text-center",
            "Pour gagner des points, appuyez sur la touche écrite en arrière-plan en ayant le curseur"
            " de la souris sur le cookie"
        }
        p {
            class: "text-center",

            span {
                class: "font-bold",
                "Cliquez "
            }
            span {
                onclick: move |_| {
                    is_menu_open.set(true);
                },
                "ici"
            },
            " pour "
            span {
                class: "underline",
                "ouvrir"
            },
            " le "
            span {
                class: "italic",
                "menu"
            }
        }
        p {
            class: "text-center text-[red]",
            "Si ca marche pas faut cliquer sur le cookie pour le focus"
        }

        p {
            class: "fixed -z-10 text-slate-300 inset-0 text-center",
            style: "font-size: 400px;",

            "{next_char}"
        }
        p {
            class: "fixed -z-10 text-slate-300 inset-0 text-center",
            style: "font-size: 400px; line-height: 1.1em",

            br {}
            "{score}"
        }

        div {
            tabindex: "0",
            onmounted: move |cx| {
                cx.data().set_focus(true);
            },
            onkeydown: move |event| {
                let expected_key = Key::from_str(&next_char.read()).unwrap();

                if !(*mouse_is_hover.read()) {
                    return;
                }

                if expected_key != event.data.key()  {
                    let new_score = *score.read() - 1;
                    score.set(new_score);
                    return;
                }

                let new_score = *score.read() + 1;
                score.set(new_score);
                x.set(
                    rand() as f64 % (window.inner_width().unwrap().as_f64().unwrap() - COOKIE_SIZE)
                );
                y.set(
                    rand() as f64 % (window.inner_height().unwrap().as_f64().unwrap() - COOKIE_SIZE)
                );
                next_char.set(KEYS[(rand() as usize) % KEYS.len()]);
            },
            onmouseenter: move |_| {
                mouse_is_hover.set(true);
            },
            onmouseleave: move |_| {
                mouse_is_hover.set(false);
            },

            id: "cookie",

            img {
                src: "/cookie.png",
                class: "absolute",
                style: "top: {y}px; left: {x}px",

                width: COOKIE_SIZE,
                height: COOKIE_SIZE
            }
        },

        if is_menu_open() {
            div {
                class: "z-999 bg-white border border-solid border-black absolute",
                style: "top: 50%; left: 50%; transform: translate(-50%, -50%); width: min(75%, 700px)",

                div {
                    class: "w-full border-b border-solid border-black text-center flex flex-row",
                    p {
                        style: "width: 1em;",
                        onclick: move |_| {
                            is_menu_open.set(false);
                        },
                        "x"
                    }
                    p {
                        class: "flex-1",
                        "Menu"
                    }
                }

                for item in items() {
                    div {
                        class: "w-full flex flex-row",
                        p {
                            onclick: move |_| {
                                let new_score = *score.read() - item.2;
                                score.set(new_score);

                                let mut i = items.read().clone();
                                let index = i.iter().position(|i| i.0 == item.0).unwrap();
                                i[index].1 = true;

                                items.set(i);
                            },
                            "{item.0}"
                        },
                        if item.1 {
                            p {
                                class: "flex-1 text-right",
                                "débloqué"
                            }
                        } else {
                            button {
                                class: "flex-1 text-right",

                                if item.2 > score() {
                                    p {
                                        class: "inline text-[red]",
                                        "acheter"
                                    }
                                } else {
                                    p {
                                        class: "inline text-[green]",
                                        "acheter"
                                    }
                                }
                                ":{item.2}"
                            }
                        }
                    }
                }
            }
        }
    }
}
