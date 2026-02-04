//! 命令行参数定义模块
//!
//! 使用 clap 的 derive 宏来声明式地定义命令行接口。
//! 这是 clap 4.x 推荐的现代写法。

use clap::Parser;

/// rp - Rust Playground 快速练习工具
///
/// 帮助 Rust 学习者快速创建练习项目，减少重复的项目初始化工作。
/// 支持临时练习和正式项目两种模式。
#[derive(Parser, Debug)]
#[command(
    name = "rp",
    author,
    version,
    about = "Rust Playground - 快速创建练习项目的工具",
    long_about = "帮助 Rust 学习者快速创建练习项目。\n\n\
                  临时练习会自动编号，可随时清理；\n\
                  正式项目保存在 keep 目录，长期保留。"
)]
pub struct Cli {
    /// 项目名称
    ///
    /// 可选参数。如果不指定，临时练习会自动生成 pXXX_时间戳 格式的名称。
    /// 正式项目必须指定名称。
    pub name: Option<String>,

    /// 保存为正式项目
    ///
    /// 正式项目存放在 ~/rust-playground/keep/ 目录，
    /// 不会被 --clean 命令清理。
    #[arg(short, long)]
    pub keep: bool,

    /// 列出所有练习项目
    ///
    /// 分别显示临时练习和正式项目列表。
    #[arg(short, long)]
    pub list: bool,

    /// 清理临时练习目录
    ///
    /// 删除 ~/rust-playground/temp/ 下的所有项目。
    /// 正式项目不受影响。
    #[arg(short, long)]
    pub clean: bool,

    /// 初始化 git 仓库
    ///
    /// 默认不初始化 git（与 cargo new 相反）。
    /// 添加此选项后会调用 git init。
    #[arg(short, long)]
    pub git: bool,

    /// 打开已有项目
    ///
    /// 显示项目列表供选择，输出编辑器打开命令。
    /// 支持模糊搜索项目名称。
    #[arg(short, long)]
    pub open: bool,
}
