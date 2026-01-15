use dioxus::prelude::*;
use crate::home::HomePage;

pub fn App() -> Element {
    rsx! {
        HomePage {}
    }
}