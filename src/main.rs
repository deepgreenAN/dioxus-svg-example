#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(non_snake_case)]

use dioxus::prelude::*;

fn SVGV1(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            color:"green",
            width: "200px",
            img {src: "/images/qiita.svg"}
        }
    })
}

fn SVGV2(cx: Scope) -> Element {
    let is_dark_mode = use_state(&cx, || false);

    let (color, button_text) = match is_dark_mode.get() {
        true => ("#CCFF99", "dark mode"),
        false => ("#006600", "light mode"),
    };

    let svg_str = include_str!("../images/qiita.svg");
    cx.render(rsx! {
        div {
            class: "svg2",
            color:"{color}",
            width: "200px",
            dangerous_inner_html: "{svg_str}",
        }
        button {
            onclick: move |_|{is_dark_mode.modify(|flag|!flag)},
            "{button_text}"
        }
    })
}

fn SVGV3(cx: Scope) -> Element {
    let is_dark_mode = use_state(&cx, || false);
    let has_frame = use_state(&cx, || false);

    let (color, button_text) = match is_dark_mode.get() {
        true => ("#CCFF99", "dark mode"),
        false => ("#006600", "light mode"),
    };

    cx.render(rsx! {
        div {
            class:"svg3",
            color:"{color}",
            width: "200px",
            svg {
                view_box: "0 0 113 113",
                style: "fill:currentColor",
                g {
                    transform:"translate(-55,-80)",
                    path {
                        d: "m 111.20497,81.034401 c -30.910533,0.19277 -55.811523,25.463479 -55.618323,56.443579 0.19321,30.9801 25.41089,56.24339 56.318013,55.74544 19.5846,-2.23034 26.83251,-4.47504 39.97327,-16.97468 -5.45746,-1.73633 -9.75709,-5.60544 -13.06328,-9.91102 -18.08498,14.61 -56.547003,14.11796 -62.935183,-13.93455 -2.29147,-10.5323 -2.20985,-21.02948 4.27054,-32.20785 l -2.51922,-10.06089 c 0.0693,-2.19933 1.38083,-2.33864 2.43809,-2.75332 0,0 5.15953,1.15211 9.17877,2.11564 10.773723,-8.31641 22.511833,-9.47741 34.949803,-5.19452 l 7.65225,-4.959909 c 1.42986,-0.6998 2.74024,-0.45643 3.85041,1.361159 l 0.45165,11.90006 c 7.54618,7.95455 9.94791,16.36757 11.68559,25.74778 1.14412,9.42027 -1.5172,13.34337 -2.03295,17.33124 0.96859,4.25133 2.52302,4.63261 5.39451,6.70501 3.52559,-0.3864 6.98819,-0.50527 10.04486,1.10123 3.24238,-7.2489 6.95861,-14.13841 6.27972,-26.70896 -0.83111,-30.96956 -25.408,-55.938212 -56.31852,-55.745439 z m -1.19683,26.600439 c -9.33096,0.0894 -18.645763,5.69308 -22.807393,18.10536 l -6.10092,-1.06867 -0.0889,1.38079 6.05596,0.75706 -0.3116,1.69189 -5.25446,0.26717 v 1.24695 l 5.21001,-0.31161 -0.26716,2.18178 -4.76509,1.73685 0.53434,1.20199 4.16357,-2.04794 c 1.4208,15.36341 12.2323,16.85853 24.425383,16.29771 8.69863,-1.14038 26.58239,-3.27313 23.52311,-21.26846 l 5.06532,1.09657 -0.0889,-1.06919 -5.07669,-1.00148 c 0,-0.0891 -0.62322,-1.84847 -0.62322,-1.84847 l 5.15421,-0.67903 -0.0331,-1.06867 -5.34386,0.61237 -0.63459,-1.80351 5.82238,-2.01486 -0.32298,-1.20251 -6.15621,2.08204 c -4.53426,-8.57575 -13.31383,-13.35814 -22.07927,-13.27413 z"
                    }
                }
                has_frame.get().then(||{
                    rsx! {rect {
                        style: "fill-opacity:0;stroke:currentColor;stroke-width:5.4;stroke-linecap:round;stroke-linejoin:round;stroke-dasharray:none;paint-order:markers stroke fill;stroke-miterlimit:4;stroke-opacity:1",
                        width: "113",
                        height: "113"
                        }
                    }
                })
            }
        }
        button {
            onclick: move |_|{is_dark_mode.modify(|flag|!flag)},
            "{button_text}"
        }
        button {
            onclick: move |_|{has_frame.modify(|flag|!flag)},
            "frame"
        }
    })
}

fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        SVGV1{}
        SVGV2{}
        SVGV3{}
    })
}

fn main() {
    dioxus::web::launch(App);
}
