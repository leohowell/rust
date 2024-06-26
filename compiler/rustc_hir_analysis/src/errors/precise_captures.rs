use rustc_macros::Diagnostic;
use rustc_span::{Span, Symbol};

#[derive(Diagnostic)]
#[diag(hir_analysis_param_not_captured)]
#[note]
pub struct ParamNotCaptured {
    #[primary_span]
    pub param_span: Span,
    #[label]
    pub opaque_span: Span,
    pub kind: &'static str,
}

#[derive(Diagnostic)]
#[diag(hir_analysis_lifetime_not_captured)]
pub struct LifetimeNotCaptured {
    #[primary_span]
    pub use_span: Span,
    #[label(hir_analysis_param_label)]
    pub param_span: Option<Span>,
    #[label]
    pub opaque_span: Span,
}

#[derive(Diagnostic)]
#[diag(hir_analysis_bad_precise_capture)]
pub struct BadPreciseCapture {
    #[primary_span]
    pub span: Span,
    pub kind: &'static str,
    pub found: String,
}

#[derive(Diagnostic)]
#[diag(hir_analysis_precise_capture_self_alias)]
pub struct PreciseCaptureSelfAlias {
    #[primary_span]
    pub span: Span,
    #[label]
    pub self_span: Span,
    pub what: &'static str,
}

#[derive(Diagnostic)]
#[diag(hir_analysis_duplicate_precise_capture)]
pub struct DuplicatePreciseCapture {
    #[primary_span]
    pub first_span: Span,
    pub name: Symbol,
    #[label]
    pub second_span: Span,
}

#[derive(Diagnostic)]
#[diag(hir_analysis_lifetime_must_be_first)]
pub struct LifetimesMustBeFirst {
    #[primary_span]
    pub lifetime_span: Span,
    pub name: Symbol,
    #[label]
    pub other_span: Span,
}
