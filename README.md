English | [中文](https://github.com/Somnia1337/csTimer-Analyzer/blob/main/README-ZH.md)

## [csTimer-Analyzer](https://github.com/Somnia1337/csTimer-Analyzer)

An easy-to-use csTimer data analyzer built in Rust.

[Here](https://github.com/Somnia1337/csTimer-Analyzer/blob/main/Analysis.pdf) is the analysis report generated from my real data, as a showcase.

Features:

- **FAST**. It can read and analyze thousands of records in under a second.
- **CONFIGURABLE**. The analyzer reads your configuration from `options.txt`, making it simple to set up.
- **FLEXIBLE**. It generates a markdown document as the analysis report, so you can read it with your favorite editor and styling.

### Usage

1. Go to [csTimer](https://www.cstimer.net/), click on the "EXPORT" button, then click "Export to file". A data file named `cstimer_YYYYMMDD_hhmmss.txt` should be downloaded.
2. Write your analysis options in `options.txt`, see below for configuration tutorials.
3. Put the two `.txt` files and `csTimer-Analyzer` executable in the same directory, run the analyzer, and an analysis directory should be generated.

### Analysis Options

The analyzer reads your options from `options.txt`, here's how to write it.

- `ObsidianFlavor(bool)`: Enable or disable Obsidian-flavored markdown, which includes wiki-style image links like `![[image.png]]`, and callouts such as `> [!error]`. This is `false` by default.
- `Overview`: Provides an overview of a session, including best and worst times, mean and average, and counts of `+2`s and `DNF`s.
- `stats`: The metric of statistics, choose one of these:
  - `single`: single solves
  - `mo{n}`: mean of `n` solves
  - `ao{n}`: average of `n` solves
- `PbHistory(stats)`: Tracks your personal best history for `stats` over time.
- `Grouping(stats, millis)`: Groups `stats` into intervals of `millis` milliseconds, producing a histogram.
- `Trending(stats)`: Tracks the trend of `stats`, generating a trend chart.

Check below for a real example of `options.txt`, the analysis report generated from it (and exported to PDF with Obsidian) is [here](https://github.com/Somnia1337/csTimer-Analyzer/blob/main/Analysis.pdf).

```text
# Comment starts with '#'

# Enable Obsidian flavored markdown
ObsidianFlavor(true) # only "true" works

# Stats overview
Overview

# Pb histories
PbHistory(single)
PbHistory(mo5)
PbHistory(ao50)

# Grouping histograms
Grouping(single, 500)
Grouping(single, 1000)

# Trending charts
Trending(mo5)
Trending(ao200)
```

### TODOs

- Parse more session metadata, such as alias.
- More analysis options.
- Improve analyze performance.
- Add tests.
