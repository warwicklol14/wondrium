use super::constants::{LECTURE_M3U_HEADER, LECTURE_M3U_TAIL};

pub fn get_lecture_m3u(lecture_name: &str) -> String {
    format!("{}/{}{}",LECTURE_M3U_HEADER,lecture_name,LECTURE_M3U_TAIL)
}
