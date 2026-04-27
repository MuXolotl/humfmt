use serde::Deserialize;
use std::collections::BTreeMap;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

const BYTES_VALUES_PER_ITER: f64 = 8.0;
const NUMBERS_VALUES_PER_ITER: f64 = 10.0;

#[derive(Debug, Deserialize)]
struct Estimates {
    median: Estimate,
}

#[derive(Debug, Deserialize)]
struct Estimate {
    // For Criterion's default WallTime measurement, values are in nanoseconds:
    // Measurement::to_f64 returns val.as_nanos() as f64 and the machine unit is "ns".
    point_estimate: f64,
}

fn main() -> io::Result<()> {
    let repo_root = repo_root()?;
    let criterion_root = repo_root.join("tools/benchmarks/target/criterion");

    let medians = load_medians(&criterion_root)?;

    let groups = vec![
        BenchGroup {
            title: "Bytes (allocating, to_string)",
            unit_label: "time per value (approx)",
            entries: vec![
                BenchEntry::bytes("humfmt", BenchKey::new("bytes/allocating", "humfmt/to_string")),
                BenchEntry::bytes(
                    "bytesize",
                    BenchKey::new("bytes/allocating", "bytesize/display_si/to_string"),
                ),
                BenchEntry::bytes(
                    "byte-unit",
                    BenchKey::new("bytes/allocating", "byte_unit/alt_format/#.2"),
                ),
                BenchEntry::bytes(
                    "prettier-bytes",
                    BenchKey::new("bytes/allocating", "prettier_bytes/to_string"),
                ),
            ],
        },
        BenchGroup {
            title: "Bytes (reused buffer, write! into String)",
            unit_label: "time per value (approx)",
            entries: vec![
                BenchEntry::bytes(
                    "humfmt",
                    BenchKey::new("bytes/reused_buffer", "humfmt/write").with_param("32"),
                ),
                BenchEntry::bytes(
                    "bytesize",
                    BenchKey::new("bytes/reused_buffer", "bytesize/write_display_si")
                        .with_param("32"),
                ),
                BenchEntry::bytes(
                    "byte-unit",
                    BenchKey::new("bytes/reused_buffer", "byte_unit/write_alt_format/#.2")
                        .with_param("32"),
                ),
                BenchEntry::bytes(
                    "prettier-bytes",
                    BenchKey::new("bytes/reused_buffer", "prettier_bytes/write").with_param("32"),
                ),
            ],
        },
        BenchGroup {
            title: "Numbers (allocating, to_string / format)",
            unit_label: "time per value (approx)",
            entries: vec![
                BenchEntry::numbers(
                    "humfmt",
                    BenchKey::new("numbers/allocating", "humfmt/to_string"),
                ),
                BenchEntry::numbers(
                    "human_format",
                    BenchKey::new("numbers/allocating", "human_format/Formatter::format"),
                ),
            ],
        },
    ];

    let markdown = render_markdown(&medians, &groups);
    fs::write(repo_root.join("BENCHMARKS.md"), markdown)?;

    let assets_dir = repo_root.join("assets/benchmarks");
    fs::create_dir_all(&assets_dir)?;

    write_svg_dark(
        &assets_dir,
        "bytes_allocating_dark.svg",
        "Bytes (allocating) — lower is better",
        &medians,
        &[
            (
                "prettier-bytes",
                BenchKey::new("bytes/allocating", "prettier_bytes/to_string"),
                BYTES_VALUES_PER_ITER,
            ),
            (
                "humfmt",
                BenchKey::new("bytes/allocating", "humfmt/to_string"),
                BYTES_VALUES_PER_ITER,
            ),
            (
                "bytesize",
                BenchKey::new("bytes/allocating", "bytesize/display_si/to_string"),
                BYTES_VALUES_PER_ITER,
            ),
            (
                "byte-unit",
                BenchKey::new("bytes/allocating", "byte_unit/alt_format/#.2"),
                BYTES_VALUES_PER_ITER,
            ),
        ],
    )?;

    write_svg_dark(
        &assets_dir,
        "bytes_reused_buffer_dark.svg",
        "Bytes (reused buffer) — lower is better",
        &medians,
        &[
            (
                "prettier-bytes",
                BenchKey::new("bytes/reused_buffer", "prettier_bytes/write").with_param("32"),
                BYTES_VALUES_PER_ITER,
            ),
            (
                "humfmt",
                BenchKey::new("bytes/reused_buffer", "humfmt/write").with_param("32"),
                BYTES_VALUES_PER_ITER,
            ),
            (
                "bytesize",
                BenchKey::new("bytes/reused_buffer", "bytesize/write_display_si")
                    .with_param("32"),
                BYTES_VALUES_PER_ITER,
            ),
            (
                "byte-unit",
                BenchKey::new("bytes/reused_buffer", "byte_unit/write_alt_format/#.2")
                    .with_param("32"),
                BYTES_VALUES_PER_ITER,
            ),
        ],
    )?;

    write_svg_dark(
        &assets_dir,
        "numbers_allocating_dark.svg",
        "Numbers (allocating) — lower is better",
        &medians,
        &[
            (
                "humfmt",
                BenchKey::new("numbers/allocating", "humfmt/to_string"),
                NUMBERS_VALUES_PER_ITER,
            ),
            (
                "human_format",
                BenchKey::new("numbers/allocating", "human_format/Formatter::format"),
                NUMBERS_VALUES_PER_ITER,
            ),
        ],
    )?;

    println!("Wrote BENCHMARKS.md and assets/benchmarks/*_dark.svg");
    Ok(())
}

fn repo_root() -> io::Result<PathBuf> {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let repo = manifest_dir
        .parent()
        .and_then(|p| p.parent())
        .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "failed to resolve repo root"))?;
    Ok(repo.to_path_buf())
}

fn load_medians(criterion_root: &Path) -> io::Result<BTreeMap<String, f64>> {
    let mut out = BTreeMap::new();

    if !criterion_root.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "criterion directory not found; run `cargo bench` first",
        ));
    }

    for entry in WalkDir::new(criterion_root)
        .follow_links(false)
        .into_iter()
        .filter_map(Result::ok)
    {
        if !entry.file_type().is_file() {
            continue;
        }
        if entry.file_name() != "estimates.json" {
            continue;
        }

        let path = entry.path();
        let id = match bench_id_from_estimates_path(criterion_root, path) {
            Some(id) => id,
            None => continue,
        };

        let json = fs::read_to_string(path)?;
        let estimates: Estimates = serde_json::from_str(&json).map_err(|e| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("failed to parse {path:?}: {e}"),
            )
        })?;

        out.insert(id, estimates.median.point_estimate);
    }

    Ok(out)
}

fn bench_id_from_estimates_path(criterion_root: &Path, path: &Path) -> Option<String> {
    let rel = path.strip_prefix(criterion_root).ok()?;
    let mut parts: Vec<&str> = rel
        .components()
        .filter_map(|c| c.as_os_str().to_str())
        .collect();

    if parts.len() < 3 {
        return None;
    }
    if parts[parts.len() - 1] != "estimates.json" {
        return None;
    }
    if parts[parts.len() - 2] != "new" {
        return None;
    }

    parts.truncate(parts.len() - 2);
    Some(parts.join("/"))
}

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

    fn expected_ids(&self) -> [String; 2] {
        [
            self.expected_id_with_case(CaseMode::Preserve),
            self.expected_id_with_case(CaseMode::Lower),
        ]
    }

    fn expected_id_with_case(&self, mode: CaseMode) -> String {
        let group = sanitize_for_criterion(self.group, mode);
        let bench = sanitize_for_criterion(self.bench, mode);
        match self.param {
            Some(p) => format!("{group}/{bench}/{p}"),
            None => format!("{group}/{bench}"),
        }
    }
}

#[derive(Copy, Clone)]
enum CaseMode {
    Preserve,
    Lower,
}

fn sanitize_for_criterion(input: &str, mode: CaseMode) -> String {
    let mut out = String::with_capacity(input.len());

    for mut ch in input.chars() {
        if matches!(mode, CaseMode::Lower) {
            ch = ch.to_ascii_lowercase();
        }

        let keep = ch.is_ascii_alphanumeric() || ch == '_' || ch == '-' || ch == '.' || ch == '#';
        if keep {
            out.push(ch);
        } else {
            out.push('_');
        }
    }

    out
}

#[derive(Clone)]
struct BenchGroup {
    title: &'static str,
    unit_label: &'static str,
    entries: Vec<BenchEntry>,
}

#[derive(Clone)]
struct BenchEntry {
    label: &'static str,
    key: BenchKey,
    per_iter_values: f64,
}

impl BenchEntry {
    fn bytes(label: &'static str, key: BenchKey) -> Self {
        Self {
            label,
            key,
            per_iter_values: BYTES_VALUES_PER_ITER,
        }
    }

    fn numbers(label: &'static str, key: BenchKey) -> Self {
        Self {
            label,
            key,
            per_iter_values: NUMBERS_VALUES_PER_ITER,
        }
    }
}

fn resolve_point_estimate(medians: &BTreeMap<String, f64>, key: BenchKey) -> Option<(String, f64)> {
    let candidates = key.expected_ids();

    // 1) Exact match.
    for expected in &candidates {
        if let Some(v) = medians.get(expected).copied() {
            return Some((expected.clone(), v));
        }
    }

    // 2) Suffix match (if Criterion adds extra prefixes).
    for expected in &candidates {
        let needle = format!("/{expected}");
        if let Some((k, v)) = medians
            .iter()
            .filter(|(k, _)| k.ends_with(&needle))
            .min_by_key(|(k, _)| k.len())
        {
            return Some((k.clone(), *v));
        }
    }

    None
}

fn env_true(name: &str) -> bool {
    match std::env::var(name) {
        Ok(v) => {
            let v = v.trim();
            v == "1" || v.eq_ignore_ascii_case("true") || v.eq_ignore_ascii_case("yes")
        }
        Err(_) => false,
    }
}

fn fmt_time_from_ns(ns: f64) -> String {
    if !ns.is_finite() {
        return "N/A".to_string();
    }

    if ns < 1_000.0 {
        format!("{ns:.0} ns")
    } else if ns < 1_000_000.0 {
        format!("{:.2} µs", ns / 1_000.0)
    } else if ns < 1_000_000_000.0 {
        format!("{:.2} ms", ns / 1_000_000.0)
    } else {
        format!("{:.2} s", ns / 1_000_000_000.0)
    }
}

fn render_markdown(medians: &BTreeMap<String, f64>, groups: &[BenchGroup]) -> String {
    let mut out = String::new();

    out.push_str("# Benchmarks\n\n");
    out.push_str("This report is generated from Criterion `median.point_estimate` values.\n\n");
    out.push_str("Regenerate locally:\n\n");
    out.push_str("```bash\n");
    out.push_str("cargo bench --manifest-path tools/benchmarks/Cargo.toml\n");
    out.push_str("cargo run --release --manifest-path tools/benchmarks/Cargo.toml --bin report\n");
    out.push_str("```\n\n");
    out.push_str("Notes:\n\n");
    out.push_str("- Results depend on machine / OS / CPU scaling.\n");
    out.push_str("- Some crates differ in formatting semantics (e.g. spaces between number and unit).\n\n");

    let mut had_missing = false;

    for g in groups {
        out.push_str("## ");
        out.push_str(g.title);
        out.push_str("\n\n");
        out.push_str("| Implementation | Median per-iteration | ");
        out.push_str(g.unit_label);
        out.push_str(" | Relative vs humfmt |\n");
        out.push_str("|---|---:|---:|---:|\n");

        let hum_entry = g.entries.iter().find(|e| e.label == "humfmt");
        let hum_ns_value = hum_entry.and_then(|e| {
            resolve_point_estimate(medians, e.key).map(|(_, ns_iter)| ns_iter / e.per_iter_values)
        });

        for e in &g.entries {
            let resolved = resolve_point_estimate(medians, e.key);
            if resolved.is_none() {
                had_missing = true;
            }

            let median_ns_iter = resolved.as_ref().map(|(_, ns)| *ns);
            let median_ns_value = median_ns_iter.map(|ns| ns / e.per_iter_values);

            let rel = match (median_ns_value, hum_ns_value) {
                (Some(v), Some(h)) if h > 0.0 => Some(v / h),
                _ => None,
            };

            out.push_str("| ");
            out.push_str(e.label);
            out.push_str(" | ");
            out.push_str(
                &median_ns_iter
                    .map(fmt_time_from_ns)
                    .unwrap_or_else(|| "N/A".to_string()),
            );
            out.push_str(" | ");
            out.push_str(
                &median_ns_value
                    .map(fmt_time_from_ns)
                    .unwrap_or_else(|| "N/A".to_string()),
            );
            out.push_str(" | ");
            out.push_str(&rel.map(|x| format!("{x:.2}×")).unwrap_or_else(|| "N/A".to_string()));
            out.push_str(" |\n");
        }

        out.push('\n');
    }

    if had_missing || env_true("HUMFMT_BENCH_INCLUDE_IDS") {
        out.push_str("<details>\n<summary>Discovered Criterion benchmark IDs</summary>\n\n");
        out.push_str("```text\n");
        for key in medians.keys() {
            out.push_str(key);
            out.push('\n');
        }
        out.push_str("```\n\n</details>\n");
    }

    out
}

fn write_svg_dark(
    assets_dir: &Path,
    filename: &str,
    title: &str,
    medians: &BTreeMap<String, f64>,
    items: &[(&str, BenchKey, f64)],
) -> io::Result<()> {
    let svg = render_svg_dark(title, medians, items);
    fs::write(assets_dir.join(filename), svg)?;
    Ok(())
}

fn render_svg_dark(
    title: &str,
    medians: &BTreeMap<String, f64>,
    items: &[(&str, BenchKey, f64)],
) -> String {
    let bg = "#0d1117";
    let fg = "#c9d1d9";
    let grid = "#30363d";
    let bar = "#58a6ff";
    let bar_humfmt = "#d2a8ff";

    let mut rows: Vec<(&str, Option<f64>)> = Vec::new();
    let mut max_ns: f64 = 0.0;

    for (label, key, per_iter_values) in items {
        let resolved = resolve_point_estimate(medians, *key);
        let ns_per_value = resolved.map(|(_, ns_iter)| ns_iter / *per_iter_values);

        if let Some(v) = ns_per_value {
            max_ns = max_ns.max(v);
        }
        rows.push((*label, ns_per_value));
    }

    let width: i32 = 960;
    let height: i32 = 260;
    let left: i32 = 240;
    let top: i32 = 54;
    let bar_w: i32 = 560;
    let row_h: i32 = 36;
    let value_x: i32 = width - 20;

    let mut svg = String::new();
    svg.push_str(&format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="{width}" height="{height}" viewBox="0 0 {width} {height}">"#
    ));
    svg.push_str(&format!(r#"<rect width="100%" height="100%" fill="{bg}"/>"#));
    svg.push_str(&format!(
        r#"<text x="20" y="30" fill="{fg}" font-family="ui-sans-serif, system-ui" font-size="18" font-weight="600">{}</text>"#,
        escape(title)
    ));
    svg.push_str(&format!(
        r#"<line x1="{left}" y1="{top}" x2="{left}" y2="{y2}" stroke="{grid}" stroke-width="1"/>"#,
        y2 = top + row_h * (rows.len() as i32) + 10
    ));

    for (i, (label, ns_opt)) in rows.iter().enumerate() {
        let y = top + (i as i32) * row_h;

        let w = match ns_opt {
            Some(ns) if max_ns > 0.0 => ((bar_w as f64) * (ns / max_ns)).round() as i32,
            _ => 0,
        };

        let color = if *label == "humfmt" { bar_humfmt } else { bar };

        svg.push_str(&format!(
            r#"<text x="20" y="{ytext}" fill="{fg}" font-family="ui-sans-serif, system-ui" font-size="14">{label}</text>"#,
            ytext = y + 22,
            label = escape(label),
        ));
        svg.push_str(&format!(
            r#"<rect x="{left}" y="{ybar}" width="{w}" height="18" fill="{color}" rx="4"/>"#,
            ybar = y + 8,
        ));

        match ns_opt {
            Some(ns) => svg.push_str(&format!(
                r#"<text x="{value_x}" y="{ytext}" fill="{fg}" font-family="ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas" font-size="12" text-anchor="end">{}</text>"#,
                fmt_time_from_ns(*ns),
                ytext = y + 22
            )),
            None => svg.push_str(&format!(
                r#"<text x="{value_x}" y="{ytext}" fill="{fg}" font-family="ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas" font-size="12" text-anchor="end">N/A</text>"#,
                ytext = y + 22
            )),
        };
    }

    svg.push_str("</svg>");
    svg
}

fn escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}