# rp

Rust 练习项目快速创建工具。

## 安装

```bash
cargo install --path .
```

或将 `target/release/rp.exe` 复制到 PATH 目录中。

## 使用示例

```bash
# 快速创建临时练习（自动命名）
rp

# 创建指定名称的临时练习
rp hello

# 创建正式项目（保存在 keep 目录，不会被清理）
rp -k ownership

# 创建正式项目并初始化 git
rp -k -g my-project

# 临时练习也可以初始化 git
rp -g test

# 列出所有项目
rp -l

# 清理所有临时练习
rp -c
```

## 目录结构

项目默认存放在 `~/rust-playground/`：

```
~/rust-playground/
├── temp/           # 临时练习
│   ├── p001_20260205_143022/
│   └── p002_hello/
└── keep/           # 正式项目
    └── ownership/
```

## 参数

| 参数 | 说明 |
|------|------|
| `[NAME]` | 项目名称，可选 |
| `-k, --keep` | 保存为正式项目 |
| `-g, --git` | 初始化 git 仓库 |
| `-l, --list` | 列出所有项目 |
| `-c, --clean` | 清理临时练习 |
