//! 配置和路径管理模块
//!
//! 集中管理所有与路径相关的逻辑，便于后续扩展（如支持自定义目录）。

use std::path::{Path, PathBuf};
use std::fs;

/// 项目类型枚举
///
/// 区分临时练习和正式项目，影响存储位置和清理策略。
#[derive(Debug, Clone, Copy)]
pub enum ProjectType {
    /// 临时练习 - 存放在 temp 目录，可被清理
    Temp,
    /// 正式项目 - 存放在 keep 目录，永久保留
    Keep,
}

impl ProjectType {
    /// 获取对应的子目录名称
    pub fn dir_name(&self) -> &'static str {
        match self {
            Self::Temp => "temp",
            Self::Keep => "keep",
        }
    }
}

/// 获取 playground 根目录
///
/// 默认为用户主目录下的 rust-playground 文件夹。
/// 例如：C:\Users\xxx\rust-playground 或 /home/xxx/rust-playground
pub fn get_playground_dir() -> PathBuf {
    dirs::home_dir()
        .expect("无法获取用户主目录，请检查系统环境")
        .join("rust-playground")
}

/// 获取指定类型的项目目录
///
/// # 参数
/// - `project_type`: 项目类型（临时或正式）
///
/// # 返回
/// 完整的目录路径
pub fn get_project_type_dir(project_type: ProjectType) -> PathBuf {
    get_playground_dir().join(project_type.dir_name())
}

/// 获取下一个可用的项目编号
///
/// 扫描目录中现有的项目，找出最大编号并加 1。
/// 项目命名格式为 pXXX_xxx，其中 XXX 是三位数编号。
///
/// # 参数
/// - `dir`: 要扫描的目录路径
///
/// # 返回
/// 下一个可用的编号（从 1 开始）
pub fn get_next_index(dir: &Path) -> u32 {
    // 目录不存在时返回 1
    if !dir.exists() {
        return 1;
    }

    // 扫描目录，提取所有符合 pXXX 格式的编号
    let max_index = fs::read_dir(dir)
        .into_iter()
        .flatten()  // 处理 Result
        .flatten()  // 处理每个 DirEntry 的 Result
        .filter_map(|entry| {
            let name = entry.file_name();
            let name = name.to_string_lossy();

            // 检查是否以 'p' 开头且长度足够
            if name.starts_with('p') && name.len() >= 4 {
                // 尝试解析编号部分（p 后面的 3 个字符）
                name[1..4].parse::<u32>().ok()
            } else {
                None
            }
        })
        .max()
        .unwrap_or(0);

    max_index + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_project_type_dir_name() {
        assert_eq!(ProjectType::Temp.dir_name(), "temp");
        assert_eq!(ProjectType::Keep.dir_name(), "keep");
    }
}
