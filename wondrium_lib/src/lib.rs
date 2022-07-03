use serde::{Serialize, Deserialize};

#[cfg(not(target_arch = "wasm32"))]
pub mod constants;

#[cfg(not(target_arch = "wasm32"))]
pub mod utils;

#[cfg(not(target_arch = "wasm32"))]
pub mod downloader;


#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
pub struct Course {
    pub course_name: String,
    pub course_id: u32,
    pub course_description: String,

    #[serde(rename = "wondrium_description")]
    pub course_overview: String,

    pub course_guidebook_path: String,

    pub lectures: Vec<Lecture>,
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
pub struct Lecture {
    pub lecture_video_filename: String,
    pub lecture_videodownload_filename: String,
    pub lecture_image_filename: String,
    pub lecture_description: String,
    pub lecture_name: String,
}



