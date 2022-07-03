#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

fn main() {
    let context = tauri::generate_context!();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_course, play_lecture])
        .menu(tauri::Menu::os_default(&context.package_info().name))
        .run(context)
        .expect("error while running tauri application");
}

use wondrium_lib::downloader::WondriumDownloader;
use wondrium_lib::Course;
use wondrium_lib::utils::get_lecture_m3u;

use std::process::Command;

#[tauri::command]
fn get_course(course_link: String) -> Course {
    println!("Course link was {}", course_link);
    let wondrium_downloader = WondriumDownloader::new(&course_link);
    println!("{:?}", wondrium_downloader.course);
    wondrium_downloader.course
}

#[tauri::command]
fn play_lecture(lecture_name: String) {
    let lecture_m3u = get_lecture_m3u(&lecture_name);
    println!("{}", lecture_m3u);
    let mpv_handle = Command::new("mpv")
        .arg(&lecture_m3u)
        .spawn()
        .expect("Unable to run mpv");
}
