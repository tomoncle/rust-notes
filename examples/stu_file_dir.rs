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
use std::ffi::OsStr;
use std::fs;
use std::path::Path;

fn main() {
    let os = std::env::consts::OS;

    match os {
        "linux" => println!("Current OS is Linux"),
        "macos" => println!("Current OS is macOS"),
        "windows" => println!("Current OS is Windows"),
        _ => println!("Unknown OS"),
    }
    println!("当前执行目录：{}", std::env::current_dir().unwrap().display());

    let input_dir = "E:\\下载\\阿里云盘";
    let output_dir = "D:\\test";

    // 使用 WalkDir 遍历文件夹
    for entry in walkdir::WalkDir::new(input_dir).into_iter().filter_map(|e| e.ok()) {
        if entry.path().is_file() {
            println!("源文件路径：{}", entry.path().display().to_string());
            let parent_dir = Path::new(entry.path()).parent().unwrap();
            println!("源文件夹路径：{}", parent_dir.display());
            let root_dir = get_root_dir(entry.path());
            println!("源文件根路径：{}", root_dir.display());

            // let target_file = entry.path().display().to_string().to_lowercase()
            //     .replace(input_dir, output_dir);
            let target_file = file_rename(&entry.path().display().to_string(), input_dir, output_dir);
            println!("目标文件路径：{}", target_file);
            let parent_dir = Path::new(&target_file).parent().unwrap();
            println!("目标文件夹路径：{}", parent_dir.display());
            let root_dir = get_root_dir(Path::new(&target_file));
            println!("目标文件根路径：{}", root_dir.display());

            let is_video = is_video_file(&entry.path().display().to_string());
            println!("{} , 是否为视频文件: {:?}", entry.path().display(), is_video);
            // 换行
            println!()
        }
    }
}

fn get_root_dir(path: &Path) -> std::path::PathBuf {
    path.ancestors().last().unwrap_or(path).to_path_buf()
}

fn is_video_file(file_path: &str) -> bool {
    let video_extensions = ["mp4", "mkv", "avi", "mwv", "rm", "rmvb", "flv", "mov",
        "vob", "mpg", "qt", "mpeg", "ogg", "3gp"];
    if let Some(ext) = std::path::Path::new(file_path).extension() {
        if let Some(ext_str) = ext.to_str() {
            return video_extensions.contains(&ext_str.to_lowercase().as_str());
        }
    }
    false
}

fn is_image_file(file_path: &str) -> bool {
    let image_extensions = ["jpeg", "png", "gif", "bmp", "tiff", "webp", "heif"];
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