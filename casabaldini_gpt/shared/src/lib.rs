use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Slider {
    pub img: String,
    pub titolo: String,
    pub caption: String,
    pub testo: String,
}

