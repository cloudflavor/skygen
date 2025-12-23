// Copyright 2025 Cloudflavor GmbH

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at

// http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use openapiv3::{OpenAPI, ReferenceOr};
use quote::quote;
use schemars::schema::RootSchema;
use typify::{TypeSpace, TypeSpaceSettings};

pub fn generate_models(openapi: &OpenAPI) -> String {
    let mut type_space = TypeSpace::new(&TypeSpaceSettings::default());

    if let Some(components) = &openapi.components {
        for (name, schema) in &components.schemas {
            let schema = match schema {
                ReferenceOr::Reference { .. } => {
                    // We can't handle references here, so we skip them.
                    continue;
                }
                ReferenceOr::Item(s) => s.clone(),
            };

            let value = match serde_json::to_value(&schema) {
                Ok(v) => v,
                Err(e) => {
                    tracing::warn!("error converting schema to value {}: {}", name, e);
                    continue;
                }
            };
            let schema_object = match serde_json::from_value(value) {
                Ok(v) => v,
                Err(e) => {
                    tracing::warn!("error converting value to schema object {}: {}", name, e);
                    continue;
                }
            };

            let mut root_schema = RootSchema {
                schema: schema_object,
                ..Default::default()
            };
            root_schema.schema.metadata().title = Some(name.clone());

            if let Err(e) = type_space.add_root_schema(root_schema) {
                tracing::warn!("error adding schema {}: {}", name, e);
            }
        }
    }

    let contents = quote! {
        use serde::{Deserialize, Serialize};
        #type_space
    };

    contents.to_string()
}
