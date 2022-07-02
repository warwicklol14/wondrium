use select::document::Document;
use select::predicate::Attr;

use super::constants::*;

pub fn scrape_course_id_from_html(html: &str) -> u32 {
        let document = Document::from(html);
        let course_image_meta_tag = document.find(Attr(COURSE_SKU_SELECTOR_ATTRIBUTE_NAME, COURSE_SKU_SELECTOR_ATTRIBUTE_VALUE)).next().unwrap();
        let course_image_url = course_image_meta_tag.attr("content").unwrap();
        let course_id = course_image_url.split("/").nth(8).unwrap();
        course_id.trim().parse().expect("Unable to parse course id")
}
