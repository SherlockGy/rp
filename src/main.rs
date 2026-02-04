//! # rp - Rust Playground
//!
//! 快速创建 Rust 练习项目的命令行工具。
//!
//! ## 功能特点
//! - 一键创建项目并打开 VS Code
//! - 支持临时练习和正式项目两种模式
//! - 自动编号，便于管理
//! - 可批量清理临时练习
//!
//! ## 使用示例
//! ```bash
//! rp              # 创建临时练习
//! rp hello        # 创建名为 hello 的临时练习
//! rp -k project   # 创建正式项目
//! rp -l           # 列出所有项目
//! rp -c           # 清理临时练习
//! ```

mod cli;
mod config;
mod commands;

use clap::Parser;

use cli::Cli;
use commands::{clean_temp, create_project, list_projects};

/// 程序入口
///
/// 解析命令行参数并分发到对应的处理函数。
fn main() {
    // 解析命令行参数
    let cli = Cli::parse();

    // 根据参数执行对应命令
    // 优先级：list > clean > create（默认）
    if cli.list {
        list_projects();
    } else if cli.clean {
        clean_temp();
    } else {
        create_project(cli.name, cli.keep, cli.git);
    }
}
