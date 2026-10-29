use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct EmailInput {
    #[serde(default)]
    pub invocation: Option<InvocationContext>,
    pub sender: Sender,
    pub subject: String,
    #[serde(rename = "textBody", default)]
    pub text_body: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct InvocationContext {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    pub config: serde_json::Value,
}

#[derive(Debug, Deserialize)]
pub struct Sender {
    pub address: String,
}

#[derive(Debug, Serialize)]
struct ExtractionResult {
    #[serde(rename = "schemaVersion")]
    schema_version: &'static str,
    r#type: String,
    template: &'static str,
    confidence: f64,
    summary: String,
    fields: Fields,
    actions: Vec<Action>,
}

#[derive(Debug, Serialize)]
struct Fields {
    sender: String,
    subject: String,
    preview: String,
    #[serde(rename = "wordCount")]
    word_count: usize,
}

#[derive(Debug, Serialize)]
struct Action {
    id: &'static str,
    kind: &'static str,
    label: &'static str,
}

const PREVIEW_LENGTH: usize = 200;

pub fn run(input: &[u8]) -> Result<String, Box<dyn std::error::Error>> {
    let email: EmailInput = serde_json::from_slice(input)?;

    let preview_length = match &email.invocation {
        Some(context) if context.schema_version != "1" => {
            return Err("unsupported invocation context".into())
        }
        Some(context) => context
            .config
            .get("previewLength")
            .and_then(|v| v.as_f64())
            .map(|v| v as usize)
            .unwrap_or(PREVIEW_LENGTH),
        None => PREVIEW_LENGTH,
    };
    let text_body = email.text_body.unwrap_or_default();
    let mut preview: String = text_body.chars().take(preview_length).collect();
    if text_body.chars().count() > preview_length {
        preview.push('…');
    }
    let word_count = text_body.split_whitespace().count();

    let result = ExtractionResult {
        schema_version: "1",
        r#type: "rust_template.text_preview".to_string(),
        template: "text-preview",
        confidence: 0.8,
        summary: format!("{} — {}", email.sender.address, email.subject),
        fields: Fields {
            sender: email.sender.address,
            subject: email.subject,
            preview,
            word_count,
        },
        actions: vec![Action {
            id: "archive",
            kind: "suggest_archive",
            label: "Archive",
        }],
    };

    Ok(serde_json::to_string(&result)?)
}
