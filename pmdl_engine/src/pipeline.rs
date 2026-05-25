use crate::contracts::{
    GenerationContext, PipelineInput, PipelineOutput, PmdlDocument, PromptContext, StageEvent, StageName,
    StageStatus, ValidationError, ValidationReport,
};
use crate::errors::{EngineError, EngineErrorKind, EngineResult};

pub trait Transcriber {
    fn transcribe(&self, audio_pcm: &[i16], language: &str) -> EngineResult<String>;
}

pub trait PromptBuilder {
    fn build_prompt(&self, transcript: &str, context: &PromptContext) -> String;
    fn build_feedback_prompt(
        &self,
        base_prompt: &str,
        current: &PmdlDocument,
        validation: &ValidationReport,
    ) -> String;
}

pub trait PmdlGenerator {
    fn generate(&self, prompt: &str, context: &GenerationContext) -> EngineResult<PmdlDocument>;
}

pub trait PmdlValidator {
    fn validate(&self, document: &PmdlDocument) -> ValidationReport;
}

pub trait Governance {
    fn authorize(&self, user_id: &str) -> EngineResult<()>;
    fn redact_prompt_input(&self, input: &str) -> String;
}

pub trait Telemetry {
    fn emit(&self, event: StageEvent) -> EngineResult<()>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EngineConfig {
    pub schema_version: String,
    pub max_iterations: usize,
}

pub struct PipelineEngine<T, P, G, V, Gov, Tel>
where
    T: Transcriber,
    P: PromptBuilder,
    G: PmdlGenerator,
    V: PmdlValidator,
    Gov: Governance,
    Tel: Telemetry,
{
    transcriber: T,
    prompt_builder: P,
    generator: G,
    validator: V,
    governance: Gov,
    telemetry: Tel,
    config: EngineConfig,
    event_seq: u64,
}

impl<T, P, G, V, Gov, Tel> PipelineEngine<T, P, G, V, Gov, Tel>
where
    T: Transcriber,
    P: PromptBuilder,
    G: PmdlGenerator,
    V: PmdlValidator,
    Gov: Governance,
    Tel: Telemetry,
{
    pub fn new(
        transcriber: T,
        prompt_builder: P,
        generator: G,
        validator: V,
        governance: Gov,
        telemetry: Tel,
        config: EngineConfig,
    ) -> Self {
        Self {
            transcriber,
            prompt_builder,
            generator,
            validator,
            governance,
            telemetry,
            config,
            event_seq: 0,
        }
    }

    pub fn run(&mut self, input: PipelineInput) -> EngineResult<PipelineOutput> {
        if input.audio_pcm.is_empty() {
            return Err(EngineError::new(
                EngineErrorKind::InvalidInput,
                "PMDL_INVALID_INPUT",
                "audio_pcm is required",
            ));
        }

        self.governance.authorize(&input.user_id)?;

        self.emit(StageName::Dictation, StageStatus::Started, "dictation received")?;
        self.emit(
            StageName::Transcription,
            StageStatus::Started,
            "transcription started",
        )?;
        let transcript = self
            .transcriber
            .transcribe(&input.audio_pcm, &input.language)
            .map_err(|e| {
                EngineError::new(EngineErrorKind::Transcription, e.code, e.message)
            })?;
        self.emit(
            StageName::Transcription,
            StageStatus::Succeeded,
            "transcription completed",
        )?;

        self.emit(StageName::PromptBuild, StageStatus::Started, "prompt build")?;
        let prompt_context = PromptContext {
            model_version: self.config.schema_version.clone(),
            product_scope: "pmdl_engine".to_string(),
        };
        let prompt = self.prompt_builder.build_prompt(
            &self.governance.redact_prompt_input(&transcript),
            &prompt_context,
        );
        self.emit(
            StageName::PromptBuild,
            StageStatus::Succeeded,
            "prompt built",
        )?;

        let mut current_prompt = prompt.clone();
        let mut final_document: Option<PmdlDocument> = None;
        let mut final_validation = ValidationReport::with_errors(vec![ValidationError {
            code: "PMDL_VALIDATION_NOT_RUN".to_string(),
            message: "validation not run".to_string(),
            line: None,
            column: None,
            hint: None,
        }]);

        for attempt in 1..=self.config.max_iterations {
            self.emit(StageName::Generation, StageStatus::Started, "llm generation")?;
            let generated = self
                .generator
                .generate(
                    &current_prompt,
                    &GenerationContext {
                        user_id: input.user_id.clone(),
                        attempt,
                    },
                )
                .map_err(|e| EngineError::new(EngineErrorKind::Generation, e.code, e.message))?;
            self.emit(
                StageName::Generation,
                StageStatus::Succeeded,
                "llm generation completed",
            )?;

            self.emit(StageName::Validation, StageStatus::Started, "validation started")?;
            let report = self.validator.validate(&generated);
            self.emit(
                StageName::Validation,
                StageStatus::Succeeded,
                "validation completed",
            )?;

            if report.is_valid {
                final_document = Some(generated);
                final_validation = report;
                return Ok(PipelineOutput {
                    transcript,
                    prompt,
                    final_document: final_document.expect("final document exists"),
                    validation: final_validation,
                    iterations_used: attempt,
                });
            }

            self.emit(
                StageName::Feedback,
                StageStatus::Started,
                "feedback loop update",
            )?;
            current_prompt = self
                .prompt_builder
                .build_feedback_prompt(&prompt, &generated, &report);
            final_document = Some(generated);
            final_validation = report;
            self.emit(
                StageName::Feedback,
                StageStatus::Succeeded,
                "feedback prompt built",
            )?;
        }

        Err(EngineError::new(
            EngineErrorKind::IterationLimit,
            "PMDL_ITERATION_LIMIT",
            format!(
                "validation did not converge after {} iterations; last_error_count={}",
                self.config.max_iterations,
                final_validation.errors.len()
            ),
        ))
    }

    fn emit(&mut self, stage: StageName, status: StageStatus, detail: &str) -> EngineResult<()> {
        self.event_seq = self.event_seq.saturating_add(1);
        self.telemetry.emit(StageEvent {
            stage,
            status,
            detail: detail.to_string(),
            sequence: self.event_seq,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct DummyTranscriber;
    impl Transcriber for DummyTranscriber {
        fn transcribe(&self, _audio_pcm: &[i16], _language: &str) -> EngineResult<String> {
            Ok("user says create active user and order".to_string())
        }
    }

    struct DummyPrompt;
    impl PromptBuilder for DummyPrompt {
        fn build_prompt(&self, transcript: &str, _context: &PromptContext) -> String {
            format!("PROMPT::{transcript}")
        }

        fn build_feedback_prompt(
            &self,
            base_prompt: &str,
            current: &PmdlDocument,
            validation: &ValidationReport,
        ) -> String {
            format!(
                "{base_prompt}\nFIX:{}:{}",
                current.content,
                validation.errors.len()
            )
        }
    }

    struct DummyGenerator {
        calls: RefCell<usize>,
    }
    impl PmdlGenerator for DummyGenerator {
        fn generate(&self, _prompt: &str, _context: &GenerationContext) -> EngineResult<PmdlDocument> {
            let mut calls = self.calls.borrow_mut();
            *calls += 1;
            let content = if *calls == 1 {
                "@usuario[persona]\n@usuario --crea--> @pedido\nINVALID".to_string()
            } else {
                "@usuario[persona]\n@pedido[transaccion]\n@usuario --crea--> @pedido".to_string()
            };
            Ok(PmdlDocument {
                schema_version: "pmdl.v1".to_string(),
                content,
            })
        }
    }

    struct DummyValidator;
    impl PmdlValidator for DummyValidator {
        fn validate(&self, document: &PmdlDocument) -> ValidationReport {
            if document.content.contains("INVALID") {
                ValidationReport::with_errors(vec![ValidationError {
                    code: "INVALID_TOKEN".to_string(),
                    message: "Found INVALID token".to_string(),
                    line: Some(3),
                    column: Some(1),
                    hint: Some("Remove INVALID token".to_string()),
                }])
            } else {
                ValidationReport::valid()
            }
        }
    }

    struct DummyGovernance;
    impl Governance for DummyGovernance {
        fn authorize(&self, _user_id: &str) -> EngineResult<()> {
            Ok(())
        }

        fn redact_prompt_input(&self, input: &str) -> String {
            input.replace("secret", "[REDACTED]")
        }
    }

    struct DummyTelemetry {
        count: RefCell<usize>,
    }
    impl Telemetry for DummyTelemetry {
        fn emit(&self, _event: StageEvent) -> EngineResult<()> {
            *self.count.borrow_mut() += 1;
            Ok(())
        }
    }

    #[test]
    fn converges_with_feedback_loop() {
        let mut engine = PipelineEngine::new(
            DummyTranscriber,
            DummyPrompt,
            DummyGenerator {
                calls: RefCell::new(0),
            },
            DummyValidator,
            DummyGovernance,
            DummyTelemetry {
                count: RefCell::new(0),
            },
            EngineConfig {
                schema_version: "pmdl.v1".to_string(),
                max_iterations: 3,
            },
        );

        let output = engine
            .run(PipelineInput {
                audio_pcm: vec![1, 2, 3],
                language: "en-US".to_string(),
                user_id: "user-1".to_string(),
            })
            .expect("pipeline should converge");

        assert!(output.validation.is_valid);
        assert_eq!(output.iterations_used, 2);
        assert!(output.final_document.content.contains("@pedido[transaccion]"));
    }

    #[test]
    fn fails_when_iteration_limit_reached() {
        struct NeverValid;
        impl PmdlValidator for NeverValid {
            fn validate(&self, _document: &PmdlDocument) -> ValidationReport {
                ValidationReport::with_errors(vec![ValidationError {
                    code: "RULE_FAIL".to_string(),
                    message: "rule fail".to_string(),
                    line: None,
                    column: None,
                    hint: None,
                }])
            }
        }

        let mut engine = PipelineEngine::new(
            DummyTranscriber,
            DummyPrompt,
            DummyGenerator {
                calls: RefCell::new(0),
            },
            NeverValid,
            DummyGovernance,
            DummyTelemetry {
                count: RefCell::new(0),
            },
            EngineConfig {
                schema_version: "pmdl.v1".to_string(),
                max_iterations: 1,
            },
        );

        let error = engine
            .run(PipelineInput {
                audio_pcm: vec![1],
                language: "en-US".to_string(),
                user_id: "user-1".to_string(),
            })
            .expect_err("pipeline should stop after one iteration");

        assert_eq!(error.kind, EngineErrorKind::IterationLimit);
        assert_eq!(error.code, "PMDL_ITERATION_LIMIT");
    }
}
