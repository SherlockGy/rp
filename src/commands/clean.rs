//! 清理命令
//!
//! 删除所有临时练习项目，释放磁盘空间。

use std::fs;

use crate::config::{get_project_type_dir, ProjectType};

/// 清理临时练习目录
///
/// 删除 temp 目录下的所有项目。
/// keep 目录（正式项目）不受影响。
///
/// # 注意
/// 此操作不可逆，请确保临时项目中没有需要保留的内容。
pub fn clean_temp() {
    let temp_dir = get_project_type_dir(ProjectType::Temp);

    // 检查目录是否存在
    if !temp_dir.exists() {
        println!("临时目录不存在，无需清理");
        return;
    }

    // 统计项目数量
    let count = count_projects(&temp_dir);

    if count == 0 {
        println!("临时目录已为空");
        return;
    }

    println!("即将删除 {} 个临时项目...", count);

    // 删除整个 temp 目录
    if let Err(e) = fs::remove_dir_all(&temp_dir) {
        eprintln!("删除失败: {}", e);
        eprintln!("提示: 可能有文件被占用，请关闭相关编辑器后重试");
        return;
    }

    // 重新创建空的 temp 目录
    if let Err(e) = fs::create_dir_all(&temp_dir) {
        eprintln!("警告: 无法重新创建 temp 目录: {}", e);
    }

    println!("✓ 清理完成");
}

/// 统计目录中的项目数量
///
/// # 参数
/// - `dir`: 要统计的目录路径
///
/// # 返回
/// 目录中的条目数量（包括文件和子目录）
fn count_projects(dir: &std::path::Path) -> usize {
    fs::read_dir(dir)
        .map(|entries| entries.count())
        .unwrap_or(0)
}
