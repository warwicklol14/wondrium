use js_sys;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures;
use wondrium_lib::Course;
use serde::{Serialize};

#[wasm_bindgen(
    inline_js = "export function invoke_tauri(cmd, args = {}) { return window.__TAURI__.invoke(cmd, args=args) }"
)]
extern "C" {
    async fn invoke_tauri(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CourseCommandInput {
    course_link: String
}

pub async fn get_course(course_link: String) -> Course {
    let course_command_input = CourseCommandInput {
        course_link
    };
    let course_link_js = JsValue::from_serde(&course_command_input).unwrap();
    let course = invoke_tauri("get_course", course_link_js).await;
    JsValue::into_serde(&course).unwrap()
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PlayLectureCommandInput {
    lecture_name: String
}

pub async fn play_lecture(lecture_name: String) {
    let play_lecture_command_input = PlayLectureCommandInput {
        lecture_name
    };
    let lecture_name_js = JsValue::from_serde(&play_lecture_command_input).unwrap();
    invoke_tauri("play_lecture", lecture_name_js).await;
}
