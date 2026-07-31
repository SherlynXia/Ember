use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use std::sync::Mutex;
use chrono::{Local, Timelike};
use lazy_static::lazy_static;

// 已播放音频记录（按文件夹分类）- 内存中，每次启动重置
lazy_static! {
    static ref PLAYED_STARTUP: Mutex<Vec<String>> = Mutex::new(Vec::new());
    static ref PLAYED_MORNING: Mutex<Vec<String>> = Mutex::new(Vec::new());
    static ref PLAYED_NOON: Mutex<Vec<String>> = Mutex::new(Vec::new());
    static ref PLAYED_AFTERNOON: Mutex<Vec<String>> = Mutex::new(Vec::new());
    static ref PLAYED_EVENING: Mutex<Vec<String>> = Mutex::new(Vec::new());
}

/// 获取音频目录（尝试多个可能的位置）
fn get_audio_dir() -> Result<PathBuf, String> {
    let cwd = std::env::current_dir()
        .map_err(|e| format!("Failed to get cwd: {}", e))?;

    let dir1 = cwd.join("assets").join("pet").join("audio");
    if dir1.exists() {
        return Ok(dir1);
    }

    let dir2 = cwd.join("src-tauri").join("assets").join("pet").join("audio");
    if dir2.exists() {
        return Ok(dir2);
    }

    if let Ok(exe_path) = std::env::current_exe() {
        if let Some(exe_dir) = exe_path.parent() {
            let dir3 = exe_dir.join("assets").join("pet").join("audio");
            if dir3.exists() {
                return Ok(dir3);
            }
        }
    }

    Err(format!("Audio directory not found. Cwd: {:?}", cwd))
}

/// 获取文件夹中的所有音频文件
fn get_audio_files(dir: &PathBuf) -> Result<Vec<PathBuf>, String> {
    let files: Vec<PathBuf> = std::fs::read_dir(dir)
        .map_err(|e| format!("Failed to read directory: {}", e))?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| {
            path.extension()
                .and_then(|ext| ext.to_str())
                .map(|ext| {
                    let ext = ext.to_lowercase();
                    ext == "mp3" || ext == "wav" || ext == "ogg" || ext == "flac"
                })
                .unwrap_or(false)
        })
        .collect();

    Ok(files)
}

/// 选择未播放过的音频（轮播机制）
fn select_unplayed(files: &mut Vec<PathBuf>, played: &mut Vec<String>) -> Result<PathBuf, String> {
    if files.is_empty() {
        return Err("No audio files found".to_string());
    }

    // 过滤掉已播放的
    let unplayed: Vec<PathBuf> = files
        .iter()
        .filter(|f| {
            let name = f.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("");
            !played.contains(&name.to_string())
        })
        .cloned()
        .collect();

    let selected = if unplayed.is_empty() {
        // 所有都播放过了，重置记录并随机选一个
        played.clear();
        let index = rand::random::<usize>() % files.len();
        files[index].clone()
    } else {
        // 从未播放的中随机选一个
        let index = rand::random::<usize>() % unplayed.len();
        unplayed[index].clone()
    };

    // 安全检查：如果记录超过文件数量的2倍，强制清空（防止异常累积）
    if played.len() > files.len() * 2 {
        played.clear();
    }

    // 记录已播放
    if let Some(name) = selected.file_name().and_then(|n| n.to_str()) {
        played.push(name.to_string());
    }

    Ok(selected)
}

/// 播放音频文件
#[tauri::command]
pub fn play_audio(path: String) -> Result<(), String> {
    let audio_dir = get_audio_dir()?;
    let audio_path = audio_dir.join(&path);

    if !audio_path.exists() {
        return Err(format!("Audio file not found: {:?}", audio_path));
    }

    // 在新线程中播放音频
    std::thread::spawn(move || {
        match OutputStream::try_default() {
            Ok((stream, stream_handle)) => {
                match Sink::try_new(&stream_handle) {
                    Ok(sink) => {
                        if let Ok(file) = File::open(&audio_path) {
                            if let Ok(source) = Decoder::new(BufReader::new(file)) {
                                sink.append(source);
                                sink.sleep_until_end();
                            }
                        }
                    }
                    Err(_) => {}
                }
                let _ = stream;
            }
            Err(_) => {}
        }
    });

    Ok(())
}

/// 播放随机启动音频（轮播机制）
#[tauri::command]
pub fn play_random_startup_audio() -> Result<(), String> {
    let audio_dir = get_audio_dir()?;
    let startup_dir = audio_dir.join("startup");

    if !startup_dir.exists() {
        return Err("Startup audio directory not found".to_string());
    }

    let mut files = get_audio_files(&startup_dir)?;
    let mut played = PLAYED_STARTUP.lock().map_err(|e| e.to_string())?;
    let selected = select_unplayed(&mut files, &mut played)?;

    let relative_path = selected
        .strip_prefix(&audio_dir)
        .map_err(|_| "Failed to get relative path".to_string())?
        .to_string_lossy()
        .replace("\\", "/");

    play_audio(relative_path)
}

/// 播放时段音频（根据当前时间选择时段，轮播机制）
#[tauri::command]
pub fn play_random_time_audio() -> Result<(), String> {
    let hour = Local::now().hour();

    let time_folder = match hour {
        6..=10 => "morning",
        11..=13 => "noon",
        14..=17 => "afternoon",
        18..=23 | 0..=5 => "evening",
        _ => "morning",
    };

    let audio_dir = get_audio_dir()?;
    let time_dir = audio_dir.join(time_folder);

    if !time_dir.exists() {
        return Err(format!("Time audio directory not found: {:?}", time_dir));
    }

    let mut files = get_audio_files(&time_dir)?;
    
    let mut played = match time_folder {
        "morning" => PLAYED_MORNING.lock().map_err(|e| e.to_string())?,
        "noon" => PLAYED_NOON.lock().map_err(|e| e.to_string())?,
        "afternoon" => PLAYED_AFTERNOON.lock().map_err(|e| e.to_string())?,
        "evening" => PLAYED_EVENING.lock().map_err(|e| e.to_string())?,
        _ => PLAYED_MORNING.lock().map_err(|e| e.to_string())?,
    };

    let selected = select_unplayed(&mut files, &mut played)?;

    let relative_path = selected
        .strip_prefix(&audio_dir)
        .map_err(|_| "Failed to get relative path".to_string())?
        .to_string_lossy()
        .replace("\\", "/");

    play_audio(relative_path)
}