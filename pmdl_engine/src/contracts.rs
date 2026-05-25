#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PmdlDocument {
    pub schema_version: String,
    pub content: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationError {
    pub code: String,
    pub message: String,
    pub line: Option<usize>,
    pub column: Option<usize>,
    pub hint: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationReport {
    pub is_valid: bool,
    pub errors: Vec<ValidationError>,
    pub warnings: Vec<String>,
}

impl ValidationReport {
    pub fn valid() -> Self {
        Self {
            is_valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }

    pub fn with_errors(errors: Vec<ValidationError>) -> Self {
        Self {
            is_valid: errors.is_empty(),
            errors,
            warnings: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PipelineInput {
    pub audio_pcm: Vec<i16>,
    pub language: String,
    pub user_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PipelineOutput {
    pub transcript: String,
    pub prompt: String,
    pub final_document: PmdlDocument,
    pub validation: ValidationReport,
    pub iterations_used: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PromptContext {
    pub model_version: String,
    pub product_scope: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GenerationContext {
    pub user_id: String,
    pub attempt: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StageStatus {
    Started,
    Succeeded,
    Failed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StageName {
    Dictation,
    Transcription,
    PromptBuild,
    Generation,
    Validation,
    Feedback,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StageEvent {
    pub stage: StageName,
    pub status: StageStatus,
    pub detail: String,
    pub sequence: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserFeedback {
    pub instruction: String,
    pub author_id: String,
}
