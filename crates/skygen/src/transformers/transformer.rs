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
    Operation as OApiOperation, Parameter as OApiParam, ReferenceOr,
    RequestBody as OApiRequestBody, Response as OApiResponse, Schema as OApiSchema, SchemaKind,
    Type as OApiType,
};
use serde_json::{Number, Value as JsonVal};
use serde_yaml::Value as YamlVal;

#[derive(Clone, Copy, Debug)]
enum NumericHint {
    Integer,
    Number,
}

#[derive(Clone, Copy, Debug)]
enum NumericValueKind {
    Signed,
    Unsigned,
    Float,
}

pub fn fallback_operation_id(method: &str, path: &str) -> String {
    let normalized_path = path
        .trim_matches('/')
        .replace('/', "_")
        .replace(['{', '}'], "");
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

pub fn extract_body(
    op: &openapiv3::Operation,
    resolver: Option<&RefResolver<'_>>,
) -> Option<RequestBody> {
    let body_ref = op.request_body.as_ref()?;

    match body_ref {
        ReferenceOr::Reference { reference } => {
            if let Some(resolver) = resolver {
                if let Some(body) = resolver.resolve_request_body(reference) {
                    return build_request_body(body);
                }
            }
            Some(RequestBody {
                ty: ParamType::Object(reference_name(reference).to_string()),
                required: true,
            })
        }

        ReferenceOr::Item(body) => build_request_body(body),
    }
}

fn build_request_body(body: &OApiRequestBody) -> Option<RequestBody> {
    let required = body.required;
    if let Some(content) = body.content.get("application/json") {
        if let Some(schema) = &content.schema {
            return Some(RequestBody {
                ty: infer_param_type_from_schema(schema),
                required,
            });
        }
    }
    None
}

pub fn extract_response(op: &OApiOperation, resolver: Option<&RefResolver<'_>>) -> ParamType {
    let responses = &op.responses;

    // Ordered preference of response codes
    let candidates = [
        openapiv3::StatusCode::Code(200),
        openapiv3::StatusCode::Code(201),
    ];

    for code in candidates {
        if let Some(response) = responses.responses.get(&code) {
            return extract_response_type_from_entry(response, resolver);
        }
    }
    if let Some(response) = &responses.default {
        return extract_response_type_from_entry(response, resolver);
    }

    // No known good response found
    ParamType::Unknown
}

fn extract_response_type_from_entry(
    entry: &ReferenceOr<OApiResponse>,
    resolver: Option<&RefResolver<'_>>,
) -> ParamType {
    match entry {
        ReferenceOr::Reference { reference } => {
            if let Some(resolver) = resolver {
                if let Some(resp) = resolver.resolve_response(reference) {
                    return extract_response_from_media(resp);
                }
            }
            ParamType::Object(reference_name(reference).to_string())
        }

        ReferenceOr::Item(response) => extract_response_from_media(response),
    }
}

fn extract_response_from_media(response: &OApiResponse) -> ParamType {
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

pub fn infer_param_type_from_schema(schema_or_ref: &ReferenceOr<OApiSchema>) -> ParamType {
    match schema_or_ref {
        ReferenceOr::Reference { reference } => {
            ParamType::Object(reference_name(reference).to_string())
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
            SchemaKind::AllOf { all_of } => infer_from_composed(all_of),
            SchemaKind::AnyOf { any_of } => infer_from_composed(any_of),
            SchemaKind::OneOf { one_of } => infer_from_composed(one_of),
            SchemaKind::Any(any_schema) => {
                if !any_schema.all_of.is_empty() {
                    let ty = infer_from_composed(&any_schema.all_of);
                    if !matches!(ty, ParamType::Unknown) {
                        return ty;
                    }
                }
                if !any_schema.one_of.is_empty() {
                    let ty = infer_from_composed(&any_schema.one_of);
                    if !matches!(ty, ParamType::Unknown) {
                        return ty;
                    }
                }
                if !any_schema.any_of.is_empty() {
                    let ty = infer_from_composed(&any_schema.any_of);
                    if !matches!(ty, ParamType::Unknown) {
                        return ty;
                    }
                }
                if let Some(title) = &schema.schema_data.title {
                    return ParamType::Object(title.clone());
                }
                if matches!(any_schema.typ.as_deref(), Some("object")) {
                    ParamType::Map(Box::new(ParamType::Unknown))
                } else {
                    ParamType::Unknown
                }
            }
            _ => ParamType::Unknown,
        },
    }
}

fn infer_from_composed(parts: &[ReferenceOr<OApiSchema>]) -> ParamType {
    let mut parts_iter = parts.iter().peekable();
    while let Some(part) = parts_iter.next() {
        if let ReferenceOr::Reference { reference } = part {
            if reference_name(reference) == "rulesets_Response" && parts_iter.peek().is_some() {
                continue;
            }
        }
        let ty = infer_param_type_from_schema(part);
        if !matches!(ty, ParamType::Unknown) {
            return ty;
        }
    }
    ParamType::Unknown
}

fn reference_name(reference: &str) -> &str {
    reference.rsplit('/').next().unwrap_or(reference)
}

pub fn fix_json_large_numbers(mut val: JsonVal) -> JsonVal {
    normalize_json_value(&mut val, None, None);
    val
}

fn normalize_json_value(
    value: &mut JsonVal,
    parent_key: Option<&str>,
    numeric_hint: Option<NumericHint>,
) {
    match value {
        JsonVal::Number(num) => {
            if let Some(kind) = numeric_kind_for_key(parent_key, numeric_hint) {
                clamp_json_number_in_place(num, kind);
            }
        }
        JsonVal::String(s) => {
            if let Some(kind) = numeric_kind_for_key(parent_key, numeric_hint) {
                if let Some(new_val) = parse_string_into_json_number(s, kind) {
                    *value = new_val;
                }
            }
        }
        JsonVal::Array(items) => {
            for item in items.iter_mut() {
                normalize_json_value(item, parent_key, numeric_hint);
            }
        }
        JsonVal::Object(map) => {
            let (has_type_field, local_hint) = infer_numeric_hint_from_map(map);
            let next_hint = local_hint.or(if has_type_field { None } else { numeric_hint });

            for (key, val) in map.iter_mut() {
                normalize_json_value(val, Some(key.as_str()), next_hint);
            }
        }
        _ => {}
    }
}

fn clamp_json_number_in_place(num: &mut Number, kind: NumericValueKind) {
    match kind {
        NumericValueKind::Signed => {
            if let Some(value) = num.as_i64() {
                *num = Number::from(value);
            } else if let Some(value) = num.as_u64() {
                let clamped = value.min(i64::MAX as u64) as i64;
                *num = Number::from(clamped);
            } else if let Some(value) = num.as_f64() {
                let clamped = value.clamp(i64::MIN as f64, i64::MAX as f64);
                if let Some(new_num) = Number::from_f64(clamped) {
                    *num = new_num;
                }
            }
        }
        NumericValueKind::Unsigned => {
            if let Some(value) = num.as_u64() {
                let upper = usize::MAX as u64;
                *num = Number::from(value.min(upper));
            } else if let Some(value) = num.as_i64() {
                if value.is_negative() {
                    *num = Number::from(0u64);
                } else {
                    let upper = usize::MAX as i64;
                    *num = Number::from(value.min(upper) as u64);
                }
            } else if let Some(value) = num.as_f64() {
                let clamped = value.clamp(0.0, usize::MAX as f64);
                if let Some(new_num) = Number::from_f64(clamped) {
                    *num = new_num;
                }
            }
        }
        NumericValueKind::Float => {
            if let Some(value) = num.as_f64() {
                let clamped = value.clamp(f64::MIN, f64::MAX);
                if let Some(new_num) = Number::from_f64(clamped) {
                    *num = new_num;
                }
            } else if let Some(value) = num.as_i64() {
                if let Some(new_num) = Number::from_f64(value as f64) {
                    *num = new_num;
                }
            } else if let Some(value) = num.as_u64() {
                if let Some(new_num) = Number::from_f64(value as f64) {
                    *num = new_num;
                }
            }
        }
    }
}

fn parse_string_into_json_number(input: &str, kind: NumericValueKind) -> Option<JsonVal> {
    match kind {
        NumericValueKind::Signed => {
            parse_string_to_i64(input).map(|v| JsonVal::Number(Number::from(v)))
        }
        NumericValueKind::Unsigned => parse_string_to_unsigned(input)
            .map(|v| JsonVal::Number(Number::from(v.min(usize::MAX as u128) as u64))),
        NumericValueKind::Float => parse_string_to_f64(input)
            .and_then(Number::from_f64)
            .map(JsonVal::Number),
    }
}

fn parse_string_to_i64(input: &str) -> Option<i64> {
    let value = input.trim().parse::<i128>().ok()?;
    let clamped = value.clamp(i64::MIN as i128, i64::MAX as i128);
    Some(clamped as i64)
}

fn parse_string_to_unsigned(input: &str) -> Option<u128> {
    let value = input.trim().parse::<i128>().ok()?;
    if value.is_negative() {
        return Some(0);
    }
    Some(value as u128)
}

fn parse_string_to_f64(input: &str) -> Option<f64> {
    let value = input.trim().parse::<f64>().ok()?;
    if value.is_finite() {
        Some(value)
    } else {
        None
    }
}

fn numeric_kind_for_key(key: Option<&str>, hint: Option<NumericHint>) -> Option<NumericValueKind> {
    match key {
        Some("maximum") | Some("minimum") | Some("multipleOf") | Some("enum") | Some("default")
        | Some("example") => match hint {
            Some(NumericHint::Integer) => {
                Some(NumericValueKind::Signed)
            }
            Some(NumericHint::Number) => Some(NumericValueKind::Float),
            None => None,
        },
        Some("maxLength")
        | Some("minLength")
        | Some("maxItems")
        | Some("minItems")
        | Some("maxProperties")
        | Some("minProperties") => Some(NumericValueKind::Unsigned),
        _ => None,
    }
}

fn infer_numeric_hint_from_map(
    map: &serde_json::Map<String, JsonVal>,
) -> (bool, Option<NumericHint>) {
    if let Some(ty) = map.get("type") {
        if let Some(type_str) = ty.as_str() {
            return match type_str {
                "integer" => (true, Some(NumericHint::Integer)),
                "number" => (true, Some(NumericHint::Number)),
                _ => (true, None),
            };
        }
        return (true, None);
    }
    (false, None)
}

pub fn fix_yaml_large_numbers(val: YamlVal) -> YamlVal {
    match val {
        YamlVal::Number(num) => YamlVal::Number(normalize_yaml_number(num)),
        YamlVal::Sequence(seq) => {
            YamlVal::Sequence(seq.into_iter().map(fix_yaml_large_numbers).collect())
        }
        YamlVal::Mapping(map) => {
            let normalized = map
                .into_iter()
                .map(|(k, v)| (k, fix_yaml_large_numbers(v)))
                .collect();
            YamlVal::Mapping(normalized)
        }
        other => other,
    }
}

fn normalize_yaml_number(num: serde_yaml::Number) -> serde_yaml::Number {
    if let Some(value) = num.as_i64() {
        serde_yaml::Number::from(value)
    } else if let Some(value) = num.as_u64() {
        serde_yaml::Number::from(value)
    } else if let Some(value) = num.as_f64() {
        serde_yaml::Number::from(value)
    } else {
        let as_str = num.to_string();
        if let Some(parsed) = parse_string_to_f64(&as_str) {
            serde_yaml::Number::from(parsed)
        } else {
            serde_yaml::Number::from(0)
        }
    }
}

pub fn clamp_overflowing_numeric_literals(input: &str) -> Option<String> {
    let bytes = input.as_bytes();
    let mut cursor = 0usize;
    let mut last_emit = 0usize;
    let mut changed = false;
    let len = bytes.len();
    let mut sanitized = String::with_capacity(input.len());

    while cursor < len {
        let byte = bytes[cursor];
        if is_number_lead(byte) {
            let start = cursor;
            let mut idx = cursor;
            if matches!(byte, b'+' | b'-') {
                idx += 1;
                if idx >= len || !bytes[idx].is_ascii_digit() {
                    cursor += 1;
                    continue;
                }
            }
            let digits_start = idx;
            while idx < len && bytes[idx].is_ascii_digit() {
                idx += 1;
            }
            if digits_start == idx {
                cursor += 1;
                continue;
            }
            if matches!(bytes.get(idx), Some(b'.' | b'e' | b'E')) {
                cursor += 1;
                continue;
            }
            if let Some(prev) = previous_non_space(bytes, start) {
                if !matches!(prev, b':' | b',' | b'[' | b'{' | b'-') {
                    cursor += 1;
                    continue;
                }
            }
            let token = &input[start..idx];
            if let Some(clamped) = clamp_numeric_literal(token) {
                sanitized.push_str(&input[last_emit..start]);
                sanitized.push_str(&clamped);
                last_emit = idx;
                changed = true;
            }
            cursor = idx;
            continue;
        }
        cursor += 1;
    }

    if changed {
        sanitized.push_str(&input[last_emit..]);
        Some(sanitized)
    } else {
        None
    }
}

fn previous_non_space(bytes: &[u8], start: usize) -> Option<u8> {
    if start == 0 {
        return None;
    }
    let mut idx = start;
    while idx > 0 {
        idx -= 1;
        let ch = bytes[idx];
        if ch.is_ascii_whitespace() {
            continue;
        }
        return Some(ch);
    }
    None
}

fn is_number_lead(byte: u8) -> bool {
    byte.is_ascii_digit() || matches!(byte, b'+' | b'-')
}

fn clamp_numeric_literal(token: &str) -> Option<String> {
    let mut cleaned = String::with_capacity(token.len());
    for ch in token.chars() {
        if ch != '_' {
            cleaned.push(ch);
        }
    }
    if cleaned.is_empty() {
        return None;
    }
    let value = cleaned.parse::<i128>().ok();
    let min = i64::MIN as i128;
    let max = i64::MAX as i128;
    match value {
        Some(val) if val < min => Some(i64::MIN.to_string()),
        Some(val) if val > max => Some(i64::MAX.to_string()),
        Some(_) => None,
        None => {
            if cleaned.starts_with('-') {
                Some(i64::MIN.to_string())
            } else {
                Some(i64::MAX.to_string())
            }
        }
    }
}

pub struct RefResolver<'a> {
    pub components: &'a openapiv3::Components,
}

impl<'a> RefResolver<'a> {
    pub fn new(components: &'a openapiv3::Components) -> Self {
        Self { components }
    }

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

    pub fn resolve_request_body(&self, reference: &str) -> Option<&OApiRequestBody> {
        let key = reference.strip_prefix("#/components/requestBodies/")?;
        match self.components.request_bodies.get(key)? {
            ReferenceOr::Item(body) => Some(body),
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
