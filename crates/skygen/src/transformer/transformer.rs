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

use crate::ir::{ParamLocation, ParamType, Parameter, RequestBody};
use openapiv3::{
    Operation as OApiOperation, Parameter as OApiParam, ReferenceOr, Response as OApiResponse,
    Schema as OApiSchema, SchemaKind, Type as OApiType,
};
use serde_json::{Number, Value as JsonVal};
use serde_yaml::Value as YamlVal;

pub fn fallback_operation_id(method: &str, path: &str) -> String {
    let normalized_path = path
        .trim_matches('/')
        .replace('/', "_")
        .replace('{', "")
        .replace('}', "");
    format!("{}_{}", method, normalized_path)
}

pub fn convert_param(param: openapiv3::Parameter) -> Option<Parameter> {
    let (name, location, required, _schema) = match param {
        OApiParam::Path { parameter_data, .. } => (
            parameter_data.name,
            ParamLocation::Path,
            true,
            parameter_data.format,
        ),
        OApiParam::Query { parameter_data, .. } => (
            parameter_data.name,
            ParamLocation::Query,
            parameter_data.required,
            parameter_data.format,
        ),
        OApiParam::Header { parameter_data, .. } => (
            parameter_data.name,
            ParamLocation::Header,
            parameter_data.required,
            parameter_data.format,
        ),
        _ => return None,
    };

    Some(Parameter {
        name,
        location,
        required,
        // ty: infer_param_type_from_schema(&schema),
    })
}

pub fn extract_body(op: &openapiv3::Operation) -> Option<RequestBody> {
    let body_ref = op.request_body.as_ref()?;

    match body_ref {
        ReferenceOr::Reference { reference } => {
            let name = reference.split('/').last().unwrap_or("Unknown");
            Some(RequestBody {
                ty: ParamType::Object(name.to_string()),
                required: true,
            })
        }

        ReferenceOr::Item(body) => {
            let required = body.required;
            if let Some(content) = body.content.get("application/json") {
                if let Some(schema) = &content.schema {
                    Some(RequestBody {
                        ty: infer_param_type_from_schema(schema),
                        required,
                    })
                } else {
                    None
                }
            } else {
                None
            }
        }
    }
}

pub fn extract_response(op: &OApiOperation) -> ParamType {
    let responses = &op.responses;

    // Ordered preference of response codes
    let candidates = [
        openapiv3::StatusCode::Code(200),
        openapiv3::StatusCode::Code(201),
    ];

    for code in candidates {
        if let Some(response) = responses.responses.get(&code) {
            return extract_response_type_from_entry(response);
        }
    }
    if let Some(response) = &responses.default {
        return extract_response_type_from_entry(response);
    }

    // No known good response found
    ParamType::Unknown
}

fn extract_response_type_from_entry(entry: &ReferenceOr<OApiResponse>) -> ParamType {
    match entry {
        ReferenceOr::Reference { reference } => {
            let name = reference.split('/').last().unwrap_or("Unknown");
            ParamType::Object(name.to_string())
        }

        ReferenceOr::Item(response) => {
            if let Some(media) = response.content.get("application/json") {
                if let Some(schema) = &media.schema {
                    infer_param_type_from_schema(schema)
                } else {
                    ParamType::Unknown
                }
            } else {
                ParamType::Unknown
            }
        }
    }
}

pub fn infer_param_type_from_schema(schema_or_ref: &ReferenceOr<OApiSchema>) -> ParamType {
    match schema_or_ref {
        ReferenceOr::Reference { reference } => {
            let name = reference.split('/').last().unwrap_or("Unknown");
            ParamType::Object(name.to_string())
        }
        ReferenceOr::Item(schema) => match &schema.schema_kind {
            SchemaKind::Type(OApiType::String(_)) => ParamType::String,
            SchemaKind::Type(OApiType::Integer(_)) => ParamType::Integer,
            SchemaKind::Type(OApiType::Number(_)) => ParamType::Float,
            SchemaKind::Type(OApiType::Boolean(_)) => ParamType::Boolean,
            SchemaKind::Type(OApiType::Array(array)) => {
                if let Some(items) = &array.items {
                    let mapped = match items {
                        ReferenceOr::Reference { reference } => ReferenceOr::Reference {
                            reference: reference.clone(),
                        },
                        ReferenceOr::Item(schema) => ReferenceOr::Item((**schema).clone()),
                    };
                    ParamType::Array(Box::new(infer_param_type_from_schema(&mapped)))
                } else {
                    ParamType::Array(Box::new(ParamType::Unknown))
                }
            }
            SchemaKind::Type(OApiType::Object(_obj)) => {
                if let Some(title) = &schema.schema_data.title {
                    ParamType::Object(title.clone())
                } else {
                    ParamType::Map(Box::new(ParamType::Unknown))
                }
            }
            _ => ParamType::Unknown,
        },
    }
}

pub fn fix_json_large_numbers(val: JsonVal) -> JsonVal {
    match val {
        JsonVal::Number(n) => {
            let n_str = n.to_string();
            match n_str.parse::<u128>() {
                Ok(parsed) if parsed > u64::MAX as u128 => JsonVal::Number(Number::from(u64::MAX)),
                Ok(parsed) => JsonVal::Number(Number::from(parsed as u64)),
                Err(_) => JsonVal::Number(Number::from(u64::MAX)), // fallback
            }
        }
        JsonVal::String(s) => JsonVal::String(s),
        JsonVal::Array(arr) => {
            JsonVal::Array(arr.into_iter().map(fix_json_large_numbers).collect())
        }
        JsonVal::Object(obj) => {
            let fixed = obj
                .into_iter()
                .map(|(k, v)| (k, fix_json_large_numbers(v)))
                .collect();
            JsonVal::Object(fixed)
        }
        other => other,
    }
}

pub fn fix_yaml_large_numbers(val: YamlVal) -> YamlVal {
    match val {
        serde_yaml::Value::Number(n) => {
            let n_str = n.to_string();
            match n_str.parse::<u128>() {
                Ok(parsed) if parsed > u64::MAX as u128 => {
                    serde_yaml::Value::Number(serde_yaml::Number::from(u64::MAX))
                }
                Ok(parsed) => serde_yaml::Value::Number(serde_yaml::Number::from(parsed as u64)),
                Err(_) => {
                    // Fallback: something weird, just replace with u64::MAX
                    serde_yaml::Value::Number(serde_yaml::Number::from(u64::MAX))
                }
            }
        }
        serde_yaml::Value::String(s) => serde_yaml::Value::String(s),
        serde_yaml::Value::Sequence(seq) => {
            serde_yaml::Value::Sequence(seq.into_iter().map(fix_yaml_large_numbers).collect())
        }
        serde_yaml::Value::Mapping(map) => {
            let fixed = map
                .into_iter()
                .map(|(k, v)| (k, fix_yaml_large_numbers(v)))
                .collect();
            serde_yaml::Value::Mapping(fixed)
        }
        other => other,
    }
}

pub struct RefResolver<'a> {
    pub components: &'a openapiv3::Components,
}

impl<'a> RefResolver<'a> {
    pub fn resolve_schema(&self, reference: &str) -> Option<&OApiSchema> {
        let key = reference.strip_prefix("#/components/schemas/")?;
        match self.components.schemas.get(key)? {
            ReferenceOr::Item(schema) => Some(schema),
            _ => None,
        }
    }

    pub fn resolve_response(&self, reference: &str) -> Option<&OApiResponse> {
        let key = reference.strip_prefix("#/components/responses/")?;
        match self.components.responses.get(key)? {
            ReferenceOr::Item(resp) => Some(resp),
            _ => None,
        }
    }

    pub fn resolve_parameter(&self, reference: &str) -> Option<&OApiParam> {
        let key = reference.strip_prefix("#/components/parameters/")?;
        match self.components.parameters.get(key)? {
            ReferenceOr::Item(p) => Some(p),
            _ => None,
        }
    }
}
