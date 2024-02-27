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
use std::process::Command;

fn main() -> Result<(), Box<dyn Error>> {
    let size = get_video_duration("E:\\下载\\阿里云盘\\冰雪奇缘2国语配音1080p.mp4")?;
    println!("file size: {}", size);

    let status = trans_video(
        "E:\\下载\\阿里云盘\\冰雪奇缘2国语配音1080p.mp4",
        "E:\\下载\\冰雪奇缘2国语配音1080p.mp4")?;
    println!("transfer status: {}", status);
    Ok(())
}

/// 视频转换
/// ```
/// let status = trans_video("E:\\下载\\1.mp4", "E:\\下载\\2.mp4")?;
/// println!("transfer status: {}", status);
/// ```
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
    println!("转换成功：{} -> {}", input_file, output_file);
    Ok(status.code().unwrap())
}

/// 图片转换
/// ```
/// let status = trans_image("E:\\下载\\1.jpeg", "E:\\下载\\2.png")?;
/// println!("transfer status: {}", status);
/// ```
fn trans_image(input_file: &str, output_file: &str) -> Result<i32, Box<dyn Error>> {
    let status = Command::new("ffmpeg")
        .arg("-i")
        .arg(input_file)
        .arg("-q:v 2")
        .arg("-y")
        .arg(output_file)
        .status()?;
    println!("转换成功：{} -> {}", input_file, output_file);
    Ok(status.code().unwrap())
}

/// 获取视频时长
/// ```
/// let size = get_video_duration("E:\\下载\\冰雪奇缘2国语配音1080p.mp4")?;
/// println!("file size: {}", size);
/// ```
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

/// 秒钟转换为 ffmpeg 支持的时间格式
/// ```
/// let duration = "2008.21";
/// let seconds = format!("{:.2}", duration.parse::<f64>()?);
/// let time_format = sec_to_time(seconds.parse::<f64>().unwrap());
/// ```
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

