# 电影文件导入工具

一个使用 Rust 编写的小型电影清单导入项目。通过命令行传入 `.txt` 文件，并将电影清单转换为 JSON 对象数组。每部电影包含光盘编号、年份、中文片名和英文文件名四个字段。

## 运行

```sh
cargo run -- convert DVDs.txt
```

也可以直接运行构建后的程序：

```sh
moive_importer convert DVDs.txt
```

生成的 `.json` 文件会保存在源文件所在目录，文件名与源文件相同。清单中的数字标题（例如 `1.`）用于设置后续电影的光盘编号。

例如，`movies.txt` 会生成 `movies.json`。使用 `moive_importer --help` 或 `moive_importer convert --help` 查看命令用法。文本中的空行会被忽略，每行首尾空格会被去除。
