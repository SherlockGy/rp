//! 项目创建命令
//!
//! 核心功能：调用 cargo new 创建项目，然后打开 VS Code。

use std::fs;
use std::process::Command;

use chrono::Local;

use crate::config::{get_next_index, get_project_type_dir, ProjectType};

/// 创建新的练习项目
///
/// # 参数
/// - `name`: 可选的项目名称，为 None 时自动生成
/// - `keep`: 是否为正式项目（true = keep 目录，false = temp 目录）
/// - `git`: 是否初始化 git 仓库
///
/// # 流程
/// 1. 确定目标目录（temp 或 keep）
/// 2. 生成项目名称（自动编号或使用用户指定名称）
/// 3. 调用 cargo new 创建项目
/// 4. 启动 VS Code 并定位到 main.rs
pub fn create_project(name: Option<String>, keep: bool, git: bool) {
    // 根据 keep 参数决定项目类型
    let project_type = if keep {
        ProjectType::Keep
    } else {
        ProjectType::Temp
    };

    let target_dir = get_project_type_dir(project_type);

    // 确保目标目录存在
    if let Err(e) = fs::create_dir_all(&target_dir) {
        eprintln!("无法创建目录 {}: {}", target_dir.display(), e);
        return;
    }

    // 生成项目名称
    let project_name = generate_project_name(name, &target_dir, keep);
    let project_path = target_dir.join(&project_name);

    // 调用 cargo new 创建项目
    if !run_cargo_new(&project_path, git) {
        return;
    }

    println!("✓ 项目已创建: {}", project_path.display());

    // 打开 VS Code
    open_vscode(&project_path);
}

/// 生成项目名称
///
/// # 命名规则
/// - 临时项目：pXXX_用户名称 或 pXXX_时间戳
/// - 正式项目：直接使用用户名称（必须提供）
fn generate_project_name(
    name: Option<String>,
    target_dir: &std::path::Path,
    keep: bool,
) -> String {
    match name {
        Some(n) if keep => {
            // 正式项目直接使用用户提供的名称
            n
        }
        Some(n) => {
            // 临时项目添加编号前缀
            let index = get_next_index(target_dir);
            format!("p{:03}_{}", index, n)
        }
        None => {
            // 无名称时使用时间戳（仅限临时项目）
            let index = get_next_index(target_dir);
            let timestamp = Local::now().format("%Y%m%d_%H%M%S");
            format!("p{:03}_{}", index, timestamp)
        }
    }
}

/// 执行 cargo new 命令
///
/// # 参数
/// - `project_path`: 项目完整路径
/// - `git`: 是否初始化 git
///
/// # 返回
/// 创建是否成功
fn run_cargo_new(project_path: &std::path::Path, git: bool) -> bool {
    // 构建命令参数
    let path_str = project_path.to_string_lossy();
    let mut args: Vec<&str> = vec!["new", &path_str];

    // 默认不初始化 git（与 cargo 默认行为相反）
    if !git {
        args.extend(["--vcs", "none"]);
    }

    // 执行命令
    let status = Command::new("cargo")
        .args(&args)
        .status()
        .expect("无法执行 cargo 命令，请确保 Rust 已正确安装");

    if !status.success() {
        eprintln!("cargo new 执行失败，请检查项目名称是否合法");
        return false;
    }

    true
}

/// 打开 VS Code 并定位到 main.rs
///
/// # 参数
/// - `project_path`: 项目根目录路径
fn open_vscode(project_path: &std::path::Path) {
    let main_rs = project_path.join("src").join("main.rs");

    // 使用 --goto 参数直接定位到文件第一行
    let result = Command::new("code")
        .args([
            project_path.to_string_lossy().as_ref(),
            "--goto",
            &format!("{}:1", main_rs.display()),
        ])
        .status();

    match result {
        Ok(status) if status.success() => {
            println!("✓ VS Code 已打开");
        }
        _ => {
            // VS Code 未安装或不在 PATH 中
            println!("提示: 无法自动打开 VS Code，请手动打开项目目录");
        }
    }
}
