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
