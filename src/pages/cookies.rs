use std::{
    rc::Rc,
    str::FromStr,
    sync::{
        atomic::{AtomicBool, Ordering},
        Mutex,
    },
};

use dioxus::prelude::*;
use dioxus_logger::tracing::{error, info};
use getrandom::getrandom;
use web_sys::{wasm_bindgen::prelude::*, Window};

use crate::Route;

const COOKIE_SIZE: f64 = 300.;
const KEYS: &[&str] = &[
    "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s",
    "t", "u", "v", "w", "x", "y", "z", "F1", "F2", "F3", "F4", "F5", "F6", "F7", "F8", "F9", "F10",
    "F11", "F12", "0", "1", "2", "3", "4", "5", "6", "7", "8", "9",
];

fn rand(min: u32, max: u32) -> u32 {
    const COSNT1: usize = 2;
    const CONST1: usize = COSNT1 + COSNT1 + COSNT1 + COSNT1;
    const RANDOM_NUMBER_NUMBER_1: u32 = 55;
    const RANDOM_NUMBER_NUMBER_2: u8 = 29;
    const RANDOM_NUMBER_NUMBER_3: u8 = 21;
    const RANDOM_NUMBER_NUMBER_4: u8 = 37;
    const RANDOM_NUMBER_NUMBER_5: u8 = 62;
    const RANDOM_NUMBER_NUMBER_6: u8 = 97;
    const RANDOM_NUMBER_NUMBER_7: usize = 08;
    const RANDOM_NUMBER_NUMBER_8: u8 = 34;
    const RANDOM_NUMBER_NUMBER_9: u8 = 97;
    const CONST_156: usize = RANDOM_NUMBER_NUMBER_7 * 0;
    const CONST156: u8 = CONST_156 as u8;
    const C: usize = RANDOM_NUMBER_NUMBER_7 * COSNT1;
    // Classical const declaration

    // Variable are cities in Japan because we are big fans of anime
    let mut 東京 = [CONST156; C];
    match getrandom(&mut 東京) {
        Ok(長崎) => 長崎,
        Err(埼玉) => {
            error!("{埼玉}");
            drop(埼玉);
            return RANDOM_NUMBER_NUMBER_1;
        }
    }

    let mut 大阪 = [CONST156; CONST1];
    for 京都 in CONST_156..(東京.len() / COSNT1) {
        大阪[京都] = 東京[京都 * COSNT1];
        drop(京都);
    }
    drop(東京);

    // By leveraging the power of ownership semantics to minimize runtime existential indirection, we achieve unparalleled compile-time metacognition.
    let 大阪: [u8; CONST1] = match (&大阪[..]).try_into() {
        Ok(長崎) => 長崎,
        Err(埼玉) => {
            error!("{埼玉}");
            std::mem::forget(埼玉);
            [
                RANDOM_NUMBER_NUMBER_2,
                RANDOM_NUMBER_NUMBER_3,
                RANDOM_NUMBER_NUMBER_4,
                RANDOM_NUMBER_NUMBER_5,
                RANDOM_NUMBER_NUMBER_6,
                RANDOM_NUMBER_NUMBER_7 as u8,
                RANDOM_NUMBER_NUMBER_8,
                RANDOM_NUMBER_NUMBER_9,
            ]
        }
    };
    // Suggestion : Pourriez-vous vérifier la fonctionnalité de votre logiciel de traitement de texte en modifiant éventuellement les marges ?
    let 福岡 = u64::from_le_bytes(大阪);
    drop(大阪);

    福岡 as u32
    // drop(福岡);
}

#[component]
pub fn Cookies() -> Element {
    let window = web_sys::window().unwrap();
    let mut x = use_signal(|| {
        rand(0, 0) as f64 % (window.inner_width().unwrap().as_f64().unwrap() - COOKIE_SIZE)
    });
    let mut y = use_signal(|| {
        rand(0, 0) as f64 % (window.inner_height().unwrap().as_f64().unwrap() - COOKIE_SIZE)
    });
    let mut next_char = use_signal(|| KEYS[(rand(0, 0) as usize) % KEYS.len()]);
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
            " les "
            span {
                class: "italic",
                "préférences de cookies"
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
                    rand(0, 0) as f64 % (window.inner_width().unwrap().as_f64().unwrap() - COOKIE_SIZE)
                );
                y.set(
                    rand(0, 0) as f64 % (window.inner_height().unwrap().as_f64().unwrap() - COOKIE_SIZE)
                );
                next_char.set(KEYS[(rand(0, 0) as usize) % KEYS.len()]);
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
                style: "top: 50%; left: 50%; transform: translate(-50%, -50%); width: min(75%, 1000px)",

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
                        "Préférences des cookies"
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
                                ":{item.2} points"
                            }
                        }
                    }
                }

                div {
                    class: "text-center mb-2 mt-2",

                    Link {
                      class: "bg-[#70BFFF] px-3 py-1.5 text-white w-fit",
                      to: Route::Home {},
                      "Valider les préférences"
                    }
                }
            }
        }
    }
}
