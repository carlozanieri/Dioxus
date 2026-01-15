use dioxus::prelude::*;
use shared::Slider;

#[component]
pub fn HomePage() -> Element {
    let sliders = use_resource(|| async {
        reqwest::get("/api/sliders")
            .await
            .unwrap()
            .json::<Vec<Slider>>()
            .await
            .unwrap()
    });

    rsx! {
        HeadContent {}

    match &*sliders.read() {
    Some(sliders) => rsx!(
        // usa sliders: &Vec<Slider>
    ),
    None => rsx!(
        p { "Caricamento..." }
    ),
}


    }
}

#[component]
fn SliderPro(sliders: Vec<Slider>) -> Element {
    rsx! {
        div { id: "example1", class: "slider-pro",
            div { class: "sp-slides",
                for s in sliders {
                    div { class: "sp-slide",
                        img { src: "/static/img/index/{s.img}" }
                        h3 { "{s.titolo}" }
                        p { "{s.caption}" }
                        p { "{s.testo}" }
                    }
                }
            }
        }
    }
}

#[component]
fn HeadContent() -> Element {
    rsx! {
        head {
            link { rel: "stylesheet", href: "/static/home/dist/css/slider-pro.min.css" }
            script { src: "https://code.jquery.com/jquery-3.6.2.min.js" }
            script { src: "/static/home/dist/js/jquery.sliderPro.min.js" }
            script { src: "/static/home/slider-init.js" }
        }
    }
}