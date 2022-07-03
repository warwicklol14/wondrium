use super::Course;

mod network;
mod scraper;

use super::constants::*;

pub struct WondriumDownloader {
    pub course: Course,
}

impl WondriumDownloader {
    pub fn new(course_url: &str) -> WondriumDownloader {
        let course_id = WondriumDownloader::get_course_id_from_url(course_url);

        let request_url = format!("{}/{}", COURSE_DETAILS_JSON_ENDPOINT, course_id);

        WondriumDownloader {
            course: network::get_json_from_url(&request_url),
        }
    }

    fn get_course_id_from_url(course_url: &str) -> u32 {
        let html = network::get_html_from_url(course_url);
        scraper::scrape_course_id_from_html(&html)
    }

    fn get_course_image_path(&self) -> String {
        format!(
            "{}/courses/{}/{}.jpg",
            IMAGES_ENDPOINT, self.course.course_id, self.course.course_id
        )
    }
}
