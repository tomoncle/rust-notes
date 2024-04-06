/*
 * MIT License
 *
 * Copyright (c) 2023 tomoncle
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

use std::error::Error;
use std::ffi::OsStr;
use std::fs;
use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::path::Path;
use std::process::{exit, Command};

use chrono::Local;
use clap::Parser;

// 日志文件
const LOG_FILE: &str = ".transform-rs.log";

// cmd 处理
#[derive(Parser)]
#[command(name = "fileTransform")]
#[command(author = "tomoncle")]
#[command(version = "1.0")]
#[command(about = "本工具用于转换指定目录中的视频及图片文件，依赖于 https://ffmpeg.org/.", long_about = None)]
struct Args {
    // 对参数的描述信息，使用 `///` 来表示
    /// 需要转换的系统目录（绝对路径）
    #[arg(short, long)]
    input: Option<String>,

    /// 转换文件保存的目录（绝对路径）
    #[arg(short, long, default_value = "./")]
    output: Option<String>,
}

struct ConsoleLog {
    date_format: String,
}

impl ConsoleLog {
    fn new() -> Self {
        ConsoleLog {
            date_format: String::from("%Y-%m-%d %H:%M:%S"),
        }
    }
    fn info(&self, msg: &str) {
        let time = Local::now().format(&self.date_format);
        println!("\x1b[32m{time} [INFO]: {msg}\x1b[0m");
    }
    fn warning(&self, msg: &str) {
        let time = Local::now().format(&self.date_format);
        println!("\x1b[33m{time} [WARN]: {msg}\x1b[0m");
    }

    fn error(&self, msg: &str) {
        let time = Local::now().format(&self.date_format);
        println!("\x1b[31m{time} [ERROR]: {msg}\x1b[0m");
    }
}

struct Transformer {
    input_dir: String,
    output_dir: String,
}

impl Transformer {
    fn new(input_dir: String, output_dir: String) -> Self {
        Transformer {
            input_dir,
            output_dir,
        }
    }

    fn ffmpeg(&self) -> Command {
        Command::new("ffmpeg")
    }

    fn ffprobe(&self) -> Command {
        Command::new("ffprobe")
    }

    fn create_file_parent_dir(&self, target_file: &str) {
        let parent_path = Path::new(target_file).parent();
        if parent_path.is_none() {
            return;
        }
        if parent_path.unwrap().exists() {
            return;
        }
        let msg = format!("创建目标文件夹 {} 失败!", parent_path.unwrap().display());
        fs::create_dir_all(parent_path.unwrap()).expect(&msg);
    }

    fn trans_dir(&self) -> Result<(), Box<dyn Error>> {
        if self.check_input_dir_is_empty() {
            return Ok(());
        }
        let exec_dir = std::env::current_dir().unwrap();
        let home_dir = dirs::home_dir().unwrap_or(exec_dir);
        let log_path = home_dir.join(LOG_FILE).display().to_string();
        let filename = log_path.as_str();
        // 打开文件，如果文件不存在则创建它
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(filename)
            .expect(format!("打开日志文件失败: {}!", filename).as_str());
        // 获取更新前的文件内容
        let mut log_content = String::new();
        file.read_to_string(&mut log_content).expect("");

        // 使用 WalkDir 遍历文件夹
        for entry in walkdir::WalkDir::new(&self.input_dir)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.path().is_file() {
                let source_file = entry.path().display().to_string();
                let target_file = self.make_target_file(&source_file);
                if log_content.contains(&target_file) {
                    continue;
                }
                self.create_file_parent_dir(&target_file);
                let code: i32;
                if self.is_image_file(&source_file) {
                    code = self.trans_image(&source_file, &target_file)?;
                } else if self.is_video_file(&source_file) {
                    code = self.trans_video(&source_file, &target_file)?;
                } else {
                    self.copy_file(entry.path(), Path::new(&target_file));
                    code = 0;
                }
                if code != 0 {
                    continue;
                }
                // 获取当前时间
                let current_time = Local::now().format("%Y-%m-%d %H:%M:%S");
                // 向文件写入内容
                let value = format!("{}: {}\n", current_time, target_file);
                file.write_all(value.as_bytes()).expect("更新日志文件失败!");
            }
        }
        Ok(())
    }

    fn trans_video(&self, input_file: &str, output_file: &str) -> Result<i32, Box<dyn Error>> {
        // let duration = get_video_duration(input_file)?;
        // 文件损坏，不能解析；这里使用 match 表达式来处理
        let duration = match self.get_video_duration(input_file) {
            Ok(val) => val,
            Err(err) => {
                eprintln!("解析文件失败: {}, {}", input_file, err);
                String::new()
            }
        };
        if duration.is_empty() {
            println!("转换视频失败：{} -> {}", input_file, output_file);
            return Ok(-1);
        }

        let status = self
            .ffmpeg()
            .arg("-i")
            .arg(input_file)
            .arg("-ss")
            .arg("00:00:00.05")
            .arg("-to")
            .arg(duration)
            .arg("-c")
            .arg("copy")
            .arg("-y")
            .arg("-loglevel")
            .arg("error")
            .arg(output_file)
            .status()?;
        let code = status.code().unwrap_or(-1);
        if code == 0 {
            println!("转换视频成功：{} -> {}", input_file, output_file);
        } else {
            println!("转换视频失败：{} -> {}", input_file, output_file);
        }
        Ok(code)
    }

    fn trans_image(&self, input_file: &str, output_file: &str) -> Result<i32, Box<dyn Error>> {
        let status = self
            .ffmpeg()
            .arg("-i")
            .arg(input_file)
            .arg("-q:v")
            .arg("2")
            .arg("-y")
            .arg("-loglevel")
            .arg("error")
            .arg(output_file)
            .status()?;
        let code = status.code().unwrap_or(-1);
        if code == 0 {
            println!("转换图片成功：{} -> {}", input_file, output_file);
        } else {
            println!("转换图片失败：{} -> {}", input_file, output_file);
        }
        Ok(code)
    }

    fn get_video_duration(&self, input_file: &str) -> Result<String, Box<dyn Error>> {
        let output = self
            .ffprobe()
            .arg(input_file)
            .arg("-show_entries")
            .arg("format=duration")
            .arg("-v")
            .arg("quiet")
            .arg("-of")
            .arg("default=noprint_wrappers=1:nokey=1")
            .output()?;
        let duration = String::from_utf8(output.stdout)?.trim().to_string();
        let seconds = format!("{:.2}", duration.parse::<f64>()?);
        Ok(self.duration_format(seconds.parse::<f64>().unwrap()))
    }

    fn duration_format(&self, sec: f64) -> String {
        let sec = sec - 0.01;
        let hour = (sec.floor() / 3600.0) as i32;

        let remaining = sec - hour as f64 * 3600.0;
        let minute = (remaining.floor() / 60.0) as i32;

        let remaining = remaining - minute as f64 * 60.0;
        let second = remaining.floor() as i32;

        let millisecond = ((remaining - second as f64) * 100.0) as i32;
        format!("{:02}:{:02}:{:02}.{:02}", hour, minute, second, millisecond)
    }

    // fn get_root_dir(path: &Path) -> PathBuf {
    //     path.ancestors().last().unwrap_or(path).to_path_buf()
    // }

    fn is_video_file(&self, file_path: &str) -> bool {
        let video_extensions = [
            "mp4", "mkv", "avi", "mwv", "rm", "rmvb", "flv", "mov", "vob", "mpg", "qt", "mpeg",
            "ogg", "3gp",
        ];
        Path::new(file_path)
            .extension()
            .and_then(OsStr::to_str)
            .map(|ext| video_extensions.contains(&ext.to_lowercase().as_str()))
            .unwrap_or(false)
    }

    fn is_image_file(&self, file_path: &str) -> bool {
        let image_extensions = ["jpeg", "png", "gif", "bmp", "tiff", "webp", "heif"];
        Path::new(file_path)
            .extension()
            .and_then(OsStr::to_str)
            .map(|ext| image_extensions.contains(&ext.to_lowercase().as_str()))
            .unwrap_or(false)
    }

    fn make_target_file(&self, file_name: &str) -> String {
        let input_parent = Path::new(&self.input_dir).parent();
        return if input_parent.is_none() {
            format!(
                "{}/{}",
                &self.output_dir.trim_end_matches("/"),
                file_name.trim_start_matches("/")
            )
        } else {
            file_name.replace(
                &input_parent.unwrap().display().to_string(),
                &self.output_dir,
            )
        };
    }

    fn copy_file(&self, source_file: &Path, target_file: &Path) {
        fs::copy(source_file, target_file).expect(&format!(
            "拷贝文件 {} -> {} 失败!",
            source_file.display(),
            target_file.display()
        ));
    }

    fn check_input_dir_is_empty(&self) -> bool {
        if !Path::new(&self.input_dir).exists() {
            return true;
        }
        fs::read_dir(&self.input_dir)
            .map_or(0, |entries| entries.filter_map(|entry| entry.ok()).count())
            == 0
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let input = args.input.unwrap_or_else(|| "".to_owned());
    let mut output = args.output.unwrap_or_else(|| "./".to_owned());
    // let input = "E:\\下载\\百度云盘".to_string();
    // let output = "E:\\backup";
    let log = ConsoleLog::new();
    if input.is_empty() {
        let os = std::env::consts::OS;
        match os {
            "linux" => log.error("请输入输入要转换文件的目录，例如：/home/test/mounts，使用[-h]参数获取更多使用说明!"),
            "macos" => log.error("请输入输入要转换文件的目录，例如：/Users/test/files，使用[-h]参数获取更多使用说明!"),
            "windows" => log.error("请输入输入要转换文件的目录，例如：D:\\test\\files，使用[-h]参数获取更多使用说明!"),
            _ => log.error("Unknown OS"),
        }
        exit(0);
    }

    if output == "./" {
        output = std::env::current_dir().unwrap().display().to_string();
        log.warning("使用当前目录作为输出目录.")
    }

    if !Path::new(&input).exists() {
        log.error("输入目录不存在!");
        exit(0);
    }
    if !Path::new(&output).exists() {
        log.error("输出目录不存在!");
        exit(0);
    }

    log.info(&format!(
        "当前执行目录：{}",
        std::env::current_dir().unwrap().display()
    ));
    Transformer::new(input, output).trans_dir()?;
    Ok(())
}
