// @agicore-protected — fill in your Rust logic; this file won't be overwritten
use serde::{Deserialize, Serialize};
use crate::db::DbPool;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportBusinessReportInput {
    pub business_id: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportBusinessReportOutput {
    pub file_path: String,
    pub success: i64,
}

#[tauri::command]
pub async fn export_business_report(
    input: ExportBusinessReportInput,
    _db: tauri::State<'_, DbPool>,
) -> Result<ExportBusinessReportOutput, String> {
    let conn = _db.lock().map_err(|e| e.to_string())?;
    let bid = &input.business_id;

    // 1. Query the business row
    let business: (String, String, Option<String>, Option<String>, String, Option<String>, Option<String>, Option<String>) = conn
        .query_row(
            "SELECT id, name, tagline, description, stage, category, market_niche, business_model \
             FROM businesses WHERE id = ?1",
            rusqlite::params![bid],
            |row| Ok((
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
                row.get(4)?,
                row.get(5)?,
                row.get(6)?,
                row.get(7)?,
            )),
        )
        .map_err(|_| "Business not found".to_string())?;

    let business_name = business.1.clone();

    // 2. Strategic position (first)
    let strat: Option<(Option<String>, Option<String>, Option<String>, Option<String>, Option<String>)> = conn
        .query_row(
            "SELECT target_market, value_proposition, primary_moat, competitors, unfair_advantages \
             FROM strategic_positions WHERE business_id = ?1 LIMIT 1",
            rusqlite::params![bid],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?, row.get(4)?)),
        )
        .ok();

    // 3. Operations audit (latest)
    let ops: Option<(i64, i64, i64, i64, i64, i64, i64, Option<String>)> = conn
        .query_row(
            "SELECT knowledge_bottleneck, quality_bottleneck, coordination_bottleneck, \
             communication_bottleneck, creative_bottleneck, decision_bottleneck, overall_score, audit_date \
             FROM operations_audits WHERE business_id = ?1 ORDER BY created_at DESC LIMIT 1",
            rusqlite::params![bid],
            |row| Ok((
                row.get(0)?, row.get(1)?, row.get(2)?,
                row.get(3)?, row.get(4)?, row.get(5)?,
                row.get(6)?, row.get(7)?,
            )),
        )
        .ok();

    // 4. Financial snapshots (all)
    let mut fin_stmt = conn
        .prepare("SELECT period, revenue, cac, ltv, ltv_cac_ratio, gross_margin_pct, mrr, cash_runway_months, recorded_at FROM financial_snapshots WHERE business_id = ?1 ORDER BY recorded_at")
        .map_err(|e| e.to_string())?;
    let fin_rows: Vec<(String, f64, f64, f64, f64, f64, f64, f64, Option<String>)> = fin_stmt
        .query_map(rusqlite::params![bid], |row| {
            Ok((
                row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?,
                row.get(4)?, row.get(5)?, row.get(6)?, row.get(7)?,
                row.get(8)?,
            ))
        })
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect();

    // 5. Sales contacts
    let mut sc_stmt = conn
        .prepare("SELECT name, company, stage, ltv_potential, source, next_action, notes FROM sales_contacts WHERE business_id = ?1 ORDER BY created_at")
        .map_err(|e| e.to_string())?;
    let sc_rows: Vec<(String, Option<String>, String, f64, Option<String>, Option<String>, Option<String>)> = sc_stmt
        .query_map(rusqlite::params![bid], |row| {
            Ok((
                row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?,
                row.get(4)?, row.get(5)?, row.get(6)?,
            ))
        })
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect();

    // 6. Marketing assets
    let mut ma_stmt = conn
        .prepare("SELECT asset_type, title, platform, published_at FROM marketing_assets WHERE business_id = ?1 ORDER BY published_at")
        .map_err(|e| e.to_string())?;
    let ma_rows: Vec<(String, String, Option<String>, Option<String>)> = ma_stmt
        .query_map(rusqlite::params![bid], |row| {
            Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?))
        })
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect();

    // 7. Content pipeline items
    let mut cp_stmt = conn
        .prepare("SELECT idea, status, platform FROM content_pipeline_items WHERE business_id = ?1 ORDER BY created_at")
        .map_err(|e| e.to_string())?;
    let cp_rows: Vec<(String, String, Option<String>)> = cp_stmt
        .query_map(rusqlite::params![bid], |row| {
            Ok((row.get(0)?, row.get(1)?, row.get(2)?))
        })
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect();

    // 8. Dollar audits
    let mut da_stmt = conn
        .prepare("SELECT activity_name, tier, hours_per_week, ai_transferable, transfer_status, notes FROM dollar_audits WHERE business_id = ?1 ORDER BY tier, activity_name")
        .map_err(|e| e.to_string())?;
    let da_rows: Vec<(String, String, f64, i64, String, Option<String>)> = da_stmt
        .query_map(rusqlite::params![bid], |row| {
            Ok((
                row.get(0)?, row.get(1)?, row.get(2)?,
                row.get(3)?, row.get(4)?, row.get(5)?,
            ))
        })
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect();

    // Build the Markdown report
    let mut md = String::new();

    md.push_str(&format!("# {}\n\n", business_name));
    if let Some(t) = &business.2 { md.push_str(&format!("_{}_\n\n", t)); }
    md.push_str(&format!("**Stage:** {}  \n", business.4));
    if let Some(c) = &business.5 { md.push_str(&format!("**Category:** {}  \n", c)); }
    if let Some(d) = &business.3 { md.push_str(&format!("\n{}\n", d)); }

    md.push_str("\n## Strategic Position\n\n");
    if let Some(s) = &strat {
        if let Some(v) = &s.0 { md.push_str(&format!("**Target Market:** {}  \n", v)); }
        if let Some(v) = &s.1 { md.push_str(&format!("**Value Proposition:** {}  \n", v)); }
        if let Some(v) = &s.2 { md.push_str(&format!("**Primary Moat:** {}  \n", v)); }
        if let Some(v) = &s.4 { md.push_str(&format!("**Unfair Advantages:** {}  \n", v)); }
        if let Some(v) = &s.3 { md.push_str(&format!("**Competitors:** {}  \n", v)); }
    } else {
        md.push_str("_No strategic position recorded._\n");
    }

    md.push_str("\n## Operations Health\n\n");
    if let Some(o) = &ops {
        md.push_str(&format!("**Overall Bottleneck Score:** {} / 30  \n", o.6));
        if let Some(d) = &o.7 { md.push_str(&format!("**Audit Date:** {}  \n", d)); }
        md.push_str(&format!(
            "\n| Area | Score |\n|------|-------|\n\
             | Knowledge | {} |\n\
             | Quality | {} |\n\
             | Coordination | {} |\n\
             | Communication | {} |\n\
             | Creative | {} |\n\
             | Decision | {} |\n",
            o.0, o.1, o.2, o.3, o.4, o.5
        ));
    } else {
        md.push_str("_No operations audit recorded._\n");
    }

    md.push_str("\n## Financial Snapshots\n\n");
    if fin_rows.is_empty() {
        md.push_str("_No financial snapshots recorded._\n");
    } else {
        md.push_str("| Period | Revenue | CAC | LTV | LTV:CAC | Margin% | MRR | Runway |\n");
        md.push_str("|--------|---------|-----|-----|---------|---------|-----|--------|\n");
        for f in &fin_rows {
            md.push_str(&format!(
                "| {} | ${:.0} | ${:.0} | ${:.0} | {:.1}x | {:.0}% | ${:.0} | {:.1} mo |\n",
                f.0, f.1, f.2, f.3, f.4, f.5, f.6, f.7
            ));
        }
    }

    md.push_str("\n## Sales Pipeline\n\n");
    if sc_rows.is_empty() {
        md.push_str("_No sales contacts recorded._\n");
    } else {
        md.push_str("| Name | Company | Stage | LTV Potential | Source |\n");
        md.push_str("|------|---------|-------|--------------|--------|\n");
        for sc in &sc_rows {
            md.push_str(&format!(
                "| {} | {} | {} | ${:.0} | {} |\n",
                sc.0,
                sc.1.as_deref().unwrap_or(""),
                sc.2,
                sc.3,
                sc.4.as_deref().unwrap_or(""),
            ));
        }
        md.push_str("\n### Notes\n\n");
        for sc in &sc_rows {
            if let Some(n) = &sc.6 {
                md.push_str(&format!("**{}:** {}  \n", sc.0, n));
            }
        }
    }

    md.push_str("\n## Marketing Assets\n\n");
    if ma_rows.is_empty() {
        md.push_str("_No marketing assets recorded._\n");
    } else {
        for ma in &ma_rows {
            md.push_str(&format!(
                "- [{}] **{}** — {} ({})\n",
                ma.0,
                ma.1,
                ma.2.as_deref().unwrap_or(""),
                ma.3.as_deref().unwrap_or(""),
            ));
        }
    }

    md.push_str("\n## Content Pipeline\n\n");
    if cp_rows.is_empty() {
        md.push_str("_No content pipeline items recorded._\n");
    } else {
        for cp in &cp_rows {
            md.push_str(&format!(
                "- [{}] {} — {}\n",
                cp.1,
                cp.0,
                cp.2.as_deref().unwrap_or(""),
            ));
        }
    }

    md.push_str("\n## $20/$200 Audit\n\n");
    if da_rows.is_empty() {
        md.push_str("_No dollar audit entries recorded._\n");
    } else {
        md.push_str("| Activity | Tier | Hrs/Wk | AI? | Status |\n");
        md.push_str("|----------|------|--------|-----|--------|\n");
        for da in &da_rows {
            md.push_str(&format!(
                "| {} | {} | {:.1} | {} | {} |\n",
                da.0,
                da.1,
                da.2,
                if da.3 == 1 { "Yes" } else { "No" },
                da.4,
            ));
        }
        md.push_str("\n### Notes\n\n");
        for da in &da_rows {
            if let Some(n) = &da.5 {
                md.push_str(&format!("**{}:** {}  \n", da.0, n));
            }
        }
    }

    // Determine output path
    let home = std::env::var("USERPROFILE")
        .unwrap_or_else(|_| std::env::var("HOME").unwrap_or_else(|_| ".".to_string()));

    let date_str = chrono::Local::now().format("%Y-%m-%d").to_string();
    let safe_name = business_name.replace(|c: char| !c.is_alphanumeric() && c != ' ', "")
        .replace(' ', "_");

    let mut dir = PathBuf::from(&home);
    dir.push("Documents");
    dir.push("NovaSyn MBA Reports");

    if let Err(e) = fs::create_dir_all(&dir) {
        return Ok(ExportBusinessReportOutput {
            file_path: String::new(),
            success: 0,
        });
    }

    let file_name = format!("{}_report_{}.md", safe_name, date_str);
    let file_path = dir.join(&file_name);

    match fs::write(&file_path, md.as_bytes()) {
        Ok(_) => Ok(ExportBusinessReportOutput {
            file_path: file_path.to_string_lossy().to_string(),
            success: 1,
        }),
        Err(_) => Ok(ExportBusinessReportOutput {
            file_path: String::new(),
            success: 0,
        }),
    }
}
