[English](https://github.com/Somnia1337/csTimer-Analyzer/blob/main/README.md) | 中文

## [csTimer 分析器](https://github.com/Somnia1337/csTimer-Analyzer)

一个简单易用的 csTimer 数据分析器，用 Rust 编写。

[这里](https://raw.githubusercontent.com/Somnia1337/csTimer-Analyzer/main/Analysis.pdf) 是对我的真实数据进行分析产生的报告，作为示例。

功能特点：

- **快速**。能够在一秒内读取并分析数千条记录。
- **可配置**。分析器从 `options.txt` 文件读取配置，使得设置非常简单。
- **灵活**。分析结果生成一个 Markdown 文件，你可以使用自己喜欢的编辑器和样式来阅读报告。

### 使用方法

1. 访问 [csTimer](https://www.cstimer.net/)，点击“导出”按钮，然后点击“导出到文件”。一个名为 `cstimer_YYYYMMDD_hhmmss.txt` 的数据文件将会下载到本地。
2. 在 `options.txt` 中编写你的分析选项，配置教程见下文。
3. 将两个 `.txt` 文件和 `csTimer-Analyzer` 可执行文件放在同一个目录下，运行分析器，它会生成一个分析结果目录。

### 分析选项

分析器会从 `options.txt` 读取分析选项，以下是配置说明。

- `ObsidianFlavor(bool)`：启用或禁用 Obsidian 风格的 Markdown，支持类似 `![[image.png]]` 的 wiki 风格图片链接，和 `> [!error]` 这样的提示框。默认值为 `false`。
- `Overview`：提供分组的概览，包括最好和最差的时间、平均值、均值，和 `+2` 及 `DNF` 的次数统计。
- `stats`：统计的指标，从以下选项中选择一个：
  - `single`：单次成绩
  - `mo{n}`：`n` 次平均成绩
  - `ao{n}`：`n` 次去头去尾平均成绩
- `PbHistory(stats)`：指标 `stats` 的个人最佳成绩的历史记录。
- `Grouping(stats, millis)`：将指标 `stats` 以 `millis` 毫秒为间隔进行分组，生成直方图。
- `Trending(stats)`：跟踪指标 `stats` 的趋势，生成趋势图。
- `Commented`：筛选有注释的记录（如果你在跳 O / 跳 P 时写注释，这可能有帮助）。

下面有 `options.txt` 的实际示例，由它产生的分析报告（并用 Obsidian 导出为 PDF）在 [这里](https://raw.githubusercontent.com/Somnia1337/csTimer-Analyzer/main/Analysis.pdf)。

```text
# 注释以 '#' 开头

# 启用 Obsidian 风格的 markdown
ObsidianFlavor(true) # 只有 "true" 有效

# 分组概览
Overview

# 个人最佳成绩历史
PbHistory(single)
PbHistory(mo5)
PbHistory(ao50)

# 直方图
Grouping(single, 500)
Grouping(single, 1000)

# 趋势图
Trending(mo5)
Trending(ao200)

# 有注释的记录
Commented
```

### 未完成的

- 解析分组的更多元数据，例如分组别名。
- 更多分析选项。
- 分析性能优化。
- 添加测试。
