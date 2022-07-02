use wondrium_lib::downloader::WondriumDownloader;

fn main() {
    let wondrium_downloader = WondriumDownloader::new("https://www.wondrium.com/learning-italian-step-by-step-and-region-by-region");
    println!("{:?}", wondrium_downloader.course);
}
