// @agicore-protected — fill in your Rust logic; this file won't be overwritten
use serde::{Deserialize, Serialize};
use crate::db::DbPool;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeneratePitchDeckOutlineInput {
    pub business_id: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GeneratePitchDeckOutlineOutput {
    pub file_path: String,
    pub success: i64,
}

#[tauri::command]
pub async fn generate_pitch_deck_outline(
    input: GeneratePitchDeckOutlineInput,
    _db: tauri::State<'_, DbPool>,
) -> Result<GeneratePitchDeckOutlineOutput, String> {
    let conn = _db.lock().map_err(|e| e.to_string())?;
    let bid = &input.business_id;

    // 1. Query the business
    let business: (String, Option<String>, Option<String>, Option<String>, Option<String>) = conn
        .query_row(
            "SELECT name, market_niche, business_model, tagline, description \
             FROM businesses WHERE id = ?1",
            rusqlite::params![bid],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?, row.get(4)?)),
        )
        .map_err(|_| "Business not found".to_string())?;

    let business_name = &business.0;
    let market_niche = business.1.as_deref().unwrap_or("[market niche not set]");
    let business_model = business.2.as_deref().unwrap_or("[business model not set]");

    // 2. Query first strategic position
    let strat: Option<(Option<String>, Option<String>, Option<String>, Option<String>, Option<String>)> = conn
        .query_row(
            "SELECT target_market, value_proposition, primary_moat, unfair_advantages, competitors \
             FROM strategic_positions WHERE business_id = ?1 LIMIT 1",
            rusqlite::params![bid],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?, row.get(4)?)),
        )
        .ok();

    let target_market = strat.as_ref()
        .and_then(|s| s.0.as_deref())
        .unwrap_or("[target market not set]");
    let value_proposition = strat.as_ref()
        .and_then(|s| s.1.as_deref())
        .unwrap_or("[value proposition not set]");
    let primary_moat = strat.as_ref()
        .and_then(|s| s.2.as_deref())
        .unwrap_or("[moat not set]");
    let unfair_advantages = strat.as_ref()
        .and_then(|s| s.3.as_deref())
        .unwrap_or("[unfair advantages not set]");

    // 3. Build the Markdown pitch outline
    let mut md = String::new();

    md.push_str(&format!("# {} — Pitch Deck Outline\n\n", business_name));

    md.push_str("## 1. Problem\n\n");
    md.push_str(&format!(
        "The market we serve: **{}**\n\n\
         Target audience: **{}**\n\n\
         _(Describe the specific pain points this audience faces. What breaks down for them today? What does it cost them in time, money, or opportunity?)_\n\n",
        market_niche, target_market
    ));

    md.push_str("## 2. Solution\n\n");
    md.push_str(&format!(
        "{}\n\n\
         _(Expand on how the solution eliminates the problem described above. Keep it concrete — what does a client's life look like after working with you?)_\n\n",
        value_proposition
    ));

    md.push_str("## 3. Why Now\n\n");
    md.push_str("[FOUNDER TO COMPLETE]\n\n");
    md.push_str("_(What has changed in the market, technology, or regulation that makes this the right moment? Why would this not have worked 3 years ago, and why is waiting 3 more years a mistake?)_\n\n");

    md.push_str("## 4. Why Us\n\n");
    md.push_str(&format!(
        "**Primary Moat:** {}\n\n\
         **Unfair Advantages:**\n{}\n\n\
         _(Translate the above into a narrative: why is this team or founder the one who will win this market?)_\n\n",
        primary_moat, unfair_advantages
    ));

    md.push_str("## 5. Business Model\n\n");
    md.push_str(&format!(
        "**Revenue Model:** {}\n\n\
         _(Describe pricing tiers, contract structure, unit economics, and path to the revenue target. How do you get paid, and how does that scale?)_\n\n",
        business_model
    ));

    md.push_str("## 6. Traction\n\n");
    md.push_str("[FOUNDER TO COMPLETE]\n\n");
    md.push_str("_(List customers, revenue milestones, partnerships, testimonials, or any evidence that the market wants this. Numbers beat words here.)_\n\n");

    md.push_str("## 7. Ask\n\n");
    md.push_str("[FOUNDER TO COMPLETE]\n\n");
    md.push_str("_(State what you're asking for: funding amount, partnership terms, or client engagement. Be specific about how the resources will be deployed and what milestone they unlock.)_\n\n");

    // 4. Determine output path
    let home = std::env::var("USERPROFILE")
        .unwrap_or_else(|_| std::env::var("HOME").unwrap_or_else(|_| ".".to_string()));

    let date_str = chrono::Local::now().format("%Y-%m-%d").to_string();
    let safe_name = business_name
        .replace(|c: char| !c.is_alphanumeric() && c != ' ', "")
        .replace(' ', "_");

    let mut dir = PathBuf::from(&home);
    dir.push("Documents");
    dir.push("NovaSyn MBA Reports");

    if fs::create_dir_all(&dir).is_err() {
        return Ok(GeneratePitchDeckOutlineOutput {
            file_path: String::new(),
            success: 0,
        });
    }

    let file_name = format!("{}_pitch_{}.md", safe_name, date_str);
    let file_path = dir.join(&file_name);

    match fs::write(&file_path, md.as_bytes()) {
        Ok(_) => Ok(GeneratePitchDeckOutlineOutput {
            file_path: file_path.to_string_lossy().to_string(),
            success: 1,
        }),
        Err(_) => Ok(GeneratePitchDeckOutlineOutput {
            file_path: String::new(),
            success: 0,
        }),
    }
}
