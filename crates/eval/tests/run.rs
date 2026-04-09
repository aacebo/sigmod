use std::collections::BTreeMap;

use ai::local::classify::LocalClassifierBuilder;
use ai::openai::OpenAIClient;
use eval::{Decision, EvalRequest, Runner, ScorerInput, ScorerOutput};

fn api_key() -> String {
    std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set")
}

fn judge_model() -> ai::model::ModelId {
    "openai/gpt-4o-mini".parse().unwrap()
}

fn runner() -> Runner {
    let client = ai::client::Client::new().with_chat(OpenAIClient::new());
    Runner::new().with(judge_model(), client)
}

fn judge_input(threshold: f32, criteria: Vec<(&str, f32)>) -> ScorerInput {
    ScorerInput::Judge(eval::judge::Input {
        model: judge_model(),
        access_token: api_key(),
        name: "quality-judge".to_string(),
        top_k: None,
        weight: 1.0,
        threshold,
        prompt: None,
        criteria: criteria
            .into_iter()
            .map(|(desc, threshold)| eval::judge::Criterion {
                weight: 1.0,
                threshold,
                description: desc.to_string(),
            })
            .collect(),
    })
}

#[tokio::test]
#[ignore]
async fn judge_accepts_high_quality_text() {
    let req = EvalRequest {
        request_id: None,
        input: "The mitochondria is the powerhouse of the cell. It generates ATP through oxidative phosphorylation.".to_string(),
        scorers: vec![judge_input(0.5, vec![
            ("Is the text factually accurate?", 0.5),
            ("Is the text coherent and well-structured?", 0.5),
        ])],
    };

    let result = runner().evaluate(&req).await;

    assert_eq!(result.scorers.len(), 1);

    if let ScorerOutput::Judge(o) = &result.scorers[0] {
        println!("score: {}, reasoning: {:?}", o.score, o.reasoning);

        for (i, c) in o.criteria.iter().enumerate() {
            println!(
                "  criterion {i}: score={}, decision={:?}, reasoning={}",
                c.score, c.decision, c.reasoning
            );
        }

        assert_eq!(o.decision, Decision::Accept);
        assert!(o.score >= 0.5, "expected score >= 0.5, got {}", o.score);
        assert_eq!(o.criteria.len(), 2);
    } else {
        panic!("expected judge output, got {:?}", result.scorers[0]);
    }

    assert_eq!(result.decision, Decision::Accept);
}

#[tokio::test]
#[ignore]
async fn judge_rejects_low_quality_text() {
    let req = EvalRequest {
        request_id: None,
        input: "asdf jkl; qwerty uiop zxcv bnm".to_string(),
        scorers: vec![judge_input(
            0.7,
            vec![
                ("Is the text factually accurate?", 0.7),
                ("Is the text coherent and well-structured?", 0.7),
            ],
        )],
    };

    let result = runner().evaluate(&req).await;

    if let ScorerOutput::Judge(o) = &result.scorers[0] {
        println!("score: {}, reasoning: {:?}", o.score, o.reasoning);

        for (i, c) in o.criteria.iter().enumerate() {
            println!(
                "  criterion {i}: score={}, decision={:?}, reasoning={}",
                c.score, c.decision, c.reasoning
            );
        }

        assert_eq!(o.decision, Decision::Reject);
        assert!(o.score < 0.7, "expected score < 0.7, got {}", o.score);
    } else {
        panic!("expected judge output, got {:?}", result.scorers[0]);
    }

    assert_eq!(result.decision, Decision::Reject);
}

#[tokio::test]
#[ignore]
async fn multiple_judges_all_accept() {
    let req = EvalRequest {
        request_id: None,
        input: "Rust is a systems programming language focused on safety, speed, and concurrency."
            .to_string(),
        scorers: vec![
            judge_input(0.5, vec![("Is the text factually accurate?", 0.5)]),
            judge_input(
                0.5,
                vec![
                    ("Is the text coherent?", 0.5),
                    ("Is the text informative?", 0.5),
                ],
            ),
        ],
    };

    let result = runner().evaluate(&req).await;

    assert_eq!(result.scorers.len(), 2);
    assert!(result.score > 0.0, "expected aggregated score > 0");
    assert_eq!(result.decision, Decision::Accept);

    for (i, scorer) in result.scorers.iter().enumerate() {
        if let ScorerOutput::Judge(o) = scorer {
            println!("scorer {i}: score={}, decision={:?}", o.score, o.decision);
            assert_eq!(o.decision, Decision::Accept);
        } else {
            panic!("expected judge output for scorer {i}");
        }
    }
}

#[tokio::test]
#[ignore]
async fn mixed_accept_reject_means_reject() {
    let req = EvalRequest {
        request_id: None,
        input: "The sky is green and made of cheese.".to_string(),
        scorers: vec![
            judge_input(
                0.3,
                vec![("Is the text coherent and grammatically correct?", 0.3)],
            ),
            judge_input(
                0.9,
                vec![(
                    "Is the text factually accurate and scientifically correct?",
                    0.9,
                )],
            ),
        ],
    };

    let result = runner().evaluate(&req).await;

    assert_eq!(result.scorers.len(), 2);
    assert_eq!(result.decision, Decision::Reject);

    for (i, scorer) in result.scorers.iter().enumerate() {
        if let ScorerOutput::Judge(o) = scorer {
            println!("scorer {i}: score={}, decision={:?}", o.score, o.decision);
        }
    }
}

#[tokio::test]
#[ignore]
async fn judge_top_k_uses_best_criteria() {
    let req = EvalRequest {
        request_id: None,
        input: "Water boils at 100 degrees Celsius at sea level.".to_string(),
        scorers: vec![ScorerInput::Judge(eval::judge::Input {
            model: judge_model(),
            access_token: api_key(),
            name: "selective-judge".to_string(),
            top_k: Some(1),
            weight: 1.0,
            threshold: 0.5,
            prompt: None,
            criteria: vec![
                eval::judge::Criterion {
                    weight: 1.0,
                    threshold: 0.5,
                    description: "Is the text factually accurate?".to_string(),
                },
                eval::judge::Criterion {
                    weight: 1.0,
                    threshold: 0.5,
                    description: "Does the text contain humor or jokes?".to_string(),
                },
            ],
        })],
    };

    let result = runner().evaluate(&req).await;

    if let ScorerOutput::Judge(o) = &result.scorers[0] {
        println!("overall score (top_k=1): {}", o.score);

        for (i, c) in o.criteria.iter().enumerate() {
            println!("  criterion {i}: score={}", c.score);
        }

        let max_criterion = o.criteria.iter().map(|c| c.score).fold(0.0f32, f32::max);
        assert!(
            (o.score - max_criterion).abs() < 0.01,
            "top_k=1 score {} should equal best criterion {}",
            o.score,
            max_criterion
        );
    } else {
        panic!("expected judge output, got {:?}", result.scorers[0]);
    }
}

#[tokio::test]
#[ignore]
async fn eval_result_has_valid_id() {
    let req = EvalRequest {
        request_id: None,
        input: "Simple test input.".to_string(),
        scorers: vec![judge_input(0.5, vec![("Is this text readable?", 0.5)])],
    };

    let r1 = runner().evaluate(&req).await;
    let r2 = runner().evaluate(&req).await;

    assert!(r1.id.to_string().starts_with("eval_"));
    assert!(r2.id.to_string().starts_with("eval_"));
    assert_ne!(r1.id, r2.id);
}

#[tokio::test]
#[ignore]
async fn missing_model_produces_error() {
    let runner = Runner::new(); // no clients registered

    let req = EvalRequest {
        request_id: None,
        input: "Some text".to_string(),
        scorers: vec![ScorerInput::Judge(eval::judge::Input {
            model: judge_model(),
            access_token: "unused".to_string(),
            name: "judge".to_string(),
            top_k: None,
            weight: 1.0,
            threshold: 0.5,
            prompt: None,
            criteria: vec![eval::judge::Criterion {
                weight: 1.0,
                threshold: 0.5,
                description: "Is this readable?".to_string(),
            }],
        })],
    };

    let result = runner.evaluate(&req).await;

    assert!(matches!(&result.scorers[0], ScorerOutput::Error(_)));
    assert_eq!(result.decision, Decision::Reject);
}

// --- Classifier tests ---

fn classifier_model() -> ai::model::ModelId {
    "local/bart-mnli".parse().unwrap()
}

async fn classifier_runner() -> Runner {
    let client = ai::client::Client::new().with_classifier(
        LocalClassifierBuilder::new()
            .build_async()
            .await
            .expect("failed to build classifier"),
    );

    Runner::new().with(classifier_model(), client)
}

fn make_category(
    labels: Vec<(&str, f32)>,
    top_k: usize,
    threshold: f32,
) -> eval::classifier::Category {
    let mut label_map = BTreeMap::new();

    for (name, thresh) in labels {
        label_map.insert(
            name.to_string(),
            eval::classifier::Label {
                weight: 1.0,
                threshold: thresh,
                description: None,
            },
        );
    }

    eval::classifier::Category {
        top_k,
        weight: 1.0,
        threshold,
        labels: label_map,
    }
}

#[tokio::test]
#[ignore]
async fn classifier_accepts_matching_text() {
    let mut categories = BTreeMap::new();

    categories.insert(
        "topic".to_string(),
        make_category(
            vec![("food", 0.3), ("sports", 0.3), ("finance", 0.3)],
            2,
            0.3,
        ),
    );

    let req = EvalRequest {
        request_id: None,
        input: "I love cooking Italian pasta with fresh tomatoes and basil".to_string(),
        scorers: vec![ScorerInput::Classifier(eval::classifier::Input {
            model: classifier_model(),
            top_k: 2,
            weight: 1.0,
            threshold: 0.3,
            categories,
        })],
    };

    let result = classifier_runner().await.evaluate(&req).await;

    assert_eq!(result.scorers.len(), 1);

    if let ScorerOutput::Classifier(o) = &result.scorers[0] {
        println!("score: {}, decision: {:?}", o.score, o.decision);

        for (name, cat) in &o.categories {
            println!(
                "  category {name}: score={}, decision={:?}",
                cat.score, cat.decision
            );

            for (label, lr) in &cat.labels {
                println!(
                    "    label {label}: score={}, decision={:?}",
                    lr.score, lr.decision
                );
            }
        }

        let food_label = o.categories["topic"].labels.get("food").unwrap();

        assert!(
            food_label.score > 0.3,
            "expected food score > 0.3, got {}",
            food_label.score
        );
        assert_eq!(o.decision, Decision::Accept);
    } else {
        panic!("expected classifier output, got {:?}", result.scorers[0]);
    }
}

#[tokio::test]
#[ignore]
async fn classifier_rejects_non_matching_text() {
    let mut categories = BTreeMap::new();

    categories.insert(
        "topic".to_string(),
        make_category(vec![("sports", 0.8)], 1, 0.8),
    );

    let req = EvalRequest {
        request_id: None,
        input: "The stock market crashed today after the Federal Reserve raised interest rates"
            .to_string(),
        scorers: vec![ScorerInput::Classifier(eval::classifier::Input {
            model: classifier_model(),
            top_k: 1,
            weight: 1.0,
            threshold: 0.8,
            categories,
        })],
    };

    let result = classifier_runner().await.evaluate(&req).await;

    if let ScorerOutput::Classifier(o) = &result.scorers[0] {
        println!("score: {}, decision: {:?}", o.score, o.decision);

        for (name, cat) in &o.categories {
            for (label, lr) in &cat.labels {
                println!(
                    "  {name}/{label}: score={}, decision={:?}",
                    lr.score, lr.decision
                );
            }
        }

        assert_eq!(o.decision, Decision::Reject);
    } else {
        panic!("expected classifier output, got {:?}", result.scorers[0]);
    }

    assert_eq!(result.decision, Decision::Reject);
}

#[tokio::test]
#[ignore]
async fn classifier_top_k_selects_best_labels() {
    let mut categories = BTreeMap::new();

    categories.insert(
        "topic".to_string(),
        make_category(
            vec![("food", 0.0), ("sports", 0.0), ("finance", 0.0)],
            1,
            0.0,
        ),
    );

    let req = EvalRequest {
        request_id: None,
        input: "I love cooking Italian pasta".to_string(),
        scorers: vec![ScorerInput::Classifier(eval::classifier::Input {
            model: classifier_model(),
            top_k: 1,
            weight: 1.0,
            threshold: 0.0,
            categories,
        })],
    };

    let result = classifier_runner().await.evaluate(&req).await;

    if let ScorerOutput::Classifier(o) = &result.scorers[0] {
        let cat = &o.categories["topic"];
        let max_label_score = cat.labels.values().map(|l| l.score).fold(0.0f32, f32::max);

        println!("category score (top_k=1): {}", cat.score);

        for (label, lr) in &cat.labels {
            println!("  {label}: score={}", lr.score);
        }

        assert!(
            (cat.score - max_label_score).abs() < 0.01,
            "top_k=1 category score {} should equal best label score {}",
            cat.score,
            max_label_score
        );
    } else {
        panic!("expected classifier output, got {:?}", result.scorers[0]);
    }
}

#[tokio::test]
#[ignore]
async fn classifier_multiple_categories() {
    let mut categories = BTreeMap::new();

    categories.insert(
        "topic".to_string(),
        make_category(vec![("food", 0.3), ("technology", 0.3)], 2, 0.3),
    );
    categories.insert(
        "quality".to_string(),
        make_category(vec![("informative", 0.3), ("gibberish", 0.3)], 2, 0.3),
    );

    let req = EvalRequest {
        request_id: None,
        input: "Italian cuisine uses fresh ingredients like tomatoes, basil, and olive oil"
            .to_string(),
        scorers: vec![ScorerInput::Classifier(eval::classifier::Input {
            model: classifier_model(),
            top_k: 2,
            weight: 1.0,
            threshold: 0.2,
            categories,
        })],
    };

    let result = classifier_runner().await.evaluate(&req).await;

    if let ScorerOutput::Classifier(o) = &result.scorers[0] {
        assert_eq!(o.categories.len(), 2);
        assert!(o.categories.contains_key("topic"));
        assert!(o.categories.contains_key("quality"));

        for (name, cat) in &o.categories {
            println!(
                "category {name}: score={}, decision={:?}",
                cat.score, cat.decision
            );

            for (label, lr) in &cat.labels {
                println!("  {label}: score={}", lr.score);
            }
        }
    } else {
        panic!("expected classifier output, got {:?}", result.scorers[0]);
    }
}

#[tokio::test]
#[ignore]
async fn mixed_classifier_and_judge() {
    let classifier_client = ai::client::Client::new().with_classifier(
        LocalClassifierBuilder::new()
            .build_async()
            .await
            .expect("failed to build classifier"),
    );

    let judge_client = ai::client::Client::new().with_chat(OpenAIClient::new());
    let mut categories = BTreeMap::new();
    let runner = Runner::new()
        .with(classifier_model(), classifier_client)
        .with(judge_model(), judge_client);

    categories.insert(
        "topic".to_string(),
        make_category(vec![("science", 0.3)], 1, 0.3),
    );

    let req = EvalRequest {
        request_id: None,
        input: "Water boils at 100 degrees Celsius at standard atmospheric pressure".to_string(),
        scorers: vec![
            ScorerInput::Classifier(eval::classifier::Input {
                model: classifier_model(),
                top_k: 1,
                weight: 1.0,
                threshold: 0.3,
                categories,
            }),
            judge_input(0.5, vec![("Is the text factually accurate?", 0.5)]),
        ],
    };

    let result = runner.evaluate(&req).await;

    assert_eq!(result.scorers.len(), 2);
    assert!(
        matches!(&result.scorers[0], ScorerOutput::Classifier(_)),
        "expected classifier output for scorer 0"
    );
    assert!(
        matches!(&result.scorers[1], ScorerOutput::Judge(_)),
        "expected judge output for scorer 1"
    );

    for (i, scorer) in result.scorers.iter().enumerate() {
        match scorer {
            ScorerOutput::Classifier(o) => {
                println!(
                    "scorer {i} (classifier): score={}, decision={:?}",
                    o.score, o.decision
                );
            }
            ScorerOutput::Judge(o) => {
                println!(
                    "scorer {i} (judge): score={}, decision={:?}",
                    o.score, o.decision
                );
            }
            ScorerOutput::Error(e) => {
                panic!("scorer {i} errored: {:?}", e);
            }
        }
    }
}
