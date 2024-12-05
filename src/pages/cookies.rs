use dioxus::prelude::*;
use getrandom::getrandom;
use web_sys::Window;

fn rand() -> u32 {
    let mut buf = [0u8; 4];
    getrandom(&mut buf).unwrap();

    u32::from_le_bytes(buf)
}

#[component]
pub fn Cookies() -> Element {
    let window = web_sys::window().unwrap();
    let mut x = use_signal(|| rand() as f64 % window.inner_width().unwrap().as_f64().unwrap());
    let mut y = use_signal(|| rand() as f64 % window.inner_height().unwrap().as_f64().unwrap());

    rsx! {
        div {
            onkeydown: move |event| {
                x.set(rand() as f64 % window.inner_width().unwrap().as_f64().unwrap());
                y.set(rand() as f64 % window.inner_height().unwrap().as_f64().unwrap());
            },

            img {
                src: "/cookie.png",
                class: "absolute",
                style: "top: {y}px; left: {x}px",

                width: 300,
                height: 300
            }
        }
    }
}
