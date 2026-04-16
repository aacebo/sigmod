use std::collections::BTreeMap;

use actix_web::web::Html;
use actix_web::{HttpResponse, get, post, web};
use askama::Template;

use crate::RequestContext;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(page)
        .service(add_scorer)
        .service(add_category)
        .service(add_label)
        .service(add_criterion)
        .service(submit_eval);
}

// -- State types (mirrors the client-side form shape) --

#[derive(Default, Clone, serde::Deserialize)]
pub struct ConsoleState {
    #[serde(default)]
    pub input: String,
    #[serde(default)]
    pub scorers: Vec<ConsoleScorer>,
}

#[derive(Clone, serde::Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ConsoleScorer {
    Classifier(ClassifierState),
    Judge(JudgeState),
}

#[derive(Clone, serde::Deserialize)]
pub struct ClassifierState {
    #[serde(default = "default_classifier_model")]
    pub model: String,
    #[serde(default = "default_consensus")]
    pub consensus: String,
    #[serde(default = "default_weight")]
    pub weight: f32,
    #[serde(default = "default_classifier_threshold")]
    pub threshold: f32,
    #[serde(default)]
    pub categories: BTreeMap<String, CategoryState>,
}

#[derive(Clone, serde::Deserialize)]
pub struct CategoryState {
    #[serde(default = "default_consensus")]
    pub consensus: String,
    #[serde(default = "default_weight")]
    pub weight: f32,
    #[serde(default = "default_label_threshold")]
    pub threshold: f32,
    #[serde(default)]
    pub labels: BTreeMap<String, LabelState>,
}

#[derive(Clone, serde::Deserialize)]
pub struct LabelState {
    #[serde(default = "default_weight")]
    pub weight: f32,
    #[serde(default = "default_label_threshold")]
    pub threshold: f32,
    #[serde(default)]
    pub description: Option<String>,
}

#[derive(Clone, serde::Deserialize)]
pub struct JudgeState {
    #[serde(default = "default_judge_model")]
    pub model: String,
    #[serde(default)]
    pub access_token: String,
    #[serde(default)]
    pub name: String,
    #[serde(default = "default_consensus")]
    pub consensus: String,
    #[serde(default = "default_weight")]
    pub weight: f32,
    #[serde(default = "default_judge_threshold")]
    pub threshold: f32,
    #[serde(default)]
    pub prompt: String,
    #[serde(default)]
    pub criteria: Vec<CriterionState>,
}

#[derive(Clone, serde::Deserialize)]
pub struct CriterionState {
    #[serde(default)]
    pub description: String,
    #[serde(default = "default_weight")]
    pub weight: f32,
    #[serde(default = "default_label_threshold")]
    pub threshold: f32,
}

fn default_classifier_model() -> String {
    "local/bart-mnli".to_string()
}
fn default_judge_model() -> String {
    "openai/gpt-4o-mini".to_string()
}
fn default_consensus() -> String {
    "top_k:2".to_string()
}
fn default_weight() -> f32 {
    1.0
}
fn default_classifier_threshold() -> f32 {
    0.75
}
fn default_judge_threshold() -> f32 {
    0.7
}
fn default_label_threshold() -> f32 {
    0.7
}

impl Default for ClassifierState {
    fn default() -> Self {
        Self {
            model: default_classifier_model(),
            consensus: default_consensus(),
            weight: default_weight(),
            threshold: default_classifier_threshold(),
            categories: BTreeMap::new(),
        }
    }
}

impl Default for JudgeState {
    fn default() -> Self {
        Self {
            model: default_judge_model(),
            access_token: String::new(),
            name: String::new(),
            consensus: default_consensus(),
            weight: default_weight(),
            threshold: default_judge_threshold(),
            prompt: String::new(),
            criteria: Vec::new(),
        }
    }
}

impl Default for CategoryState {
    fn default() -> Self {
        Self {
            consensus: default_consensus(),
            weight: default_weight(),
            threshold: default_label_threshold(),
            labels: BTreeMap::new(),
        }
    }
}

impl Default for LabelState {
    fn default() -> Self {
        Self {
            weight: default_weight(),
            threshold: default_label_threshold(),
            description: None,
        }
    }
}

impl Default for CriterionState {
    fn default() -> Self {
        Self {
            description: String::new(),
            weight: default_weight(),
            threshold: default_label_threshold(),
        }
    }
}

// -- Templates --

#[derive(Template)]
#[template(path = "console/page.html")]
struct PlaygroundPage {
    state: ConsoleState,
}

#[derive(Template)]
#[template(path = "console/partials/scorer_classifier.html")]
struct ClassifierFragment {
    classifier: ClassifierState,
}

#[derive(Template)]
#[template(path = "console/partials/scorer_judge.html")]
struct JudgeFragment {
    judge: JudgeState,
}

#[derive(Template)]
#[template(path = "console/partials/category.html")]
struct CategoryFragment {
    cat_name: String,
    category: CategoryState,
}

#[derive(Template)]
#[template(path = "console/partials/label.html")]
struct LabelFragment {
    label_name: String,
    label: LabelState,
}

#[derive(Template)]
#[template(path = "console/partials/criterion.html")]
struct CriterionFragment {
    criterion: CriterionState,
}

#[derive(Template)]
#[template(path = "console/partials/result.html")]
struct ResultFragment {
    result: eval::EvalResult,
    json: String,
}

// -- Handlers --

#[derive(serde::Deserialize)]
struct ConsoleQuery {
    state: Option<String>,
}

#[derive(serde::Deserialize)]
struct ScorerQuery {
    r#type: String,
}

fn render(tmpl: &impl Template) -> Result<Html, actix_web::Error> {
    Ok(Html::new(
        tmpl.render()
            .map_err(actix_web::error::ErrorInternalServerError)?,
    ))
}

#[get("/console")]
async fn page(query: web::Query<ConsoleQuery>) -> Result<Html, actix_web::Error> {
    let state = query
        .state
        .as_deref()
        .map(serde_json::from_str::<ConsoleState>)
        .transpose()
        .map_err(actix_web::error::ErrorBadRequest)?
        .unwrap_or_default();

    render(&PlaygroundPage { state })
}

#[post("/console/scorer")]
async fn add_scorer(query: web::Query<ScorerQuery>) -> Result<Html, actix_web::Error> {
    match query.r#type.as_str() {
        "classifier" => render(&ClassifierFragment {
            classifier: ClassifierState::default(),
        }),
        "judge" => render(&JudgeFragment {
            judge: JudgeState::default(),
        }),
        _ => Err(actix_web::error::ErrorBadRequest("invalid scorer type")),
    }
}

#[post("/console/category")]
async fn add_category() -> Result<Html, actix_web::Error> {
    render(&CategoryFragment {
        cat_name: String::new(),
        category: CategoryState::default(),
    })
}

#[post("/console/label")]
async fn add_label() -> Result<Html, actix_web::Error> {
    render(&LabelFragment {
        label_name: String::new(),
        label: LabelState::default(),
    })
}

#[post("/console/criterion")]
async fn add_criterion() -> Result<Html, actix_web::Error> {
    render(&CriterionFragment {
        criterion: CriterionState::default(),
    })
}

#[post("/console/eval")]
async fn submit_eval(
    ctx: RequestContext,
    payload: web::Json<eval::EvalRequest>,
) -> Result<HttpResponse, actix_web::Error> {
    let req = payload.into_inner();
    let result = ctx.runner().evaluate(&req).await;
    let json =
        serde_json::to_string_pretty(&result).unwrap_or_else(|e| format!("serialize error: {e}"));

    let tmpl = ResultFragment { result, json };

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(
            tmpl.render()
                .map_err(actix_web::error::ErrorInternalServerError)?,
        ))
}
