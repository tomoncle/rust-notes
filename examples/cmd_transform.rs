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
use std::path::Path;
use std::process::{Command, exit};

use clap::Parser;

#[derive(Parser)]
#[command(name = "fileTransform")]
#[command(author = "tomoncle")]
#[command(version = "1.0")]
#[command(about = "转换指定目录中的视频及图片文件.", long_about = None)]
struct Args {
    // 注意下面的注释是三个斜杠!!!
    /// 需要转换的系统目录（绝对路径）
    #[arg(short, long)]
    input: Option<String>,

    /// 转换文件保存的目录（绝对路径）
    #[arg(short, long, default_value = "./")]
    output: Option<String>,
}


fn trans_video(input_file: &str, output_file: &str) -> Result<i32, Box<dyn Error>> {
    let duration = get_video_duration(input_file)?;
    let status = Command::new("ffmpeg")
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
        .arg("warning")
        .arg(output_file)
        .status()?;
    println!("转换视频成功：{} -> {}", input_file, output_file);
    Ok(status.code().unwrap())
}


fn trans_image(input_file: &str, output_file: &str) -> Result<i32, Box<dyn Error>> {
    let status = Command::new("ffmpeg")
        .arg("-i")
        .arg(input_file)
        .arg("-q:v 2")
        .arg("-y")
        .arg(output_file)
        .status()?;
    println!("转换图片成功：{} -> {}", input_file, output_file);
    Ok(status.code().unwrap())
}

fn get_video_duration(input_file: &str) -> Result<String, Box<dyn Error>> {
    let output = Command::new("ffprobe")
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
    Ok(sec_to_time(seconds.parse::<f64>().unwrap()))
}

fn sec_to_time(sec: f64) -> String {
    let sec = sec - 0.01;
    let hour = (sec.floor() / 3600.0) as i32;

    let remaining = sec - hour as f64 * 3600.0;
    let minute = (remaining.floor() / 60.0) as i32;

    let remaining = remaining - minute as f64 * 60.0;
    let second = remaining.floor() as i32;

    let millisecond = ((remaining - second as f64) * 100.0) as i32;
    format!("{:02}:{:02}:{:02}.{:02}", hour, minute, second, millisecond)
}

fn get_root_dir(path: &Path) -> std::path::PathBuf {
    path.ancestors().last().unwrap_or(path).to_path_buf()
}

fn is_video_file(file_path: &str) -> bool {
    let video_extensions = [
        "mp4", "mkv", "avi", "mwv", "rm", "rmvb", "flv",
        "mov", "vob", "mpg", "qt", "mpeg", "ogg", "3gp"];
    if let Some(ext) = Path::new(file_path).extension() {
        if let Some(ext_str) = ext.to_str() {
            return video_extensions.contains(&ext_str.to_lowercase().as_str());
        }
    }
    false
}

fn is_image_file(file_path: &str) -> bool {
    let image_extensions = ["jpeg", "png", "gif", "bmp", "tiff", "webp", "heif"];
    // if let Some(ext) = Path::new(file_path).extension() {
    //     if let Some(ext_str) = ext.to_str() {
    //         return image_extensions.contains(&ext_str.to_lowercase().as_str());
    //     }
    // }
    // false
    Path::new(file_path)
        .extension()
        .and_then(OsStr::to_str)
        .map(|ext| image_extensions.contains(&ext.to_lowercase().as_str()))
        .unwrap_or(false)
}

fn file_rename(file_name: &str, input_dir: &str, output_dir: &str) -> String {
    let input_parent = Path::new(input_dir).parent();
    return if input_parent.is_none() {
        format!("{}/{}", output_dir.trim_end_matches("/"), file_name.trim_start_matches("/"))
    } else {
        file_name.replace(&input_parent.unwrap().display().to_string(), output_dir)
    };
}

fn copy_file(source_file: &Path, target_file: &Path) {
    let parent_path = target_file.parent();
    let mut msg;
    if !parent_path.is_none() && !parent_path.unwrap().exists() {
        msg = format!("创建文件夹 {} 失败!", parent_path.unwrap().display());
        fs::create_dir_all(parent_path.unwrap()).expect(&msg);
    }
    msg = format!("拷贝文件 {} -> {} 失败!", source_file.display(), target_file.display());
    fs::copy(source_file, target_file).expect(&msg);
}

fn dir_empty(dir_path: &str) -> bool {
    fs::read_dir(dir_path).map_or(0, |entries|
        entries.filter_map(|entry| entry.ok()).count()) == 0
}

fn loop_dir(input_dir: &str, output_dir: &str) -> Result<(), Box<dyn Error>> {
    if dir_empty(input_dir) {
        return Ok(())
    }
    // 使用 WalkDir 遍历文件夹
    for entry in walkdir::WalkDir::new(input_dir).into_iter().filter_map(|e| e.ok()) {
        if entry.path().is_file() {
            let source_file = entry.path().display().to_string();
            let target_file = file_rename(&source_file, input_dir, output_dir);
            let parent_path = Path::new(&target_file).parent();
            if !parent_path.is_none() && !parent_path.unwrap().exists() {
                let msg = format!("创建目标文件夹 {} 失败!", parent_path.unwrap().display());
                fs::create_dir_all(parent_path.unwrap()).expect(&msg);
            }
            if is_image_file(&source_file) {
                trans_image(&source_file, &target_file)?;
            } else if is_video_file(&source_file) {
                trans_video(&source_file, &target_file)?;
            } else {
                copy_file(entry.path(), Path::new(&target_file));
            }
        }
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let input = args.input.unwrap_or_else(|| "".to_owned());
    let output = args.output.unwrap_or_else(|| "./".to_owned());
    let os = std::env::consts::OS;
    // println!("{input} {output}");
    if !input.is_empty() && output != "./" {
        if !Path::new(&input).exists() {
            println!("Error：输入目录不存在!");
            exit(1);
        }
        if !Path::new(&output).exists() {
            println!("Error：输出目录不存在!");
            exit(1);
        }
        match os {
            "linux" => println!("Current OS is Linux"),
            "macos" => println!("Current OS is macOS"),
            "windows" => println!("Current OS is Windows"),
            _ => println!("Unknown OS"),
        }
        println!("当前执行目录：{}", std::env::current_dir().unwrap().display());
        loop_dir(&input, &output)?;
    } else {
        println!("Error：请输入输入目录和输出目录!");
    };
    Ok(())
}

