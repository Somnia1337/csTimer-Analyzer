use std::fs::File;
use std::io::{self, Write};
use std::time::{Duration, Instant};

use chrono::Local;

use crate::files::*;
use crate::session::*;
use crate::types::*;

/// Appends the data file path
/// and the parsed options.
fn append_analysis_info(
    file: &mut File,
    data_file_path: &str,
    session_count: usize,
    record_count: usize,
    options: &[Analyze],
) -> io::Result<()> {
    writeln!(file, "### Dataset\n")?;
    writeln!(
        file,
        "Parsed `{}` sessions (`{}` records) in `{}`.\n",
        session_count, record_count, data_file_path
    )?;

    writeln!(file, "### Analysis Options\n")?;
    writeln!(
        file,
        "Successfully parsed `{}` options (failures ignored and duplicates removed).\n",
        options.len(),
    )?;
    for a_type in options.iter() {
        writeln!(file, "- {}", a_type)?;
    }
    writeln!(file)?;

    append_page_breaker(file)?;

    Ok(())
}

/// Appends session as a title.
fn append_session_title(file: &mut File, session: &Session) -> io::Result<()> {
    writeln!(file, "### {}\n", session)
}

/// Appends an image link.
fn append_image_link(file: &mut File, image_name: String, ob_flavor: bool) -> io::Result<()> {
    if ob_flavor {
        writeln!(file, "![[{}]]\n", image_name)
    } else {
        writeln!(file, "![](images/{})\n", image_name)
    }
}

/// Appends a callout if Obsidian flavor is enabled,
/// or a quote if disabled.
fn append_message(
    file: &mut File,
    callout_type: &str,
    content: String,
    ob_flavor: bool,
) -> io::Result<()> {
    if ob_flavor {
        writeln!(
            file,
            r#"> [!{}]
> 
> {}
"#,
            callout_type, content
        )
    } else {
        writeln!(file, "> **{}**: {}", callout_type, content)
    }
}

/// Appends an analysis section.
fn append_section(
    file: &mut File,
    session: &Session,
    a_type: &Analyze,
    image_dir: &str,
    ob_flavor: bool,
) -> io::Result<()> {
    write!(file, "#### ")?;

    match a_type {
        Analyze::Overview => {
            let (best, worst, mean, average) = session.overview();
            let overview = format!(
                r#"| best | worst | mean | avg |
| --- | --- | --- | --- |
| `{}` | `{}` | `{}` | `{}` |"#,
                best, worst, mean, average,
            );

            let (ok, plus2, dnf) = session.solve_states();
            let total = session.records().len() as f64;
            let solve_states = format!(
                r#"| Ok | +2 | DNF |
| --- | --- | --- |
| `{}` | `{}` `({:.2}%)` | `{}` `({:.2}%)` |"#,
                ok,
                plus2,
                (plus2 * 100) as f64 / total,
                dnf,
                (dnf * 100) as f64 / total
            );

            writeln!(file, "Overview\n\n{}\n\n{}\n", overview, solve_states)
        }

        Analyze::PbHistory(stats_type) => {
            writeln!(file, "**{}** PB History\n", stats_type)?;

            if let Some(pairs) = session.pb_breakers(stats_type) {
                if !pairs.is_empty() {
                    let pb_history = pairs
                        .iter()
                        .map(|p| p.0.readable())
                        .collect::<Vec<_>>()
                        .join(" -> ");
                    writeln!(file, "```\n{}\n```\n", pb_history)?;

                    if matches!(stats_type, StatsType::Single) {
                        for pair in &pairs {
                            writeln!(file, "{}", pair.1)?;
                        }
                    }

                    Ok(())
                } else {
                    append_message(
                        file,
                        "error",
                        format!("NO PB HISTORIES OF **{}**.", stats_type),
                        ob_flavor,
                    )
                }
            } else {
                append_message(
                    file,
                    "error",
                    format!("NOT ENOUGH RECORDS FOR **{}** STATISTICS.", stats_type),
                    ob_flavor,
                )
            }
        }

        Analyze::Grouping(stats_type, interval) => {
            writeln!(
                file,
                "**{}** Grouping by {}s\n",
                stats_type,
                *interval as f32 / 1000.0
            )?;

            match session.save_group_by_interval(image_dir, *interval) {
                Some(image_name) => append_image_link(file, image_name, ob_flavor),
                None => append_message(
                    file,
                    "error",
                    format!("GOURPING BY {}s FAILED.", interval),
                    ob_flavor,
                ),
            }
        }

        Analyze::Trending(stats_type) => {
            writeln!(file, "**{}** Trending\n", stats_type)?;
            if let Some(data) = session.trend(stats_type) {
                let desc = stats_type.to_string();
                let image_name = format!("session_{}_trending_{}.png", session.id(), desc);
                let path = format!("{}/{}", image_dir, image_name);

                match session.draw_trending(data, &desc, &path) {
                    Ok(()) => {
                        append_image_link(&mut file.try_clone().unwrap(), image_name, ob_flavor)
                    }
                    Err(e) => append_message(
                        file,
                        "error",
                        format!("SAVING IMAGE FAILED: {}.", e),
                        ob_flavor,
                    ),
                }
            } else {
                append_message(
                    file,
                    "error",
                    format!("NOT ENOUGH RECORDS FOR **{}** STATISTICS.", stats_type),
                    ob_flavor,
                )
            }
        }

        Analyze::Commented => {
            writeln!(file, "Commented Records\n")?;

            let commented = session.commented_records();
            if !commented.is_empty() {
                for (i, r) in commented {
                    writeln!(file, "[{}] {}", i, r)?;
                }
            } else {
                append_message(
                    file,
                    "error",
                    String::from("NO COMMENTED RECORD."),
                    ob_flavor,
                )?;
            }

            Ok(())
        }
    }
}

/// Appends some debug info.
fn append_debug_info(file: &mut File, timings: Vec<Duration>) -> io::Result<()> {
    writeln!(file, "### Debug Info\n")?;

    for (i, t) in timings.iter().enumerate() {
        writeln!(file, "- Analyzing session {} took {:.2?}", i + 1, t)?;
    }

    Ok(())
}

/// Appends a HTML page breaker,
/// to break between sessions
/// when exported to a PDF.
fn append_page_breaker(file: &mut File) -> io::Result<()> {
    writeln!(file, "<div style=\"page-break-after: always\"></div>\n")
}

/// Analyzes each session, using parsed options.
pub fn analyze(
    sessions: &Vec<Session>,
    options: &[Analyze],
    data_file_path: &str,
    ob_flavor: bool,
) -> io::Result<()> {
    let date_time = Local::now().format("%y%m%d_%H%M%S").to_string();
    let dir = format!("csTimer-Analysis_{}", date_time);
    let mut file = create_dir_and_file(&dir)?;
    let image_dir = format!("{}/images", dir);

    let mut timings = Vec::new();

    append_analysis_info(
        &mut file,
        data_file_path,
        sessions.len(),
        sessions.iter().map(|s| s.records().len()).sum::<usize>(),
        options,
    )?;

    for session in sessions {
        let start = Instant::now();

        append_session_title(&mut file, session)?;

        for a_type in options {
            append_section(&mut file, session, a_type, &image_dir, ob_flavor)?;
        }

        append_page_breaker(&mut file)?;

        timings.push(start.elapsed());
    }

    append_debug_info(&mut file, timings)?;

    Ok(())
}
