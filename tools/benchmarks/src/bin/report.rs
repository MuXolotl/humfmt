//! Benchmark report generator.
//!
//! Reads Criterion `estimates.json` files, produces `BENCHMARKS.md`
//! (capability matrix + sorted, bold-best tables) and SVG chart files under
//! `assets/benchmarks/`.
//!
//! SVG design:
//!   - Horizontal bar chart, fastest (lowest ns) at the top.
//!   - X-axis with 5 tick marks (0 / 25 / 50 / 75 / 100 % of max).
//!   - Tick labels showing actual ns values.
//!   - humfmt bars use a distinct accent color.

use serde::Deserialize;
use std::collections::BTreeMap;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

// ---------------------------------------------------------------------------
// Per-iteration value counts (for ns-per-value normalization)
// ---------------------------------------------------------------------------

const BYTES_U64_PER_ITER: f64 = 8.0;
const BYTES_U128_PER_ITER: f64 = 4.0;
const BYTES_NEG_I64_PER_ITER: f64 = 4.0;

const NUMBERS_PER_ITER: f64 = 10.0;

const DURATION_PER_ITER: f64 = 8.0;
const AGO_PER_ITER: f64 = 8.0;

// ---------------------------------------------------------------------------
// Criterion JSON structures
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
struct Estimates {
    median: Estimate,
}

#[derive(Debug, Deserialize)]
struct Estimate {
    /// Wall-time in nanoseconds (Criterion default measurement unit).
    point_estimate: f64,
}

// ---------------------------------------------------------------------------
// Entry point
// ---------------------------------------------------------------------------

fn main() -> io::Result<()> {
    let repo_root = repo_root()?;
    let criterion_root = repo_root.join("tools/benchmarks/target/criterion");
    let medians = load_medians(&criterion_root)?;

    // ---- Markdown ----

    let md = build_markdown(&medians);
    fs::write(repo_root.join("BENCHMARKS.md"), md)?;

    // ---- SVG charts ----

    let assets = repo_root.join("assets/benchmarks");
    fs::create_dir_all(&assets)?;

    // Combined SVG for bytes (allocating + reused buffer sections).
    write_combined_svg(
        &assets,
        "bytes_comparison_dark.svg",
        &[
            SvgSection {
                title: "Bytes — allocating (to_string) — lower is better",
                items: bytes_alloc_items(),
            },
            SvgSection {
                title: "Bytes — reused buffer (write!) — lower is better",
                items: bytes_reuse_items(),
            },
        ],
        &medians,
    )?;

    // Combined SVG for time-related benchmarks (duration + ago).
    write_combined_svg(
        &assets,
        "time_comparison_dark.svg",
        &[
            SvgSection {
                title: "Duration formatting — allocating — lower is better",
                items: duration_items(),
            },
            SvgSection {
                title: "Relative time (ago) — allocating — lower is better",
                items: ago_items(),
            },
        ],
        &medians,
    )?;

    // Standalone SVG for numbers.
    write_standalone_svg(
        &assets,
        "numbers_dark.svg",
        "Numbers — allocating (to_string) — lower is better",
        &numbers_items(),
        &medians,
    )?;

    println!("Wrote BENCHMARKS.md + 3 SVGs under assets/benchmarks/");
    Ok(())
}

// ---------------------------------------------------------------------------
// Item lists for each benchmark category
// ---------------------------------------------------------------------------

fn bytes_alloc_items() -> Vec<SvgItem> {
    vec![
        SvgItem::humfmt(
            "humfmt  i8-u128, any precision",
            BenchKey::new("bytes/allocating", "humfmt/u64/to_string"),
            BYTES_U64_PER_ITER,
        ),
        SvgItem::other(
            "bytesize  u64 only",
            BenchKey::new("bytes/allocating", "bytesize/u64/display_si/to_string"),
            BYTES_U64_PER_ITER,
        ),
        SvgItem::other(
            "byte-unit  u64/u128",
            BenchKey::new("bytes/allocating", "byte_unit/u64/alt_format/#.2"),
            BYTES_U64_PER_ITER,
        ),
        SvgItem::other(
            "prettier-bytes  u64 only, fixed 2dp, no negatives",
            BenchKey::new("bytes/allocating", "prettier_bytes/u64/to_string"),
            BYTES_U64_PER_ITER,
        ),
    ]
}

fn bytes_reuse_items() -> Vec<SvgItem> {
    vec![
        SvgItem::humfmt(
            "humfmt  i8-u128, any precision",
            BenchKey::new("bytes/reused_buffer", "humfmt/u64/write").with_param("32"),
            BYTES_U64_PER_ITER,
        ),
        SvgItem::other(
            "bytesize  u64 only",
            BenchKey::new("bytes/reused_buffer", "bytesize/u64/write_display_si").with_param("32"),
            BYTES_U64_PER_ITER,
        ),
        SvgItem::other(
            "byte-unit  u64/u128",
            BenchKey::new("bytes/reused_buffer", "byte_unit/u64/write_alt_format/#.2")
                .with_param("32"),
            BYTES_U64_PER_ITER,
        ),
        SvgItem::other(
            "prettier-bytes  u64 only, fixed 2dp, no negatives",
            BenchKey::new("bytes/reused_buffer", "prettier_bytes/u64/write").with_param("32"),
            BYTES_U64_PER_ITER,
        ),
    ]
}

fn duration_items() -> Vec<SvgItem> {
    vec![
        SvgItem::humfmt(
            "humfmt  short, 2 units (default)",
            BenchKey::new("duration/allocating", "humfmt/short/default"),
            DURATION_PER_ITER,
        ),
        SvgItem::humfmt(
            "humfmt  short, 3 units",
            BenchKey::new("duration/allocating", "humfmt/short/max3"),
            DURATION_PER_ITER,
        ),
        SvgItem::humfmt(
            "humfmt  long labels, 2 units",
            BenchKey::new("duration/allocating", "humfmt/long/default"),
            DURATION_PER_ITER,
        ),
        SvgItem::other(
            "humantime  EN only, all non-zero units",
            BenchKey::new("duration/allocating", "humantime/default"),
            DURATION_PER_ITER,
        ),
    ]
}

fn ago_items() -> Vec<SvgItem> {
    vec![
        SvgItem::humfmt(
            "humfmt  short, 2 units (default)",
            BenchKey::new("ago/allocating", "humfmt/short/default"),
            AGO_PER_ITER,
        ),
        SvgItem::humfmt(
            "humfmt  short, 2 units (explicit)",
            BenchKey::new("ago/allocating", "humfmt/short/2_units"),
            AGO_PER_ITER,
        ),
        SvgItem::humfmt(
            "humfmt  long, 2 units",
            BenchKey::new("ago/allocating", "humfmt/long/2_units"),
            AGO_PER_ITER,
        ),
        SvgItem::other(
            "timeago  EN, 1 unit (default), returns String",
            BenchKey::new("ago/allocating", "timeago/default/1_unit"),
            AGO_PER_ITER,
        ),
        SvgItem::other(
            "timeago  EN, 2 units, returns String",
            BenchKey::new("ago/allocating", "timeago/2_units"),
            AGO_PER_ITER,
        ),
    ]
}

fn numbers_items() -> Vec<SvgItem> {
    vec![
        SvgItem::humfmt(
            "humfmt  i8-u128 + f32/f64, locale-aware",
            BenchKey::new("numbers/allocating", "humfmt/to_string"),
            NUMBERS_PER_ITER,
        ),
        SvgItem::other(
            "human_format  f64 only, EN only, returns String",
            BenchKey::new("numbers/allocating", "human_format/Formatter::format"),
            NUMBERS_PER_ITER,
        ),
    ]
}

// ---------------------------------------------------------------------------
// Markdown builder
// ---------------------------------------------------------------------------

fn build_markdown(medians: &BTreeMap<String, f64>) -> String {
    let mut out = String::new();

    out.push_str("# Benchmarks\n\n");
    out.push_str("This report is generated from Criterion `median.point_estimate` values.\n\n");
    out.push_str("Regenerate locally:\n\n```bash\n");
    out.push_str("cargo bench --manifest-path tools/benchmarks/Cargo.toml\n");
    out.push_str("cargo run --release --manifest-path tools/benchmarks/Cargo.toml --bin report\n");
    out.push_str("```\n\n");

    out.push_str("---\n\n");
    out.push_str("## Capability Matrix\n\n");
    out.push_str(CAPABILITY_MATRIX);
    out.push_str("\n\n---\n\n");

    out.push_str("## Notes\n\n");
    out.push_str("- Results depend on machine / OS / CPU scaling.\n");
    out.push_str("- **Bold** = best (lowest) value in column.\n");
    out.push_str("- Limitation tags are shown next to each crate name.\n");
    out.push_str("- Rows are sorted fastest to slowest within each group.\n");
    out.push_str(
        "- Duration semantics can differ between crates (e.g. full-unit rendering vs capped output).\n",
    );
    out.push_str(
        "- Some crates return an owned `String` by design; `humfmt` formatters implement `Display`.\n\n",
    );
    out.push_str("---\n\n");

    // Bytes (u64 comparison)
    push_md_group(
        &mut out,
        "Bytes — allocating (`to_string`), u64 inputs",
        Some(
            "> prettier-bytes and bytesize are **u64-only**. humfmt accepts i8-i128 and u8-u128.",
        ),
        "time per value",
        &bytes_alloc_items(),
        medians,
    );

    push_md_group(
        &mut out,
        "Bytes — reused buffer (`write!` into `String`), u64 inputs",
        None,
        "time per value",
        &bytes_reuse_items(),
        medians,
    );

    // Extended range (humfmt-only)
    out.push_str("## Bytes — extended range (u128 > u64::MAX) — humfmt only\n\n");
    out.push_str("> No other benchmarked crate handles values above `u64::MAX`.\n\n");
    out.push_str("| Scenario | Median per-iteration | Time per value |\n");
    out.push_str("|---|---:|---:|\n");
    {
        let key = BenchKey::new("bytes/allocating", "humfmt/u128_extended/to_string");
        out.push_str(&format_md_row_single(
            "humfmt/u128_extended",
            key,
            BYTES_U128_PER_ITER,
            medians,
        ));
    }
    out.push('\n');

    // Negative bytes (humfmt-only)
    out.push_str("## Bytes — negative values (i64) — humfmt only\n\n");
    out.push_str("> No other benchmarked crate in this harness supports signed byte inputs.\n\n");
    out.push_str("| Scenario | Median per-iteration | Time per value |\n");
    out.push_str("|---|---:|---:|\n");
    {
        let key = BenchKey::new("bytes/allocating", "humfmt/negative_i64/to_string");
        out.push_str(&format_md_row_single(
            "humfmt/negative_i64",
            key,
            BYTES_NEG_I64_PER_ITER,
            medians,
        ));
    }
    out.push('\n');

    // Numbers
    push_md_group(
        &mut out,
        "Numbers — allocating (`to_string`)",
        Some(
            "> human_format accepts f64 only and returns an owned `String`. humfmt accepts all integer and float primitives.",
        ),
        "time per value",
        &numbers_items(),
        medians,
    );

    // Duration
    push_md_group(
        &mut out,
        "Duration formatting — allocating",
        Some(
            "> humantime renders all non-zero units. humfmt caps at `max_units` (default 2). These produce different output for the same input.",
        ),
        "time per value",
        &duration_items(),
        medians,
    );

    // Ago
    push_md_group(
        &mut out,
        "Relative time — allocating",
        Some(
            "> timeago returns an owned `String` from `convert()`. humfmt implements `Display` and writes directly with no intermediate allocation.",
        ),
        "time per value",
        &ago_items(),
        medians,
    );

    if env_true("HUMFMT_BENCH_INCLUDE_IDS") {
        out.push_str("<details>\n<summary>All Criterion IDs found on disk</summary>\n\n```text\n");
        for k in medians.keys() {
            out.push_str(k);
            out.push('\n');
        }
        out.push_str("```\n\n</details>\n");
    }

    out
}

/// Appends one benchmark group table to `out`, sorted fastest-first.
fn push_md_group(
    out: &mut String,
    title: &str,
    note: Option<&str>,
    unit_col: &str,
    items: &[SvgItem],
    medians: &BTreeMap<String, f64>,
) {
    out.push_str("## ");
    out.push_str(title);
    out.push('\n');

    if let Some(n) = note {
        out.push('\n');
        out.push_str(n);
        out.push('\n');
    }

    out.push('\n');
    out.push_str("| Implementation | Median per-iteration | ");
    out.push_str(unit_col);
    out.push_str(" | Relative vs humfmt |\n");
    out.push_str("|---|---:|---:|---:|\n");

    // Resolve values.
    let mut rows: Vec<(&SvgItem, Option<f64>, Option<f64>)> = items
        .iter()
        .map(|item| {
            let ns_iter = resolve(medians, item.key).map(|(_, v)| v);
            let ns_value = ns_iter.map(|v| v / item.per_iter_values);
            (item, ns_iter, ns_value)
        })
        .collect();

    // Sort fastest-first; missing data goes last.
    rows.sort_by(|(_, _, a), (_, _, b)| cmp_opt_f64(*a, *b));

    // Find humfmt baseline for relative column.
    let hum_ns_value = rows
        .iter()
        .find(|(item, _, _)| item.is_humfmt)
        .and_then(|(_, _, v)| *v);

    // Find column-wise minimums for bold.
    let best_iter = min_idx(&rows.iter().map(|(_, a, _)| *a).collect::<Vec<_>>());
    let best_value = min_idx(&rows.iter().map(|(_, _, b)| *b).collect::<Vec<_>>());

    for (i, (item, ns_iter, ns_value)) in rows.iter().enumerate() {
        let rel = match (ns_value, hum_ns_value) {
            (Some(v), Some(h)) if h > 0.0 => Some(v / h),
            _ => None,
        };

        let iter_str = ns_iter.map(fmt_ns).unwrap_or_else(|| "N/A".into());
        let val_str = ns_value.map(fmt_ns).unwrap_or_else(|| "N/A".into());
        let rel_str = rel.map(|x| format!("{x:.2}x")).unwrap_or_else(|| "N/A".into());

        let iter_cell = if best_iter == Some(i) {
            bold(&iter_str)
        } else {
            iter_str
        };
        let val_cell = if best_value == Some(i) {
            bold(&val_str)
        } else {
            val_str
        };

        out.push_str("| ");
        out.push_str(item.label);
        out.push_str(" | ");
        out.push_str(&iter_cell);
        out.push_str(" | ");
        out.push_str(&val_cell);
        out.push_str(" | ");
        out.push_str(&rel_str);
        out.push_str(" |\n");
    }

    out.push('\n');
}

fn format_md_row_single(
    label: &str,
    key: BenchKey,
    per_iter: f64,
    medians: &BTreeMap<String, f64>,
) -> String {
    let ns_iter = resolve(medians, key).map(|(_, v)| v);
    let ns_value = ns_iter.map(|v| v / per_iter);
    format!(
        "| {} | {} | {} |\n",
        label,
        ns_iter.map(fmt_ns).unwrap_or_else(|| "N/A".into()),
        ns_value.map(fmt_ns).unwrap_or_else(|| "N/A".into()),
    )
}

// ---------------------------------------------------------------------------
// Capability matrix (static markdown table)
// ---------------------------------------------------------------------------

const CAPABILITY_MATRIX: &str = "\
| Feature | humfmt | bytesize | byte-unit | prettier-bytes | humantime | timeago | human_format |
|---|:---:|:---:|:---:|:---:|:---:|:---:|:---:|
| Byte sizes | yes | yes | yes | yes | no | no | no |
| Compact numbers | yes | no | no | no | no | no | yes |
| Duration formatting | yes | no | no | no | yes | yes | no |
| Relative time (ago) | yes | no | no | no | no | yes | no |
| Ordinals | yes | no | no | no | no | no | no |
| List formatting | yes | no | no | no | no | no | no |
| Signed input (negatives) | yes | no | no | no | — | — | no |
| u128 / i128 range | yes | no | partial | no | — | — | no |
| Float input | yes | no | no | no | — | — | yes |
| Long-form labels | yes | no | yes | no | yes | yes | yes |
| Max-units cap | yes | — | — | — | no | yes | — |
| Binary (IEC) units | yes | yes | yes | yes | — | — | — |
| Configurable precision | yes | no | yes | no | — | — | yes |
| Locale-aware | yes | no | no | no | no | yes | no |
| Custom locale builder | yes | no | no | no | no | no | no |
| no_std compatible | yes | no | no | yes | no | no | no |
| Zero-alloc Display | yes | yes | no | yes | yes | no | no |";

// ---------------------------------------------------------------------------
// BenchKey
// ---------------------------------------------------------------------------

#[derive(Copy, Clone)]
struct BenchKey {
    group: &'static str,
    bench: &'static str,
    param: Option<&'static str>,
}

impl BenchKey {
    const fn new(group: &'static str, bench: &'static str) -> Self {
        Self {
            group,
            bench,
            param: None,
        }
    }

    const fn with_param(mut self, param: &'static str) -> Self {
        self.param = Some(param);
        self
    }

    fn candidates(&self) -> [String; 2] {
        [self.build(CaseMode::Preserve), self.build(CaseMode::Lower)]
    }

    fn build(&self, mode: CaseMode) -> String {
        let g = sanitize(self.group, mode);
        let b = sanitize(self.bench, mode);
        match self.param {
            Some(p) => format!("{g}/{b}/{p}"),
            None => format!("{g}/{b}"),
        }
    }
}

#[derive(Copy, Clone)]
enum CaseMode {
    Preserve,
    Lower,
}

fn sanitize(s: &str, mode: CaseMode) -> String {
    s.chars()
        .map(|mut c| {
            if matches!(mode, CaseMode::Lower) {
                c = c.to_ascii_lowercase();
            }
            if c.is_ascii_alphanumeric() || matches!(c, '_' | '-' | '.' | '#') {
                c
            } else {
                '_'
            }
        })
        .collect()
}

fn resolve(medians: &BTreeMap<String, f64>, key: BenchKey) -> Option<(String, f64)> {
    for id in key.candidates() {
        if let Some(&v) = medians.get(&id) {
            return Some((id, v));
        }
    }

    // Suffix fallback when Criterion prefixes the group path differently.
    for id in key.candidates() {
        let needle = format!("/{id}");
        if let Some((k, &v)) = medians
            .iter()
            .filter(|(k, _)| k.ends_with(&needle))
            .min_by_key(|(k, _)| k.len())
        {
            return Some((k.clone(), v));
        }
    }

    None
}

// ---------------------------------------------------------------------------
// SvgItem — describes one bar in a chart
// ---------------------------------------------------------------------------

struct SvgItem {
    /// Label shown to the left of the bar (may contain limitation notes).
    label: &'static str,
    key: BenchKey,
    per_iter_values: f64,
    /// Determines bar color: accent for humfmt, neutral for others.
    is_humfmt: bool,
}

impl SvgItem {
    fn humfmt(label: &'static str, key: BenchKey, per_iter_values: f64) -> Self {
        Self {
            label,
            key,
            per_iter_values,
            is_humfmt: true,
        }
    }

    fn other(label: &'static str, key: BenchKey, per_iter_values: f64) -> Self {
        Self {
            label,
            key,
            per_iter_values,
            is_humfmt: false,
        }
    }
}

// ---------------------------------------------------------------------------
// SVG generation
// ---------------------------------------------------------------------------

struct SvgSection {
    title: &'static str,
    items: Vec<SvgItem>,
}

/// Writes a multi-section SVG (one section per benchmark sub-group).
fn write_combined_svg(
    assets: &Path,
    filename: &str,
    sections: &[SvgSection],
    medians: &BTreeMap<String, f64>,
) -> io::Result<()> {
    let row_h = 36i32;
    let section_header = 50i32;
    let section_footer = 36i32;
    let gap = 28i32;
    let width = 1000i32;

    let total_h: i32 = sections
        .iter()
        .map(|s| section_header + (s.items.len() as i32) * row_h + section_footer)
        .sum::<i32>()
        + gap * (sections.len() as i32 - 1);

    let bg = "#0d1117";
    let mut svg = String::new();

    svg.push_str(&format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="{width}" height="{total_h}" viewBox="0 0 {width} {total_h}">"#
    ));
    svg.push_str(&format!(r#"<rect width="100%" height="100%" fill="{bg}"/>"#));

    let mut y_cursor = 0i32;
    for (i, section) in sections.iter().enumerate() {
        let section_h = section_header + (section.items.len() as i32) * row_h + section_footer;
        push_svg_section(&mut svg, section.title, &section.items, medians, y_cursor);

        y_cursor += section_h;

        if i + 1 < sections.len() {
            let divider_y = y_cursor + gap / 2;
            let x2 = width - 20;
            let stroke = "#30363d";
            svg.push_str(&format!(
                r#"<line x1="20" y1="{divider_y}" x2="{x2}" y2="{divider_y}" stroke="{stroke}" stroke-width="1" stroke-dasharray="5 4"/>"#
            ));
            y_cursor += gap;
        }
    }

    svg.push_str("</svg>");
    fs::write(assets.join(filename), svg)
}

/// Writes a single-section standalone SVG.
fn write_standalone_svg(
    assets: &Path,
    filename: &str,
    title: &str,
    items: &[SvgItem],
    medians: &BTreeMap<String, f64>,
) -> io::Result<()> {
    let row_h = 36i32;
    let header = 50i32;
    let footer = 36i32;
    let width = 1000i32;
    let total_h = header + (items.len() as i32) * row_h + footer;

    let bg = "#0d1117";
    let mut svg = String::new();

    svg.push_str(&format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="{width}" height="{total_h}" viewBox="0 0 {width} {total_h}">"#
    ));
    svg.push_str(&format!(r#"<rect width="100%" height="100%" fill="{bg}"/>"#));

    push_svg_section(&mut svg, title, items, medians, 0);

    svg.push_str("</svg>");
    fs::write(assets.join(filename), svg)
}

/// Renders one chart section at the given vertical offset.
///
/// Items are sorted fastest-first automatically.
fn push_svg_section(
    svg: &mut String,
    title: &str,
    items: &[SvgItem],
    medians: &BTreeMap<String, f64>,
    y_offset: i32,
) {
    let left = 330i32;
    let bar_max_w = 530i32;
    let row_h = 36i32;
    let header = 50i32;
    let value_x = left + bar_max_w + 8;

    let fg = "#c9d1d9";
    let grid_color = "#21262d";
    let axis_color = "#30363d";
    let tick_fg = "#8b949e";
    let bar_humfmt = "#a371f7";
    let bar_other = "#388bfd";
    let bar_missing = "#30363d";

    // Resolve and sort fastest-first.
    let mut rows: Vec<(&SvgItem, Option<f64>)> = items
        .iter()
        .map(|item| {
            let ns = resolve(medians, item.key).map(|(_, v)| v / item.per_iter_values);
            (item, ns)
        })
        .collect();
    rows.sort_by(|(_, a), (_, b)| cmp_opt_f64(*a, *b));

    let max_ns = rows
        .iter()
        .filter_map(|(_, v)| *v)
        .fold(0.0_f64, f64::max);

    let content_y = y_offset + header;
    let axis_y = content_y + (rows.len() as i32) * row_h;
    let bar_right = left + bar_max_w;

    // Section title
    let title_y = y_offset + 28;
    svg.push_str(&format!(
        r#"<text x="20" y="{title_y}" fill="{fg}" font-family="ui-sans-serif,system-ui,sans-serif" font-size="13" font-weight="600">{}</text>"#,
        escape(title)
    ));

    // Vertical axis line
    svg.push_str(&format!(
        r#"<line x1="{left}" y1="{content_y}" x2="{left}" y2="{axis_y}" stroke="{axis_color}" stroke-width="1"/>"#
    ));

    // Horizontal axis line
    svg.push_str(&format!(
        r#"<line x1="{left}" y1="{axis_y}" x2="{bar_right}" y2="{axis_y}" stroke="{axis_color}" stroke-width="1"/>"#
    ));

    // Grid lines + tick labels at 0, 25, 50, 75, 100 %
    for pct in [0u32, 25, 50, 75, 100] {
        let x = left + (bar_max_w as f64 * pct as f64 / 100.0).round() as i32;
        let ns_at_tick = max_ns * pct as f64 / 100.0;
        let tick_label = if pct == 0 { "0".into() } else { fmt_ns(ns_at_tick) };

        if pct > 0 {
            svg.push_str(&format!(
                r#"<line x1="{x}" y1="{content_y}" x2="{x}" y2="{axis_y}" stroke="{grid_color}" stroke-width="1" stroke-dasharray="2 3"/>"#
            ));
        }

        let tick_bottom = axis_y + 4;
        svg.push_str(&format!(
            r#"<line x1="{x}" y1="{axis_y}" x2="{x}" y2="{tick_bottom}" stroke="{axis_color}" stroke-width="1"/>"#
        ));

        let anchor = match pct {
            0 => "start",
            100 => "end",
            _ => "middle",
        };
        let tick_text_y = axis_y + 16;
        svg.push_str(&format!(
            r#"<text x="{x}" y="{tick_text_y}" fill="{tick_fg}" font-family="ui-monospace,monospace" font-size="10" text-anchor="{anchor}">{}</text>"#,
            escape(&tick_label)
        ));
    }

    // Bars
    for (i, (item, ns_opt)) in rows.iter().enumerate() {
        let y = content_y + (i as i32) * row_h;
        let bar_y = y + 7;
        let bar_h = 22i32;
        let text_y = y + 22;

        let bar_w = match ns_opt {
            Some(ns) if max_ns > 0.0 => ((bar_max_w as f64) * (ns / max_ns)).round() as i32,
            _ => 0,
        };

        let color = if ns_opt.is_none() {
            bar_missing
        } else if item.is_humfmt {
            bar_humfmt
        } else {
            bar_other
        };

        svg.push_str(&format!(
            r#"<rect x="{left}" y="{bar_y}" width="{bar_w}" height="{bar_h}" fill="{color}" rx="3"/>"#
        ));

        let label_x = left - 8;
        svg.push_str(&format!(
            r#"<text x="{label_x}" y="{text_y}" fill="{fg}" font-family="ui-sans-serif,system-ui,sans-serif" font-size="11" text-anchor="end">{}</text>"#,
            escape(item.label)
        ));

        let val_str = ns_opt.map(fmt_ns).unwrap_or_else(|| "N/A".into());
        svg.push_str(&format!(
            r#"<text x="{value_x}" y="{text_y}" fill="{fg}" font-family="ui-monospace,monospace" font-size="11">{}</text>"#,
            escape(&val_str)
        ));
    }
}

// ---------------------------------------------------------------------------
// File loading helpers
// ---------------------------------------------------------------------------

fn repo_root() -> io::Result<PathBuf> {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest_dir
        .parent()
        .and_then(|p| p.parent())
        .map(|p| p.to_path_buf())
        .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "failed to resolve repo root"))
}

fn load_medians(criterion_root: &Path) -> io::Result<BTreeMap<String, f64>> {
    let mut out = BTreeMap::new();

    if !criterion_root.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "criterion directory not found — run `cargo bench` first",
        ));
    }

    for entry in WalkDir::new(criterion_root)
        .follow_links(false)
        .into_iter()
        .filter_map(Result::ok)
    {
        if !entry.file_type().is_file() || entry.file_name() != "estimates.json" {
            continue;
        }

        let path = entry.path();
        let Some(id) = bench_id_from_path(criterion_root, path) else {
            continue;
        };

        let json = fs::read_to_string(path)?;
        let est: Estimates = serde_json::from_str(&json).map_err(|e| {
            io::Error::new(io::ErrorKind::InvalidData, format!("{path:?}: {e}"))
        })?;

        out.insert(id, est.median.point_estimate);
    }

    Ok(out)
}

fn bench_id_from_path(root: &Path, path: &Path) -> Option<String> {
    let rel = path.strip_prefix(root).ok()?;
    let mut parts: Vec<&str> = rel
        .components()
        .filter_map(|c| c.as_os_str().to_str())
        .collect();

    if parts.len() < 3 || *parts.last()? != "estimates.json" || parts[parts.len() - 2] != "new" {
        return None;
    }

    parts.truncate(parts.len() - 2);
    Some(parts.join("/"))
}

// ---------------------------------------------------------------------------
// Formatting / sorting utilities
// ---------------------------------------------------------------------------

fn fmt_ns(ns: f64) -> String {
    if !ns.is_finite() {
        return "N/A".into();
    }
    if ns < 1_000.0 {
        format!("{ns:.0} ns")
    } else if ns < 1_000_000.0 {
        format!("{:.2} us", ns / 1_000.0)
    } else if ns < 1_000_000_000.0 {
        format!("{:.2} ms", ns / 1_000_000.0)
    } else {
        format!("{:.2} s", ns / 1_000_000_000.0)
    }
}

fn cmp_opt_f64(a: Option<f64>, b: Option<f64>) -> std::cmp::Ordering {
    match (a, b) {
        (Some(x), Some(y)) => x.partial_cmp(&y).unwrap_or(std::cmp::Ordering::Equal),
        (Some(_), None) => std::cmp::Ordering::Less,
        (None, Some(_)) => std::cmp::Ordering::Greater,
        (None, None) => std::cmp::Ordering::Equal,
    }
}

fn min_idx(values: &[Option<f64>]) -> Option<usize> {
    values
        .iter()
        .enumerate()
        .filter_map(|(i, v)| v.map(|x| (i, x)))
        .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
        .map(|(i, _)| i)
}

fn bold(s: &str) -> String {
    format!("**{s}**")
}

fn env_true(name: &str) -> bool {
    matches!(
        std::env::var(name).as_deref().map(str::trim),
        Ok("1") | Ok("true") | Ok("yes")
    )
}

fn escape(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;")
}