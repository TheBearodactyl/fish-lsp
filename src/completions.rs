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
        CompletionItem {
            label: "for".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            insert_text: Some("for $1 in $2\n\t$0\nend".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        },
        CompletionItem {
            label: "while".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            insert_text: Some("while $1\n\t$0\nend".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        },
        CompletionItem {
            label: "if".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            insert_text: Some("if $1\n\t$0\nend".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        },
        CompletionItem {
            label: "elif".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            insert_text: Some("elif $1\n\t$0\nend".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        },
        CompletionItem {
            label: "else".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            insert_text: Some("else\n\t$0\nend".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        },
        CompletionItem {
            label: "return".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            insert_text: Some("return $1".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        },
        CompletionItem {
            label: "exit".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            insert_text: Some("exit $1".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        },
        CompletionItem {
            label: "source".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            insert_text: Some("source $1".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        },
        CompletionItem {
            label: "cd".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            insert_text: Some("cd $1".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        },
        CompletionItem {
            label: "pwd".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            insert_text: Some("pwd".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        },
        CompletionItem {
            label: "ls".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            insert_text: Some("ls $1".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        },
        CompletionItem {
            label: "cat".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            insert_text: Some("cat $1".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        },
        CompletionItem {
            label: "rm".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            insert_text: Some("rm $1".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        },
        CompletionItem {
            label: "mv".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            insert_text: Some("mv $1 $2".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        },
        CompletionItem {
            label: "cp".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            insert_text: Some("cp $1 $2".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        },
        CompletionItem {
            label: "mkdir".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            insert_text: Some("mkdir $1".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        },
        CompletionItem {
            label: "rmdir".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            insert_text: Some("rmdir $1".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        },
        CompletionItem {
            label: "touch".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            insert_text: Some("touch $1".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        },
        CompletionItem {
            label: "chmod".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            insert_text: Some("chmod $1 $2".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        },
        CompletionItem {
            label: "chown".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            insert_text: Some("chown $1 $2".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        },
        CompletionItem {
            label: "chgrp".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            insert_text: Some("chgrp $1 $2".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        },
        CompletionItem {
            label: "ln".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            insert_text: Some("ln $1 $2".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        },
        CompletionItem {
            label: "grep".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            insert_text: Some("grep $1 $2".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        },
        CompletionItem {
            label: "sed".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            insert_text: Some("sed $1 $2".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        },
        CompletionItem {
            label: "awk".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            insert_text: Some("awk $1 $2".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        },
        CompletionItem {
            label: "cut".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            insert_text: Some("cut $1 $2".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        },
        CompletionItem {
            label: "sort".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            insert_text: Some("sort $1".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        },
        CompletionItem {
            label: "uniq".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            insert_text: Some("uniq $1".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        },
        CompletionItem {
            label: "wc".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            insert_text: Some("wc $1".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        },
        CompletionItem {
            label: "head".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            insert_text: Some("head $1".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        },
        CompletionItem {
            label: "tail".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            insert_text: Some("tail $1".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        },
        CompletionItem {
            label: "date".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            insert_text: Some("date".to_string()),
            ..Default::default()
        },
        CompletionItem {
            label: "sleep".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            insert_text: Some("sleep $1".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        },
        CompletionItem {
            label: "kill".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            insert_text: Some("kill $1".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        },
        CompletionItem {
            label: "ps".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            insert_text: Some("ps".to_string()),
            ..Default::default()
        },
        CompletionItem {
            label: "top".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            insert_text: Some("top".to_string()),
            ..Default::default()
        },
        CompletionItem {
            label: "free".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            insert_text: Some("free".to_string()),
            ..Default::default()
        },
        CompletionItem {
            label: "df".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            insert_text: Some("df".to_string()),
            ..Default::default()
        },
        CompletionItem {
            label: "du".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            insert_text: Some("du".to_string()),
            ..Default::default()
        },
        CompletionItem {
            label: "uname".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            insert_text: Some("uname".to_string()),
            ..Default::default()
        },
        CompletionItem {
            label: "uptime".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            insert_text: Some("uptime".to_string()),
            ..Default::default()
        },
        CompletionItem {
            label: "who".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            insert_text: Some("who".to_string()),
            ..Default::default()
        },
        CompletionItem {
            label: "w".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            insert_text: Some("w".to_string()),
            ..Default::default()
        },
        CompletionItem {
            label: "users".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            insert_text: Some("users".to_string()),
            ..Default::default()
        },
        CompletionItem {
            label: "groups".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            insert_text: Some("groups".to_string()),
            ..Default::default()
        },
        CompletionItem {
            label: "id".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            insert_text: Some("id".to_string()),
            ..Default::default()
        },
        CompletionItem {
            label: "whoami".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            insert_text: Some("whoami".to_string()),
            ..Default::default()
        },
        CompletionItem {
            label: "hostname".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            insert_text: Some("hostname".to_string()),
            ..Default::default()
        },
        CompletionItem {
            label: "ping".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            insert_text: Some("ping $1".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        },
        CompletionItem {
            label: "ss".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            insert_text: Some("ss".to_string()),
            ..Default::default()
        },
        CompletionItem {
            label: "test".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            insert_text: Some("test $1 $2 $3".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        },
        CompletionItem {
            label: "break".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            insert_text: Some("break".to_string()),
            ..Default::default()
        },
        CompletionItem {
            label: "continue".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            insert_text: Some("continue".to_string()),
            ..Default::default()
        },
        CompletionItem {
            label: "switch".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            insert_text: Some("switch $1\n\tcase $2\n\t\t$0\n\tbreak\nend".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        },
        CompletionItem {
            label: "case".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            insert_text: Some("case $1\n\t$0\nbreak".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        },
        CompletionItem {
            label: "builtin".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            insert_text: Some("builtin $1 $2".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        },
        CompletionItem {
            label: "time".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            insert_text: Some("time $1".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        },
        CompletionItem {
            label: "begin".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            insert_text: Some("begin\n\t$0\nend".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        },
        CompletionItem {
            label: "end".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            insert_text: Some("end".to_string()),
            ..Default::default()
        },
        CompletionItem {
            label: "set_color".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            insert_text: Some("set_color $1 $2".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        },
        CompletionItem {
            label: "read".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            insert_text: Some("read $1 $2".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        },
        CompletionItem {
            label: "string".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            insert_text: Some("string $1 $2".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        },
        CompletionItem {
            label: "math".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            insert_text: Some("math $1 $2".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        },
        CompletionItem {
            label: "argparse".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            insert_text: Some("argparse $1 $2".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        },
        CompletionItem {
            label: "count".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            insert_text: Some("count $1 $2".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        },
        CompletionItem {
            label: "type".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            insert_text: Some("type $1".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        },
        CompletionItem {
            label: "contains".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            insert_text: Some("contains $1 $2".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        },
        CompletionItem {
            label: "abbr".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            insert_text: Some("abbr $1 $2".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        },
        CompletionItem {
            label: "bind".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            insert_text: Some("bind $1 $2 $3".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        },
        CompletionItem {
            label: "complete".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            insert_text: Some("complete $1 $2 $3".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        },
        CompletionItem {
            label: "commandline".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            insert_text: Some("commandline $1 $2".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        },
        CompletionItem {
            label: "fish_config".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            insert_text: Some("fish_config $1 $2".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        },
        CompletionItem {
            label: "random".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            insert_text: Some("random $1 $2 $3".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        },
        CompletionItem {
            label: "argv".to_string(),
            kind: Some(CompletionItemKind::VARIABLE),
            insert_text: Some("argv[$1]".to_string()),
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

    for variable in &state.custom_vars {
        completions.push(CompletionItem {
            label: variable.clone(),
            kind: Some(CompletionItemKind::VARIABLE),
            insert_text: Some(format!("\\${}", variable.to_string())),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        });
    }

    completions
}
