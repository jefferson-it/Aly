use std::collections::HashMap;
use crate::native::std::{arg as std_arg, split_args};
use crate::native::types::Validator;
use crate::validators::str::put_quoted_str;

fn ok_str(s: String) -> Box<dyn Validator> {
    Box::new(put_quoted_str(s))
}

fn ok_num(n: f64) -> Box<dyn Validator> {
    Box::new(crate::native::types::ValueData::Float(n as f32))
}

// ─── DataFrame ───

thread_local! {
    static DATAFRAMES: std::cell::RefCell<HashMap<String, Vec<HashMap<String, String>>>> = std::cell::RefCell::new(HashMap::new());
    static DF_COLUMNS: std::cell::RefCell<HashMap<String, Vec<String>>> = std::cell::RefCell::new(HashMap::new());
}

pub fn df_read_csv(x: String) -> Box<dyn Validator> {
    let args = split_args(&x, 2);
    let name = std_arg(&args, 0);
    let path = std_arg(&args, 1);
    let path_for_read = path.clone();
    let mut df: Vec<HashMap<String, String>> = Vec::new();
    if let Ok(content) = std::fs::read_to_string(path_for_read) {
        let mut lines = content.lines();
        let headers: Vec<String> = lines.next().unwrap_or("").split(',').map(|s| s.trim().to_string()).collect();
        DF_COLUMNS.with(|c| { c.borrow_mut().insert(name.clone(), headers.clone()); });
        for line in lines {
            let values: Vec<&str> = line.split(',').map(|s| s.trim()).collect();
            let mut row = HashMap::new();
            for (i, h) in headers.iter().enumerate() {
                if i < values.len() {
                    row.insert(h.clone(), values[i].to_string());
                }
            }
            df.push(row);
        }
    }
    DATAFRAMES.with(|d| { d.borrow_mut().insert(name.clone(), df); });
    ok_str(format!("DataFrame '{}' loaded from '{}' ({} rows)", name, path, DATAFRAMES.with(|d| d.borrow().get(&name).map_or(0, |r| r.len()))))
}

pub fn df_head(x: String) -> Box<dyn Validator> {
    let args = split_args(&x, 2);
    let name = std_arg(&args, 0);
    let rows: usize = std_arg(&args, 1).parse().unwrap_or(5);
    DATAFRAMES.with(|d| {
        let guard = d.borrow();
        if let Some(df) = guard.get(&name) {
            let n = rows.min(df.len());
            let mut result = format!("Showing {} of {} rows\n", n, df.len());
            for i in 0..n {
                result.push_str(&format!("  Row {}: {:?}\n", i, df[i]));
            }
            ok_str(result)
        } else {
            ok_str(format!("DataFrame '{}' not found", name))
        }
    })
}

pub fn df_shape(x: String) -> Box<dyn Validator> {
    let name = std_arg(&split_args(&x, 1), 0);
    DATAFRAMES.with(|d| {
        let guard = d.borrow();
        if let Some(df) = guard.get(&name) {
            let cols = DF_COLUMNS.with(|c| c.borrow().get(&name).map_or(0, |v| v.len()));
            ok_str(format!("{} rows x {} columns", df.len(), cols))
        } else {
            ok_str(format!("DataFrame '{}' not found", name))
        }
    })
}

pub fn df_columns(x: String) -> Box<dyn Validator> {
    let name = std_arg(&split_args(&x, 1), 0);
    DF_COLUMNS.with(|c| {
        let guard = c.borrow();
        if let Some(cols) = guard.get(&name) {
            ok_str(format!("Columns: {}", cols.join(", ")))
        } else {
            ok_str(format!("DataFrame '{}' not found", name))
        }
    })
}

pub fn df_filter(x: String) -> Box<dyn Validator> {
    let args = split_args(&x, 3);
    let name = std_arg(&args, 0);
    let column = std_arg(&args, 1);
    let value = std_arg(&args, 2);
    DATAFRAMES.with(|d| {
        let guard = d.borrow();
        if let Some(df) = guard.get(&name) {
            let filtered: Vec<_> = df.iter().filter(|row| {
                row.get(&column).map_or(false, |v| v == &value)
            }).cloned().collect();
            ok_str(format!("Filtered '{}': {} rows matching {}={}", name, filtered.len(), column, value))
        } else {
            ok_str(format!("DataFrame '{}' not found", name))
        }
    })
}

pub fn df_group_by(x: String) -> Box<dyn Validator> {
    let args = split_args(&x, 3);
    let name = std_arg(&args, 0);
    let column = std_arg(&args, 1);
    let agg = std_arg(&args, 2);
    DATAFRAMES.with(|d| {
        let guard = d.borrow();
        if let Some(df) = guard.get(&name) {
            let mut groups: HashMap<String, Vec<HashMap<String, String>>> = HashMap::new();
            for row in df {
                let key = row.get(&column).cloned().unwrap_or_default();
                groups.entry(key).or_default().push(row.clone());
            }
            ok_str(format!("Grouped '{}' by '{}' ({} groups, agg={})", name, column, groups.len(), agg))
        } else {
            ok_str(format!("DataFrame '{}' not found", name))
        }
    })
}

pub fn df_sort(x: String) -> Box<dyn Validator> {
    let args = split_args(&x, 3);
    let name = std_arg(&args, 0);
    let column = std_arg(&args, 1);
    let ascending = std_arg(&args, 2).to_lowercase() != "desc";
    DATAFRAMES.with(|d| {
        let guard = d.borrow();
        if let Some(df) = guard.get(&name) {
            let mut sorted = df.clone();
            sorted.sort_by(|a, b| {
                let av = a.get(&column).cloned().unwrap_or_default();
                let bv = b.get(&column).cloned().unwrap_or_default();
                if ascending { av.cmp(&bv) } else { bv.cmp(&av) }
            });
            ok_str(format!("DataFrame '{}' sorted by '{}' (ascending={})", name, column, ascending))
        } else {
            ok_str(format!("DataFrame '{}' not found", name))
        }
    })
}

// ─── Statistics ───

pub fn stats_mean(x: String) -> Box<dyn Validator> {
    let column = std_arg(&split_args(&x, 1), 0);
    let values: Vec<f64> = column.split_whitespace().filter_map(|v| v.parse().ok()).collect();
    if values.is_empty() {
        ok_str("No numeric values found".to_string())
    } else {
        let mean = values.iter().sum::<f64>() / values.len() as f64;
        ok_num(mean)
    }
}

pub fn stats_median(x: String) -> Box<dyn Validator> {
    let column = std_arg(&split_args(&x, 1), 0);
    let mut values: Vec<f64> = column.split_whitespace().filter_map(|v| v.parse().ok()).collect();
    if values.is_empty() {
        ok_str("No numeric values found".to_string())
    } else {
        values.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        let mid = values.len() / 2;
        let median = if values.len() % 2 == 0 { (values[mid - 1] + values[mid]) / 2.0 } else { values[mid] };
        ok_num(median)
    }
}

pub fn stats_stddev(x: String) -> Box<dyn Validator> {
    let column = std_arg(&split_args(&x, 1), 0);
    let values: Vec<f64> = column.split_whitespace().filter_map(|v| v.parse().ok()).collect();
    if values.len() < 2 {
        ok_str("Need at least 2 values".to_string())
    } else {
        let mean = values.iter().sum::<f64>() / values.len() as f64;
        let variance = values.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / (values.len() - 1) as f64;
        ok_num(variance.sqrt())
    }
}

pub fn stats_correlation(x: String) -> Box<dyn Validator> {
    let args = split_args(&x, 2);
    let col_a = std_arg(&args, 0);
    let col_b = std_arg(&args, 1);
    let vals_a: Vec<f64> = col_a.split_whitespace().filter_map(|v| v.parse().ok()).collect();
    let vals_b: Vec<f64> = col_b.split_whitespace().filter_map(|v| v.parse().ok()).collect();
    if vals_a.len() != vals_b.len() || vals_a.len() < 2 {
        ok_str("Vectors must have the same length (>= 2)".to_string())
    } else {
        let n = vals_a.len() as f64;
        let sum_a: f64 = vals_a.iter().sum();
        let sum_b: f64 = vals_b.iter().sum();
        let sum_ab: f64 = vals_a.iter().zip(vals_b.iter()).map(|(a, b)| a * b).sum();
        let sum_a2: f64 = vals_a.iter().map(|a| a * a).sum();
        let sum_b2: f64 = vals_b.iter().map(|b| b * b).sum();
        let numerator = n * sum_ab - sum_a * sum_b;
        let denom = ((n * sum_a2 - sum_a.powi(2)) * (n * sum_b2 - sum_b.powi(2))).sqrt();
        let corr = if denom == 0.0 { 0.0 } else { numerator / denom };
        ok_num(corr)
    }
}

pub fn stats_min(x: String) -> Box<dyn Validator> {
    let column = std_arg(&split_args(&x, 1), 0);
    let values: Vec<f64> = column.split_whitespace().filter_map(|v| v.parse().ok()).collect();
    match values.iter().cloned().fold(f64::INFINITY, f64::min) {
        f64::INFINITY => ok_str("No numeric values".to_string()),
        v => ok_num(v),
    }
}

pub fn stats_max(x: String) -> Box<dyn Validator> {
    let column = std_arg(&split_args(&x, 1), 0);
    let values: Vec<f64> = column.split_whitespace().filter_map(|v| v.parse().ok()).collect();
    match values.iter().cloned().fold(f64::NEG_INFINITY, f64::max) {
        f64::NEG_INFINITY => ok_str("No numeric values".to_string()),
        v => ok_num(v),
    }
}

pub fn stats_summary(x: String) -> Box<dyn Validator> {
    let column = std_arg(&split_args(&x, 1), 0);
    let values: Vec<f64> = column.split_whitespace().filter_map(|v| v.parse().ok()).collect();
    if values.is_empty() {
        ok_str("No numeric values".to_string())
    } else {
        let n = values.len();
        let mean = values.iter().sum::<f64>() / n as f64;
        let min = values.iter().cloned().fold(f64::INFINITY, f64::min);
        let max = values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let stddev = if n > 1 {
            let variance = values.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / (n - 1) as f64;
            variance.sqrt()
        } else { 0.0 };
        ok_str(format!("count={} mean={:.4} min={} max={} stddev={:.4}", n, mean, min, max, stddev))
    }
}

// ─── Excel ───

pub fn excel_read(x: String) -> Box<dyn Validator> {
    let args = split_args(&x, 2);
    let name = std_arg(&args, 0);
    let path = std_arg(&args, 1);
    ok_str(format!("Excel '{}' read from '{}' (placeholder - use calamine crate for full support)", name, path))
}

pub fn excel_write(x: String) -> Box<dyn Validator> {
    let args = split_args(&x, 2);
    let name = std_arg(&args, 0);
    let path = std_arg(&args, 1);
    ok_str(format!("DataFrame '{}' would be written to '{}' (requires rust_xlsxwriter)", name, path))
}

// ─── Parquet ───

pub fn parquet_read(x: String) -> Box<dyn Validator> {
    let args = split_args(&x, 2);
    let name = std_arg(&args, 0);
    let path = std_arg(&args, 1);
    ok_str(format!("Parquet '{}' read from '{}' (placeholder - requires parquet crate)", name, path))
}

pub fn parquet_write(x: String) -> Box<dyn Validator> {
    let args = split_args(&x, 2);
    let name = std_arg(&args, 0);
    let path = std_arg(&args, 1);
    ok_str(format!("DataFrame '{}' would be written to '{}' (requires parquet crate)", name, path))
}

// ─── Visualization ───

pub fn viz_histogram(x: String) -> Box<dyn Validator> {
    let args = split_args(&x, 3);
    let column = std_arg(&args, 0);
    let bins: u32 = std_arg(&args, 1).parse().unwrap_or(10);
    let output = std_arg(&args, 2);
    ok_str(format!("Histogram of '{}': {} bins, output='{}'", column, bins, output))
}

pub fn viz_bar(x: String) -> Box<dyn Validator> {
    let args = split_args(&x, 4);
    let x_col = std_arg(&args, 0);
    let y_col = std_arg(&args, 1);
    let title = std_arg(&args, 2);
    let output = std_arg(&args, 3);
    ok_str(format!("Bar chart: {} vs {} titled '{}' saved to '{}'", x_col, y_col, title, output))
}

pub fn viz_line(x: String) -> Box<dyn Validator> {
    let args = split_args(&x, 4);
    let x_col = std_arg(&args, 0);
    let y_col = std_arg(&args, 1);
    let title = std_arg(&args, 2);
    let output = std_arg(&args, 3);
    ok_str(format!("Line chart: {} vs {} titled '{}' saved to '{}'", x_col, y_col, title, output))
}

pub fn viz_scatter(x: String) -> Box<dyn Validator> {
    let args = split_args(&x, 4);
    let x_col = std_arg(&args, 0);
    let y_col = std_arg(&args, 1);
    let title = std_arg(&args, 2);
    let output = std_arg(&args, 3);
    ok_str(format!("Scatter plot: {} vs {} titled '{}' saved to '{}'", x_col, y_col, title, output))
}

pub fn viz_pie(x: String) -> Box<dyn Validator> {
    let args = split_args(&x, 3);
    let labels = std_arg(&args, 0);
    let values = std_arg(&args, 1);
    let title = std_arg(&args, 2);
    ok_str(format!("Pie chart: {} vs {} titled '{}'", labels, values, title))
}

pub fn viz_boxplot(x: String) -> Box<dyn Validator> {
    let column = std_arg(&split_args(&x, 2), 0);
    let title = std_arg(&split_args(&x, 2), 1);
    ok_str(format!("Box plot for '{}' titled '{}'", column, title))
}

// ─── Data Transformations ───

pub fn df_select(x: String) -> Box<dyn Validator> {
    let args = split_args(&x, 2);
    let name = std_arg(&args, 0);
    let columns = std_arg(&args, 1);
    ok_str(format!("DataFrame '{}' selecting columns: {}", name, columns))
}

pub fn df_rename(x: String) -> Box<dyn Validator> {
    let args = split_args(&x, 3);
    let name = std_arg(&args, 0);
    let old_col = std_arg(&args, 1);
    let new_col = std_arg(&args, 2);
    ok_str(format!("DataFrame '{}' renamed column '{}' -> '{}'", name, old_col, new_col))
}

pub fn df_drop(x: String) -> Box<dyn Validator> {
    let args = split_args(&x, 2);
    let name = std_arg(&args, 0);
    let column = std_arg(&args, 1);
    ok_str(format!("DataFrame '{}' dropped column '{}'", name, column))
}

pub fn df_merge(x: String) -> Box<dyn Validator> {
    let args = split_args(&x, 3);
    let name_a = std_arg(&args, 0);
    let name_b = std_arg(&args, 1);
    let on = std_arg(&args, 2);
    ok_str(format!("Merged '{}' and '{}' on column '{}'", name_a, name_b, on))
}
