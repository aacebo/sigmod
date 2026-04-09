use std::sync::Mutex;

use async_trait::async_trait;
use rust_bert::pipelines::common::{ModelResource, ModelType};
use rust_bert::pipelines::zero_shot_classification::{
    ZeroShotClassificationConfig, ZeroShotClassificationModel,
};
use rust_bert::resources::ResourceProvider;
use tch::{Device, Kind};

use crate::Error;
use crate::client::classify::{ClassificationClient, Label, LabelResult};

pub struct LocalClassifierBuilder {
    config: ZeroShotClassificationConfig,
}

impl LocalClassifierBuilder {
    pub fn new() -> Self {
        Self {
            config: ZeroShotClassificationConfig::default(),
        }
    }

    pub fn with_model_type(mut self, model_type: ModelType) -> Self {
        self.config.model_type = model_type;
        self
    }

    pub fn with_model_resource(mut self, model_resource: ModelResource) -> Self {
        self.config.model_resource = model_resource;
        self
    }

    pub fn with_config_resource(
        mut self,
        config_resource: Box<dyn ResourceProvider + Send>,
    ) -> Self {
        self.config.config_resource = config_resource;
        self
    }

    pub fn with_vocab_resource(mut self, vocab_resource: Box<dyn ResourceProvider + Send>) -> Self {
        self.config.vocab_resource = vocab_resource;
        self
    }

    pub fn with_merges_resource(
        mut self,
        merges_resource: Box<dyn ResourceProvider + Send>,
    ) -> Self {
        self.config.merges_resource = Some(merges_resource);
        self
    }

    pub fn with_device(mut self, device: Device) -> Self {
        self.config.device = device;
        self
    }

    pub fn with_lower_case(mut self, lower_case: bool) -> Self {
        self.config.lower_case = lower_case;
        self
    }

    pub fn with_strip_accents(mut self, strip_accents: bool) -> Self {
        self.config.strip_accents = Some(strip_accents);
        self
    }

    pub fn with_add_prefix_space(mut self, add_prefix_space: bool) -> Self {
        self.config.add_prefix_space = Some(add_prefix_space);
        self
    }

    pub fn with_kind(mut self, kind: Kind) -> Self {
        self.config.kind = Some(kind);
        self
    }

    pub fn build(self) -> Result<LocalClassifierClient, Error> {
        let model = ZeroShotClassificationModel::new(self.config)?;
        Ok(LocalClassifierClient {
            model: Mutex::new(model),
        })
    }
}

impl Default for LocalClassifierBuilder {
    fn default() -> Self {
        Self::new()
    }
}

pub struct LocalClassifierClient {
    model: Mutex<ZeroShotClassificationModel>,
}

#[async_trait]
impl ClassificationClient for LocalClassifierClient {
    async fn predict(
        &self,
        inputs: &[&str],
        labels: &[Label],
        max_length: usize,
    ) -> Result<Vec<Vec<LabelResult>>, Error> {
        let label_names: Vec<&str> = labels.iter().map(|l| l.name.as_str()).collect();
        let model = self.model.lock().unwrap();
        let results = model.predict_multilabel(inputs, &label_names, None, max_length)?;

        Ok(results
            .into_iter()
            .map(|sentence_labels| {
                sentence_labels
                    .into_iter()
                    .map(|l| LabelResult {
                        id: l.id,
                        text: l.text,
                        score: l.score,
                        sentence: l.sentence,
                    })
                    .collect()
            })
            .collect())
    }
}
