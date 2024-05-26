use crate::server::State;
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_lsp::lsp_types::*;

pub async fn get_completions(state: Arc<RwLock<State>>) -> Vec<CompletionItem> {
    let state = state.read().await;

    let mut completions = vec![
        CompletionItem {
            label: "echo".to_string(),
            kind: Some(CompletionItemKind::FUNCTION),
            insert_text: Some("echo \"${1}\"".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        },
        CompletionItem {
            label: "function".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            insert_text: Some("function $1\n\t$0\nend".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        },
        CompletionItem {
            label: "set".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            insert_text: Some("set $1 $2 $3".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        },
    ];

    for function in &state.custom_funcs {
        completions.push(CompletionItem {
            label: function.clone(),
            kind: Some(CompletionItemKind::KEYWORD),
            insert_text: Some(format!("{}", function.to_string())),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        });
    }

    completions
}
