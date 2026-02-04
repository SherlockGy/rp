//! 项目列表命令
//!
//! 显示所有临时练习和正式项目。

use std::fs;
use std::path::Path;

use crate::config::{get_project_type_dir, ProjectType};

/// 列出所有练习项目
///
/// 分别显示 temp（临时）和 keep（正式）目录下的项目。
/// 项目按名称排序显示。
pub fn list_projects() {
    println!("=== 临时练习 (temp) ===");
    list_dir(&get_project_type_dir(ProjectType::Temp));

    println!("\n=== 正式项目 (keep) ===");
    list_dir(&get_project_type_dir(ProjectType::Keep));
}

/// 列出指定目录下的所有项目
///
/// # 参数
/// - `dir`: 要列出的目录路径
///
/// # 输出
/// - 如果目录存在且非空，逐行打印项目名称
/// - 如果目录不存在或为空，显示 "(空)"
fn list_dir(dir: &Path) {
    // 检查目录是否存在
    if !dir.exists() {
        println!("  (空)");
        return;
    }

    // 读取目录内容
    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(_) => {
            println!("  (无法读取)");
            return;
        }
    };

    // 收集并排序项目名称（只收集目录，忽略文件）
    let mut projects: Vec<_> = entries
        .flatten()
        .filter(|e| e.path().is_dir())  // 只保留目录
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .collect();

    if projects.is_empty() {
        println!("  (空)");
        return;
    }

    // 按名称排序
    projects.sort();

    // 打印每个项目
    for name in projects {
        println!("  {}", name);
    }
}
