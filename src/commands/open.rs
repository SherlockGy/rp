//! 打开项目命令
//!
//! 列出所有项目供用户选择，输出编辑器打开命令。

use std::fs;
use std::io::{self, Write};

use crate::config::{get_project_type_dir, ProjectType};
use crate::commands::create::print_editor_commands;

/// 项目信息
struct ProjectInfo {
    /// 项目名称
    name: String,
    /// 项目类型（temp/keep）
    project_type: ProjectType,
}

/// 打开已有项目
///
/// 显示所有项目列表，用户输入编号后输出编辑器打开命令。
/// 支持输入项目名称进行模糊匹配。
///
/// # 参数
/// - `filter`: 可选的过滤关键字，用于筛选项目
pub fn open_project(filter: Option<String>) {
    // 收集所有项目
    let mut projects = collect_projects();

    if projects.is_empty() {
        println!("没有找到任何项目");
        println!("提示: 使用 rp 创建第一个练习项目");
        return;
    }

    // 如果提供了过滤关键字，进行筛选
    if let Some(ref keyword) = filter {
        projects.retain(|p| p.name.contains(keyword));
        if projects.is_empty() {
            println!("没有找到匹配 \"{}\" 的项目", keyword);
            return;
        }
    }

    // 显示项目列表
    println!("选择要打开的项目:\n");
    for (i, project) in projects.iter().enumerate() {
        let type_tag = match project.project_type {
            ProjectType::Temp => "temp",
            ProjectType::Keep => "keep",
        };
        println!("  [{:2}] [{}] {}", i + 1, type_tag, project.name);
    }

    // 读取用户输入
    print!("\n输入编号 (1-{}): ", projects.len());
    io::stdout().flush().ok();

    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_err() {
        println!("读取输入失败");
        return;
    }

    // 解析用户输入
    let input = input.trim();
    if input.is_empty() {
        return;
    }

    let index: usize = match input.parse::<usize>() {
        Ok(n) if n >= 1 && n <= projects.len() => n - 1,
        _ => {
            println!("无效的编号");
            return;
        }
    };

    // 获取选中的项目并输出编辑器命令
    let selected = &projects[index];
    let project_path = get_project_type_dir(selected.project_type).join(&selected.name);

    println!("\n已选择: {}", selected.name);
    print_editor_commands(&project_path);
}

/// 收集所有项目
///
/// 扫描 temp 和 keep 目录，返回项目列表。
fn collect_projects() -> Vec<ProjectInfo> {
    let mut projects = Vec::new();

    // 收集临时项目
    collect_from_dir(ProjectType::Temp, &mut projects);

    // 收集正式项目
    collect_from_dir(ProjectType::Keep, &mut projects);

    projects
}

/// 从指定目录收集项目
fn collect_from_dir(project_type: ProjectType, projects: &mut Vec<ProjectInfo>) {
    let dir = get_project_type_dir(project_type);

    if !dir.exists() {
        return;
    }

    if let Ok(entries) = fs::read_dir(&dir) {
        let mut names: Vec<_> = entries
            .flatten()
            .filter(|e| e.path().is_dir())
            .map(|e| e.file_name().to_string_lossy().into_owned())
            .collect();

        // 按名称排序
        names.sort();

        for name in names {
            projects.push(ProjectInfo { name, project_type });
        }
    }
}
