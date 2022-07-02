#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

fn main() {
    let context = tauri::generate_context!();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_course])
        .menu(tauri::Menu::os_default(&context.package_info().name))
        .run(context)
        .expect("error while running tauri application");
}

use wondrium_lib::downloader::WondriumDownloader;
use wondrium_lib::Course;

#[tauri::command]
fn get_course(course_link: String) -> Course {
    println!("Course link was {}", course_link);
    let wondrium_downloader = WondriumDownloader::new(&course_link);
    println!("{:?}", wondrium_downloader.course);
    wondrium_downloader.course
}
