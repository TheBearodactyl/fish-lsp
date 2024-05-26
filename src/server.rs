use {
    crate::cli::Cli,
    anyhow::Result as Anyhow,
    clap::Parser,
    regex::Regex,
    std::{str::FromStr, sync::Arc},
    tokio::sync::RwLock,
    tower_lsp::{jsonrpc::Result, lsp_types::*, Client, LanguageServer, LspService, Server},
};

#[derive(Debug, Clone)]
pub struct Backend {
    client: Client,
    state: Arc<RwLock<State>>,
}

#[derive(Debug, Default, Clone)]
pub struct State {
    pub custom_funcs: Vec<String>,
    pub custom_vars: Vec<String>,
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                completion_provider: Some(CompletionOptions {
                    resolve_provider: Some(false),
                    trigger_characters: Some(vec![".".to_string()]),
                    ..Default::default()
                }),
                ..Default::default()
            },
            ..Default::default()
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "Fish Language Server initialized!")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        let comps = crate::completions::get_completions(self.state.clone()).await;

        Ok(Some(CompletionResponse::Array(comps)))
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let params_clone = params.clone();

        self.check_syntax(params.text_document.text).await;
        self.update_custom_funcs(params_clone.text_document.text)
            .await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let content = params
            .content_changes
            .into_iter()
            .next()
            .map(|change| change.text)
            .unwrap_or_default();

        let sssniperwolf = content.clone();

        self.check_syntax(content).await;
        self.update_custom_funcs(sssniperwolf).await;
    }
}

impl Backend {
    async fn check_syntax(&self, text: String) {
        let mut diagnostics: Vec<Diagnostic> = Vec::new();
        let function_re = Regex::new(r"(?m)^function\s+\w+\s*(?:\n.*)*?\nend\s*$").unwrap();
        let variable_re = Regex::new(r#"(?m)^set\s+\w+(\s+\"[^\"]*\")*\s*$"#).unwrap();

        for (i, line) in text.lines().enumerate() {
            if line.starts_with("function ") && !function_re.is_match(&text) {
                diagnostics.push(Diagnostic {
                    range: Range {
                        start: Position {
                            line: i as u32,
                            character: 0,
                        },
                        end: Position {
                            line: i as u32,
                            character: line.len() as u32,
                        },
                    },
                    severity: Some(DiagnosticSeverity::ERROR),
                    code: Some(NumberOrString::String(
                        "INVALID_FUNCTION_DECLARATION".into(),
                    )),
                    source: Some("fish-lsp".into()),
                    message: "Functions must be declared as `function <name>` followed by `end`"
                        .into(),
                    ..Default::default()
                })
            }
        }

        for (i, line) in text.lines().enumerate() {
            if line.starts_with("set ") && !variable_re.is_match(line) {
                diagnostics.push(Diagnostic {
                    range: Range {
                        start: Position {
                            line: i as u32,
                            character: 0,
                        },
                        end: Position {
                            line: i as u32,
                            character: line.len() as u32,
                        },
                    },
                    severity: Some(DiagnosticSeverity::ERROR),
                    code: Some(NumberOrString::String(
                        "INVALID_VARIABLE_DECLARATION".into(),
                    )),
                    source: Some("fish-language-server".into()),
                    message: "Variables must be declared as `set <name> \"<value>\"`".into(),
                    ..Default::default()
                });
            }
        }

        self.client
            .publish_diagnostics(
                Url::from_str("file://dummy.fish").unwrap(),
                diagnostics,
                None,
            )
            .await;
    }

    async fn update_custom_funcs(&self, text: String) {
        let function_re = Regex::new(r"(?m)^function\s+(\w+)").unwrap();
        let variable_re = Regex::new(r"(?m)^set\s+(\w+)\s+").unwrap();

        let mut custom_functions = Vec::new();
        let mut custom_variables = Vec::new();

        for cap in function_re.captures_iter(&text) {
            if let Some(function_name) = cap.get(1) {
                custom_functions.push(function_name.as_str().to_string());
            }
        }

        for cap in variable_re.captures_iter(&text) {
            if let Some(variable_name) = cap.get(1) {
                custom_variables.push(variable_name.as_str().to_string());
            }
        }

        let mut state = self.state.write().await;
        state.custom_funcs = custom_functions;
        state.custom_vars = custom_variables;
    }
}

pub async fn run_server() -> Anyhow<()> {
    let argv = Cli::parse();
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();
    let state = Arc::new(RwLock::new(State::default()));

    let (service, socket) = LspService::new(|client| Backend { client, state });

    Server::new(stdin, stdout, socket).serve(service).await;

    Ok(())
}
