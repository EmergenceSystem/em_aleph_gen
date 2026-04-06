use aleph_syntax_tree::syntax::AlephTree;
use em_filter::{async_trait, AgentConfig, EmFilterError, Filter, FilterRunner};
use serde_json::{json, Value};

struct AleGenAGent;

#[async_trait]
impl Filter for AleGenAGent {
    async fn handle(&mut self, body: &str) -> Result<Value, EmFilterError> {
        tracing::info!(len = body.len(), "generating aleph code");

        let embryo: Value = serde_json::from_str(body)?;
        let tree: AlephTree = serde_json::from_value(embryo["properties"]["tree"].clone())?;
        let source_lang = embryo["properties"]["source_language"]
            .as_str()
            .unwrap_or("unknown")
            .to_string();

        let code = alegen::generate(tree);

        tracing::info!(source_lang = %source_lang, "generated aleph code");

        Ok(json!([{
            "type": "code",
            "properties": {
                "language": "aleph",
                "source_language": source_lang,
                "content": code
            }
        }]))
    }

    fn capabilities(&self) -> Vec<String> {
        vec!["generator".into(), "aleph".into()]
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    FilterRunner::new("em_aleph_gen", AleGenAGent, AgentConfig::default())
        .run()
        .await
        .unwrap();
}
