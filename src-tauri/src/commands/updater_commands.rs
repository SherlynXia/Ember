use tauri::AppHandle;
use serde_json::Value;
use std::time::Duration;

/// 打开浏览器访问指定 URL
#[tauri::command]
pub async fn open_browser(url: String) -> Result<(), String> {
    let _ = tauri_plugin_opener::open_url(&url, None::<&str>);
    Ok(())
}

/// 手动检查更新（直接访问 GitHub API）
#[tauri::command]
pub async fn check_update(app: AppHandle) -> Result<String, String> {
    // GitHub API URL - 使用 releases 列表端点
    // 注意：私有仓库需要 GitHub Token 才能访问
    let url = "https://api.github.com/repos/SherlynXia/Ember/releases";
    
    // 使用 tauri-plugin-http，添加超时设置
    let client = tauri_plugin_http::reqwest::Client::builder()
        .timeout(Duration::from_secs(30))
        .build()
        .map_err(|e| format!("创建 HTTP 客户端失败: {}", e))?;
    
    // 重试机制
    let mut last_error = String::new();
    for attempt in 1..=3 {
        println!("尝试请求 GitHub API (第 {} 次)...", attempt);
        
        match client
            .get(url)
            .header("User-Agent", "Ember-App/1.1.0")
            .header("Accept", "application/vnd.github.v3+json")
            .send()
            .await
        {
            Ok(response) => {
                let status = response.status();
                println!("GitHub API 响应状态: {}", status);
                
                if status.is_success() {
                    // 使用 text() 获取响应体，然后手动解析 JSON
                    match response.text().await {
                        Ok(text) => {
                            println!("GitHub API 响应文本长度: {}", text.len());
                            
                            // 手动解析 JSON
                            match serde_json::from_str::<Value>(&text) {
                                Ok(json) => {
                                    // /releases 返回的是数组，取第一个（最新的）
                                    if let Some(releases) = json.as_array() {
                                        if let Some(latest_release) = releases.first() {
                                            // 获取最新版本号
                                            if let Some(tag_name) = latest_release.get("tag_name").and_then(|v| v.as_str()) {
                                                // 获取当前版本号
                                                let current_version = app.package_info().version.to_string();
                                                
                                                // 比较版本号
                                                let latest_version = tag_name.trim_start_matches('v');
                                                let current = current_version.trim_start_matches('v');
                                                
                                                if latest_version != current {
                                                    return Ok(format!("发现新版本: {} (当前版本: {})", tag_name, current_version));
                                                } else {
                                                    return Ok(format!("当前已是最新版本 ({})", current_version));
                                                }
                                            } else {
                                                return Err("无法解析版本号".to_string());
                                            }
                                        } else {
                                            return Err("没有发布版本".to_string());
                                        }
                                    } else {
                                        return Err("无法解析发布列表".to_string());
                                    }
                                }
                                Err(e) => {
                                    return Err(format!("解析 JSON 失败: {}", e));
                                }
                            }
                        }
                        Err(e) => {
                            return Err(format!("获取响应文本失败: {}", e));
                        }
                    }
                } else if status.as_u16() == 404 {
                    // 404 错误 - 仓库可能不存在或是私有的
                    return Err("无法访问 GitHub 仓库。可能原因：\n1. 仓库不存在\n2. 仓库是私有的\n3. 网络问题\n\n请访问 https://github.com/SherlynXia/Ember/releases 手动检查更新".to_string());
                } else if status.as_u16() == 403 {
                    // 403 错误 - API 速率限制
                    return Err("GitHub API 请求次数过多，请稍后再试。\n\n请访问 https://github.com/SherlynXia/Ember/releases 手动检查更新".to_string());
                } else {
                    // 获取错误响应体
                    let error_body = response.text().await.unwrap_or_default();
                    println!("GitHub API 错误响应: {}", error_body);
                    last_error = format!("GitHub API 返回错误: {} - {}", status, error_body);
                }
            }
            Err(e) => {
                println!("请求失败 (第 {} 次): {}", attempt, e);
                last_error = format!("请求失败: {}", e);
                
                // 等待后重试
                if attempt < 3 {
                    std::thread::sleep(Duration::from_secs(2));
                }
            }
        }
    }
    
    // 如果 API 请求失败，提供备用方案
    println!("API 请求失败，建议手动检查更新");
    Err("检查更新失败。\n\n请访问 https://github.com/SherlynXia/Ember/releases 手动检查更新".to_string())
}

/// 下载并安装更新
#[tauri::command]
pub async fn download_and_install_update(_app: AppHandle) -> Result<(), String> {
    // 获取最新版本信息
    let url = "https://api.github.com/repos/SherlynXia/Ember/releases";
    
    let client = tauri_plugin_http::reqwest::Client::builder()
        .timeout(Duration::from_secs(30))
        .build()
        .map_err(|e| format!("创建 HTTP 客户端失败: {}", e))?;
    
    match client
        .get(url)
        .header("User-Agent", "Ember-App/1.1.0")
        .header("Accept", "application/vnd.github.v3+json")
        .send()
        .await
    {
        Ok(response) => {
            if response.status().is_success() {
                match response.text().await {
                    Ok(text) => {
                        match serde_json::from_str::<Value>(&text) {
                            Ok(json) => {
                                // /releases 返回的是数组，取第一个（最新的）
                                if let Some(releases) = json.as_array() {
                                    if let Some(latest_release) = releases.first() {
                                        // 获取下载链接
                                        if let Some(assets) = latest_release.get("assets").and_then(|v| v.as_array()) {
                                            // 查找 Windows 安装包
                                            let windows_asset = assets.iter().find(|asset| {
                                                if let Some(name) = asset.get("name").and_then(|v| v.as_str()) {
                                                    name.ends_with(".msi") || name.ends_with(".exe")
                                                } else {
                                                    false
                                                }
                                            });
                                            
                                            if let Some(asset) = windows_asset {
                                                if let Some(download_url) = asset.get("browser_download_url").and_then(|v| v.as_str()) {
                                                    // 使用 opener 插件打开下载链接
                                                    let _ = tauri_plugin_opener::open_url(download_url, None::<&str>);
                                                    Ok(())
                                                } else {
                                                    Err("无法获取下载链接".to_string())
                                                }
                                            } else {
                                                Err("未找到 Windows 安装包".to_string())
                                            }
                                        } else {
                                            Err("无法解析资源列表".to_string())
                                        }
                                    } else {
                                        Err("没有发布版本".to_string())
                                    }
                                } else {
                                    Err("无法解析发布列表".to_string())
                                }
                            }
                            Err(e) => {
                                Err(format!("解析 JSON 失败: {}", e))
                            }
                        }
                    }
                    Err(e) => {
                        Err(format!("获取响应文本失败: {}", e))
                    }
                }
            } else {
                Err(format!("GitHub API 返回错误: {}", response.status()))
            }
        }
        Err(e) => {
            Err(format!("请求失败: {}", e))
        }
    }
}