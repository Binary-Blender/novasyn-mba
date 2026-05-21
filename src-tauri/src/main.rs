#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod db;
mod ai_service;
mod router;
mod compiler;

use std::sync::Mutex;
use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let app_dir = app.path().app_data_dir().expect("failed to resolve app data dir");
            std::fs::create_dir_all(&app_dir).ok();
            let db_path = app_dir.join("novasyn_mba.db");
            let pool = db::init_db(db_path);
            app.manage(pool);
            let api_keys = Mutex::new(ai_service::load_api_keys());
            app.manage(api_keys);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            ai_service::send_chat,
            ai_service::get_api_keys,
            ai_service::set_api_key,
            commands::business::list_businesses,
            commands::business::create_business,
            commands::business::get_business,
            commands::business::update_business,
            commands::business::delete_business,
            commands::strategic_position::list_strategic_positions,
            commands::strategic_position::create_strategic_position,
            commands::strategic_position::get_strategic_position,
            commands::strategic_position::update_strategic_position,
            commands::strategic_position::delete_strategic_position,
            commands::operations_audit::list_operations_audits,
            commands::operations_audit::create_operations_audit,
            commands::operations_audit::get_operations_audit,
            commands::operations_audit::update_operations_audit,
            commands::operations_audit::delete_operations_audit,
            commands::dollar_audit::list_dollar_audits,
            commands::dollar_audit::create_dollar_audit,
            commands::dollar_audit::get_dollar_audit,
            commands::dollar_audit::update_dollar_audit,
            commands::dollar_audit::delete_dollar_audit,
            commands::financial_snapshot::list_financial_snapshots,
            commands::financial_snapshot::create_financial_snapshot,
            commands::financial_snapshot::get_financial_snapshot,
            commands::financial_snapshot::update_financial_snapshot,
            commands::financial_snapshot::delete_financial_snapshot,
            commands::marketing_asset::list_marketing_assets,
            commands::marketing_asset::create_marketing_asset,
            commands::marketing_asset::get_marketing_asset,
            commands::marketing_asset::update_marketing_asset,
            commands::marketing_asset::delete_marketing_asset,
            commands::sales_contact::list_sales_contacts,
            commands::sales_contact::create_sales_contact,
            commands::sales_contact::get_sales_contact,
            commands::sales_contact::update_sales_contact,
            commands::sales_contact::delete_sales_contact,
            commands::content_pipeline_item::list_content_pipeline_items,
            commands::content_pipeline_item::create_content_pipeline_item,
            commands::content_pipeline_item::get_content_pipeline_item,
            commands::content_pipeline_item::update_content_pipeline_item,
            commands::content_pipeline_item::delete_content_pipeline_item,
            commands::a_i_stack_decision::list_a_i_stack_decisions,
            commands::a_i_stack_decision::create_a_i_stack_decision,
            commands::a_i_stack_decision::get_a_i_stack_decision,
            commands::a_i_stack_decision::update_a_i_stack_decision,
            commands::a_i_stack_decision::delete_a_i_stack_decision,
            commands::board_member::list_board_members,
            commands::board_member::create_board_member,
            commands::board_member::get_board_member,
            commands::board_member::update_board_member,
            commands::board_member::delete_board_member,
            commands::portfolio_synergy::list_portfolio_synergies,
            commands::portfolio_synergy::create_portfolio_synergy,
            commands::portfolio_synergy::get_portfolio_synergy,
            commands::portfolio_synergy::update_portfolio_synergy,
            commands::portfolio_synergy::delete_portfolio_synergy,
            commands::weekly_review::list_weekly_reviews,
            commands::weekly_review::create_weekly_review,
            commands::weekly_review::get_weekly_review,
            commands::weekly_review::update_weekly_review,
            commands::weekly_review::delete_weekly_review,
            commands::actions::strategy_advisor,
            commands::actions::operations_advisor,
            commands::actions::finance_advisor,
            commands::actions::marketing_advisor,
            commands::actions::sales_advisor,
            commands::actions::tech_advisor,
            commands::actions::leadership_advisor,
            commands::actions::portfolio_synergy_detector,
            commands::actions::synthesize_first_30_days,
            commands::advance_business_stage::advance_business_stage,
            commands::calculate_ltv_cac::calculate_ltv_cac,
            commands::export_business_report::export_business_report,
            commands::generate_pitch_deck_outline::generate_pitch_deck_outline,
            commands::import_csv_contacts::import_csv_contacts,
            router::broadcast_chat,
            router::council_chat,
            compiler::read_document_content,
            compiler::write_document_content,
            compiler::scan_documents_dir,
            compiler::reviews_to_action_plan,
            compiler::advisor_outputs_to_priorities,
            compiler::financial_data_to_report,
            compiler::business_data_to_pitch,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
