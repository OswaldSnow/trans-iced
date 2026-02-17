use std::{env, process::Command};

use trans_iced::app_gui::AppState;

const APP_DIR_NAME: &str = ".translate_app";
const TEMP_PNG_NAME: &str = "shot.png";

fn main() -> iced::Result {
    let mut args = env::args().skip(1);

    let (appid, appkey) = parse_args_or_exit(&mut args);

    let model = parse_args_or_default(&mut args);

    let mut text = String::default();

    if !"jw".eq(&model) {
        text = run_ocr().expect("ocr failed");
    }

    iced::application(
        move || AppState::new(&text, appid.clone(), appkey.clone()),
        AppState::update,
        AppState::view,
    )
    .run()
}

fn parse_args_or_exit<I>(args: &mut I) -> (String, String)
where
    I: Iterator<Item = String>,
{
    let appid = args.next().unwrap_or_else(|| usage_and_exit());
    let appkey = args.next().unwrap_or_else(|| usage_and_exit());
    (appid, appkey)
}

fn parse_args_or_default<I>(args: &mut I) -> String
where
    I: Iterator<Item = String>,
{
    let model = args.next().unwrap_or_default();

    model
}

fn usage_and_exit() -> ! {
    eprintln!("Usage: translate_app <APPID> <APPKEY>");
    std::process::exit(2);
}

fn run_ocr() -> Result<String, Box<dyn std::error::Error>> {
    let work_dir = home::home_dir()
        .expect("cannot determine home dir")
        .join(APP_DIR_NAME);
    let full_png_path = work_dir.join(TEMP_PNG_NAME);

    std::fs::create_dir_all(work_dir).expect("create dit fail"); // 确保目录存在

    let path = full_png_path.to_string_lossy().to_string();

    Command::new("rm").args(["-rf", &path]).status()?;

    Command::new("flameshot")
        .args(["gui", "--path", &path])
        .status()?;

    let out = Command::new("tesseract")
        .args([&path, "-", "-l", "eng+chi_sim"])
        .output()?;

    if !out.status.success() {
        return Err(format!("tesseract failed: {}", String::from_utf8_lossy(&out.stderr)).into());
    }

    Ok(String::from_utf8_lossy(&out.stdout).to_string())
}
