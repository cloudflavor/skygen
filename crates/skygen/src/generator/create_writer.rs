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

use crate::deserialize_data;
use crate::ir::{
    Function, FunctionParam, HttpMethod, Operation, ParamLocation, ParamType, PathParam,
    RequestBodyConfig,
};
use crate::transformer::{
    convert_param, extract_body, extract_response, fallback_operation_id,
    infer_param_type_from_schema, RefResolver,
};
use crate::TEMPLATES;
use anyhow::{ensure, Context, Result};
use heck::{ToSnakeCase, ToUpperCamelCase};
use openapiv3::{Components, Info as OApiInfo, OpenAPI, Paths, ReferenceOr, SchemaKind};
use semver::Version;
use serde::Serialize;
use std::collections::btree_map::Entry;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::path::Path;
use tokio::process::Command;

pub async fn generate(schema: impl AsRef<Path>, output_dir: impl AsRef<Path>) -> Result<()> {
    match tokio::fs::read_to_string(schema).await {
        Ok(content) => {
            let openapi: OpenAPI = deserialize_data(content.as_str()).await?;
            let component_ref = openapi.components.as_ref();
            let reqs = walk_paths(openapi.paths, component_ref).await?;
            generate_lib(reqs, output_dir, openapi.info, component_ref).await?;
            Ok(())
        }
        Err(e) => Err(e.into()),
    }
}
fn generate_function_name(
    method: &str,
    operation_id: &str,
    path: &str,
    module_tokens: &[String],
) -> String {
    let mut parts: Vec<String> = operation_id
        .to_snake_case()
        .split('_')
        .filter_map(|segment| {
            let trimmed = segment.trim();
            if trimmed.is_empty() {
                None
            } else {
                Some(trimmed.to_string())
            }
        })
        .collect();

    if let Some(first) = parts.first() {
        if first == method {
            parts.remove(0);
        }
    }

    if let Some(idx) = parts.iter().position(|part| part == "by") {
        parts.truncate(idx);
    }

    parts = parts
        .into_iter()
        .enumerate()
        .filter_map(|(idx, part)| {
            if should_drop_token(&part, idx) {
                None
            } else {
                Some(part)
            }
        })
        .collect();
    parts.dedup();

    if parts.is_empty() {
        parts = path
            .trim_matches('/')
            .split('/')
            .filter(|seg| !seg.starts_with('{'))
            .map(|seg| seg.to_snake_case())
            .map(|seg| seg.trim_matches('_').to_string())
            .collect();
    }

    parts = trim_module_prefix(parts, module_tokens);

    if parts.is_empty() {
        parts.push(method.to_lowercase());
    }

    if parts.len() > 4 {
        parts.truncate(4);
    }

    parts.join("_")
}

fn should_drop_token(token: &str, position: usize) -> bool {
    let normalized = token.trim_matches('_');
    if normalized.is_empty() {
        return true;
    }
    let structural = matches!(
        normalized,
        "a" | "an"
            | "the"
            | "by"
            | "for"
            | "of"
            | "to"
            | "and"
            | "or"
            | "with"
            | "from"
            | "in"
            | "on"
            | "via"
    );
    let identifier_tokens = matches!(
        normalized,
        "id" | "ids" | "identifier" | "account" | "accounts" | "zone" | "zones" | "user" | "users"
    );
    (structural && position > 0)
        || (identifier_tokens && position > 0)
        || (normalized.len() == 1 && !normalized.chars().any(|c| c.is_alphabetic()))
}

fn trim_module_prefix(parts: Vec<String>, module_tokens: &[String]) -> Vec<String> {
    let mut tokens = parts;
    loop {
        let remove = tokens
            .first()
            .map(|part| module_tokens.iter().any(|module| module == part))
            .unwrap_or(false);
        if remove && tokens.len() > 1 {
            tokens.remove(0);
        } else {
            break;
        }
    }

    if tokens.iter().all(|part| module_tokens.contains(part)) {
        tokens.clear();
    }

    if let Some(last) = tokens.last() {
        if last == "a" || last == "an" {
            tokens.pop();
        }
    }

    tokens
}

fn extract_path_params(path: &str) -> Vec<PathParam> {
    let mut params = Vec::new();
    let mut seen = HashSet::new();
    let mut start = None;
    for (idx, ch) in path.chars().enumerate() {
        match (ch, start) {
            ('{', None) => start = Some(idx + 1),
            ('}', Some(begin)) if begin < idx => {
                let wire = path[begin..idx].to_string();
                if !seen.insert(wire.clone()) {
                    start = None;
                    continue;
                }
                let setter = sanitize_method_suffix(&wire);
                params.push(PathParam {
                    wire_name: wire,
                    setter,
                });
                start = None;
            }
            _ => {}
        }
    }
    params
}

pub async fn walk_paths(
    paths: Paths,
    components: Option<&Components>,
) -> Result<BTreeMap<String, Vec<Operation>>> {
    let mut grouped: BTreeMap<String, Vec<Operation>> = BTreeMap::new();
    let resolver = components.map(RefResolver::new);

    for (path, item) in paths.paths {
        if let Some(item) = item.into_item() {
            let shared_params: Vec<_> = item
                .parameters
                .into_iter()
                .filter_map(|p| p.into_item())
                .collect();

            for (method_str, op_opt) in [
                ("get", item.get),
                ("post", item.post),
                ("put", item.put),
                ("delete", item.delete),
                ("patch", item.patch),
                ("options", item.options),
            ] {
                if let Some(op) = op_opt {
                    let mut all_params = shared_params.clone();
                    let params = op.parameters.clone();
                    all_params.extend(params.into_iter().filter_map(|p| p.into_item()));

                    let params = all_params
                        .into_iter()
                        .filter_map(convert_param)
                        .collect::<Vec<_>>();

                    let method = match method_str {
                        "get" => HttpMethod::Get,
                        "post" => HttpMethod::Post,
                        "put" => HttpMethod::Put,
                        "delete" => HttpMethod::Delete,
                        "patch" => HttpMethod::Patch,
                        "options" => HttpMethod::Options,
                        _ => continue,
                    };

                    let id = op
                        .operation_id
                        .clone()
                        .unwrap_or_else(|| fallback_operation_id(method_str, &path));

                    let op_tags = op.tags.clone();
                    let tag = op_tags
                        .first()
                        .map(|tag| tag.to_snake_case())
                        .unwrap_or_else(|| "untagged".into());

                    let operation = Operation {
                        id,
                        summary: op.summary.clone(),
                        description: op.description.clone(),
                        method,
                        path: path.clone(),
                        tags: op_tags,
                        params,
                        request_body: extract_body(&op, resolver.as_ref()),
                        response: extract_response(&op, resolver.as_ref()),
                    };

                    grouped.entry(tag).or_default().push(operation);
                }
            }
        }
    }

    Ok(grouped)
}

async fn generate_lib(
    reqs: BTreeMap<String, Vec<Operation>>,
    path: impl AsRef<Path>,
    api_info: OApiInfo,
    components: Option<&Components>,
) -> Result<()> {
    let output_root = path.as_ref().to_path_buf();
    let src_dir = output_root.join("src");
    let apis_dir = src_dir.join("apis");
    let models_dir = src_dir.join("models");

    tokio::fs::create_dir_all(&apis_dir).await?;
    tokio::fs::create_dir_all(&models_dir).await?;

    let mut api_modules: Vec<String> = Vec::new();

    let model_result = if let Some(components) = components {
        generate_models(&models_dir, components).await?
    } else {
        ModelGenerationResult::default()
    };
    let OApiInfo {
        title,
        description,
        version,
        ..
    } = api_info;
    let crate_version = sanitize_crate_version(&version);
    let sample_metadata = reqs.iter().next().map(|(tag, ops)| {
        let module_tokens: Vec<String> = tag
            .to_lowercase()
            .to_snake_case()
            .split('_')
            .map(|s| s.to_string())
            .collect();
        let func = ops
            .first()
            .map(|op| {
                generate_function_name(
                    &op.method.to_string().to_lowercase(),
                    &op.id,
                    &op.path,
                    &module_tokens,
                )
            })
            .unwrap_or_else(|| "health_check".to_string());
        (module_tokens, func)
    });
    let crate_name_slug = sanitize_crate_name(title.as_str());
    let sample_function_name = sample_metadata
        .as_ref()
        .map(|(_, func)| func.clone())
        .unwrap_or_else(|| "health_check".to_string());

    for (tag, ops) in reqs.into_iter() {
        let module_name = sanitize_module_name(&tag);
        let mut module_functions: Vec<Function> = Vec::new();
        let mut model_usage: BTreeSet<ModelUsage> = BTreeSet::new();
        let mut name_tracker: HashMap<String, usize> = HashMap::new();
        let module_tokens: Vec<String> = module_name
            .split('_')
            .map(|segment| segment.to_string())
            .collect();

        for op in ops {
            let method = op.method.to_string().to_lowercase();
            let base_name = generate_function_name(
                method.as_str(),
                op.id.as_str(),
                op.path.as_str(),
                &module_tokens,
            );
            let clean_name = uniquify_name(base_name, method.as_str(), &mut name_tracker);

            let normalized_path = op.path.trim_start_matches('/').to_string();
            let path_placeholders = extract_path_params(&normalized_path);
            let path_setters: HashSet<String> =
                path_placeholders.iter().map(|p| p.setter.clone()).collect();
            let description_lines = format_description_lines(op.description.as_deref());

            let mut setter_tracker: HashMap<String, usize> = HashMap::new();
            let params: Vec<FunctionParam> = op
                .params
                .into_iter()
                .map(|param| {
                    let snake = param.name.to_snake_case();
                    let param_name = sanitize_identifier(&snake);
                    let mut setter = sanitize_method_suffix(&snake);
                    if param.location != ParamLocation::Path && path_setters.contains(&setter) {
                        setter = format!(
                            "{}_{}",
                            setter,
                            match param.location {
                                ParamLocation::Header => "header",
                                ParamLocation::Query => "query",
                                ParamLocation::Path => "path",
                            }
                        );
                    }
                    let base = setter.clone();
                    let count = setter_tracker.entry(base.clone()).or_insert(0);
                    if *count > 0 {
                        setter = format!("{}_{}", base, *count + 1);
                    }
                    *count += 1;
                    FunctionParam {
                        name: param_name,
                        setter,
                        wire_name: param.name.clone(),
                        required: param.required,
                        location: param.location,
                    }
                })
                .filter(|param| !matches!(param.location, ParamLocation::Path))
                .collect();

            let return_type = param_type_to_rust(
                &op.response,
                Some(&model_result.module_name_map),
                Some(&model_result.module_set),
            );
            track_model_usage(
                &op.response,
                &mut model_usage,
                &model_result.module_name_map,
            );
            let request_body = op.request_body.as_ref().map(|body| RequestBodyConfig {
                ty: param_type_to_rust(
                    &body.ty,
                    Some(&model_result.module_name_map),
                    Some(&model_result.module_set),
                ),
                required: body.required,
            });
            let method_name = sanitize_identifier(&clean_name);
            let builder_struct = format!("{}Request", sanitize_struct_name(&method_name));
            let fn_name = Function {
                name: method_name,
                builder_struct,
                summary: op.summary.clone(),
                description_lines,
                params,
                path_params: path_placeholders,
                return_type,
                method: op.method.to_string(),
                path: normalized_path,
                path_display: op.path.clone(),
                request_body,
            };

            module_functions.push(fn_name);
        }

        apply_model_aliases(
            &mut module_functions,
            &model_usage,
            &model_result.module_set,
        );

        let module_template = TEMPLATES
            .get_file("operation.rs.tera")
            .with_context(|| "failed to open operations template")?;
        let contents = module_template
            .contents_utf8()
            .with_context(|| "failed to read template")?;
        let mut tera = tera::Tera::default();
        tera.add_raw_template("operations.rs", contents)
            .with_context(|| "failed to load module template")?;
        let mut context = tera::Context::new();
        let uses: Vec<_> = model_usage
            .into_iter()
            .filter(|usage| model_result.module_set.contains(&usage.module))
            .collect();
        context.insert("crate_name", &crate_name_slug);
        context.insert("module_name", &module_name);
        context.insert("functions", &module_functions);
        context.insert("model_uses", &uses);
        let render = tera
            .render("operations.rs", &context)
            .with_context(|| "failed to render module template")?;

        tokio::fs::write(apis_dir.join(format!("{module_name}.rs")), render)
            .await
            .with_context(|| format!("failed to write module {module_name}"))?;

        api_modules.push(module_name);
    }

    render_mod_file(&api_modules, apis_dir.join("mod.rs")).await?;
    render_mod_file(&model_result.modules, models_dir.join("mod.rs")).await?;
    let crate_title = title.clone();
    let mut docs = format!("/// {} API version: {}\n", crate_title, version);

    if let Some(desc) = description {
        for row in desc.lines() {
            if row.trim().is_empty() {
                docs.push_str("///\n");
            } else {
                docs.push_str(format!("/// {row}\n").as_str());
            }
        }
    }

    let lib_template = TEMPLATES
        .get_file("lib.rs.tera")
        .with_context(|| "failed to retrieve lib template")?;
    let contents = lib_template
        .contents_utf8()
        .with_context(|| "failed to read lib template")?;
    let mut tera = tera::Tera::default();
    tera.add_raw_template("lib.rs", contents)
        .with_context(|| "failed to load lib template")?;
    let mut context = tera::Context::new();
    let lib_modules = vec!["apis".to_string(), "models".to_string()];
    let docs_block = docs.trim_end().to_string();
    let sample_module = api_modules
        .first()
        .cloned()
        .unwrap_or_else(|| "apis".to_string());
    let sample_function = sample_function_name;
    context.insert("crate_name", &crate_name_slug);
    context.insert("sample_module", &sample_module);
    context.insert("sample_function", &sample_function);
    context.insert("docs", &docs_block);
    context.insert("modules", &lib_modules);
    let render = tera
        .render("lib.rs", &context)
        .with_context(|| "failed to render lib template")?;

    tokio::fs::write(src_dir.join("lib.rs"), render)
        .await
        .with_context(|| "failed to write lib.rs")?;

    create_rust_project(title, crate_version, &output_root).await?;
    if let Err(err) = run_cargo_tasks(&output_root).await {
        tracing::warn!("cargo post-processing failed: {err}");
    }

    Ok(())
}

async fn render_mod_file(modules: &[String], destination: impl AsRef<Path>) -> Result<()> {
    let mod_template = TEMPLATES
        .get_file("mod.rs.tera")
        .with_context(|| "failed to retrieve mod template")?;
    let contents = mod_template
        .contents_utf8()
        .with_context(|| "failed to read mod template")?;
    let mut tera = tera::Tera::default();
    tera.add_raw_template("mod.rs", contents)
        .with_context(|| "failed to load mod template")?;
    let mut context = tera::Context::new();
    context.insert("modules", modules);
    let render = tera
        .render("mod.rs", &context)
        .with_context(|| "failed to render mod template")?;
    tokio::fs::write(destination, render)
        .await
        .with_context(|| "failed to write mod.rs file")?;
    Ok(())
}

async fn run_cargo_tasks(path: &Path) -> Result<()> {
    run_cargo_command(path, &["fmt"]).await?;
    run_cargo_command(path, &["check"]).await?;
    Ok(())
}

async fn run_cargo_command(path: &Path, args: &[&str]) -> Result<()> {
    let status = Command::new("cargo")
        .args(args)
        .current_dir(path)
        .status()
        .await
        .with_context(|| format!("failed to run cargo {}", args.join(" ")))?;
    ensure!(
        status.success(),
        "cargo {} failed with status {status}",
        args.join(" ")
    );
    Ok(())
}

#[derive(Serialize)]
struct ModelField {
    name: String,
    ty: String,
    required: bool,
    flatten: bool,
}

struct ModelTemplate {
    module: String,
    name: String,
    fields: Vec<ModelField>,
    alias: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
struct ModelUsage {
    module: String,
    ty: String,
}

#[derive(Default)]
struct ModelGenerationResult {
    modules: Vec<String>,
    module_set: HashSet<String>,
    module_name_map: HashMap<String, String>,
}

async fn generate_models(
    models_dir: &Path,
    components: &Components,
) -> Result<ModelGenerationResult> {
    let template_file = TEMPLATES
        .get_file("model.rs.tera")
        .with_context(|| "failed to retrieve model template")?;
    let contents = template_file
        .contents_utf8()
        .with_context(|| "failed to read model template")?;
    let mut tera = tera::Tera::default();
    tera.add_raw_template("model.rs", contents)
        .with_context(|| "failed to load model template")?;

    let resolver = RefResolver::new(components);
    let module_name_map = build_schema_module_name_map(components);
    let struct_modules = collect_struct_modules(components, &resolver, &module_name_map);
    let available_modules: HashSet<String> =
        module_name_map.values().cloned().collect::<HashSet<_>>();

    let mut templates = Vec::new();
    for (raw_name, schema_or_ref) in components.schemas.iter() {
        let module_name = module_name_map
            .get(raw_name)
            .cloned()
            .unwrap_or_else(|| sanitize_module_name(raw_name));
        let struct_name = raw_name.to_upper_camel_case();
        if struct_modules.contains(&module_name) {
            let mut visited = BTreeSet::new();
            if let Some(mut fields) = collect_model_fields(
                schema_or_ref,
                &resolver,
                &module_name_map,
                &struct_modules,
                &available_modules,
                &mut visited,
            ) {
                fields.sort_by(|a, b| a.name.cmp(&b.name));
                templates.push(ModelTemplate {
                    module: module_name.clone(),
                    name: struct_name,
                    fields,
                    alias: None,
                });
                continue;
            }
        }

        let alias = schema_alias_type(
            schema_or_ref,
            &resolver,
            &module_name_map,
            &available_modules,
        );
        templates.push(ModelTemplate {
            module: module_name,
            name: raw_name.to_upper_camel_case(),
            fields: Vec::new(),
            alias: Some(alias),
        });
    }

    templates.sort_by(|a, b| a.module.cmp(&b.module));
    let mut modules = Vec::new();
    for template in templates {
        let mut context = tera::Context::new();
        context.insert("name", &template.name);
        context.insert("fields", &template.fields);
        context.insert("alias", &template.alias);

        let render = tera
            .render("model.rs", &context)
            .with_context(|| format!("failed to render model {}", template.name))?;

        tokio::fs::write(models_dir.join(format!("{}.rs", template.module)), render)
            .await
            .with_context(|| format!("failed to write model file {}", template.module))?;

        modules.push(template.module);
    }

    modules.sort();
    modules.dedup();

    Ok(ModelGenerationResult {
        modules,
        module_set: available_modules,
        module_name_map,
    })
}

fn build_schema_module_name_map(components: &Components) -> HashMap<String, String> {
    let mut counts: HashMap<String, usize> = HashMap::new();
    let mut map = HashMap::new();

    for name in components.schemas.keys() {
        let base = sanitize_module_name(name);
        let counter = counts.entry(base.clone()).or_insert(0);
        let module_name = if *counter == 0 {
            base.clone()
        } else {
            format!("{base}_{}", counter)
        };
        *counter += 1;
        map.insert(name.clone(), module_name);
    }

    map
}

fn collect_struct_modules(
    components: &Components,
    resolver: &RefResolver<'_>,
    module_name_map: &HashMap<String, String>,
) -> HashSet<String> {
    let mut struct_modules = HashSet::new();
    for (name, schema_or_ref) in components.schemas.iter() {
        let mut visited = BTreeSet::new();
        if schema_supports_struct(schema_or_ref, resolver, &mut visited) {
            if let Some(module) = module_name_map.get(name) {
                struct_modules.insert(module.clone());
            }
        }
    }
    struct_modules
}

fn schema_supports_struct(
    schema_or_ref: &ReferenceOr<openapiv3::Schema>,
    resolver: &RefResolver<'_>,
    visited: &mut BTreeSet<String>,
) -> bool {
    match schema_or_ref {
        ReferenceOr::Reference { reference } => {
            if let Some(name) = component_name_from_reference(reference) {
                if !visited.insert(name.to_string()) {
                    return false;
                }
                let result = resolver
                    .resolve_schema(reference)
                    .map(|schema| schema_supports_struct_item(schema, resolver, visited))
                    .unwrap_or(false);
                visited.remove(name);
                result
            } else {
                false
            }
        }
        ReferenceOr::Item(schema) => schema_supports_struct_item(schema, resolver, visited),
    }
}

fn schema_supports_struct_item(
    schema: &openapiv3::Schema,
    resolver: &RefResolver<'_>,
    visited: &mut BTreeSet<String>,
) -> bool {
    match &schema.schema_kind {
        SchemaKind::Type(openapiv3::Type::Object(obj)) => !obj.properties.is_empty(),
        SchemaKind::AllOf { all_of } => all_of
            .iter()
            .any(|inner| schema_supports_struct(inner, resolver, visited)),
        SchemaKind::Any(any_schema) => {
            !any_schema.properties.is_empty()
                || any_schema
                    .all_of
                    .iter()
                    .any(|inner| schema_supports_struct(inner, resolver, visited))
        }
        _ => false,
    }
}

fn collect_model_fields(
    schema_or_ref: &ReferenceOr<openapiv3::Schema>,
    resolver: &RefResolver<'_>,
    module_name_map: &HashMap<String, String>,
    struct_modules: &HashSet<String>,
    available_modules: &HashSet<String>,
    visited: &mut BTreeSet<String>,
) -> Option<Vec<ModelField>> {
    collect_fields_impl(
        schema_or_ref,
        resolver,
        module_name_map,
        struct_modules,
        available_modules,
        visited,
    )
}

fn collect_fields_impl(
    schema_or_ref: &ReferenceOr<openapiv3::Schema>,
    resolver: &RefResolver<'_>,
    module_name_map: &HashMap<String, String>,
    struct_modules: &HashSet<String>,
    available_modules: &HashSet<String>,
    visited: &mut BTreeSet<String>,
) -> Option<Vec<ModelField>> {
    match schema_or_ref {
        ReferenceOr::Reference { reference } => {
            let name = component_name_from_reference(reference)?.to_string();
            if !visited.insert(name.clone()) {
                return None;
            }
            let schema = resolver.resolve_schema(reference)?;
            let fields = collect_fields_from_schema(
                schema,
                resolver,
                module_name_map,
                struct_modules,
                available_modules,
                visited,
            );
            visited.remove(&name);
            fields
        }
        ReferenceOr::Item(schema) => collect_fields_from_schema(
            schema,
            resolver,
            module_name_map,
            struct_modules,
            available_modules,
            visited,
        ),
    }
}

fn collect_fields_from_schema(
    schema: &openapiv3::Schema,
    resolver: &RefResolver<'_>,
    module_name_map: &HashMap<String, String>,
    struct_modules: &HashSet<String>,
    available_modules: &HashSet<String>,
    visited: &mut BTreeSet<String>,
) -> Option<Vec<ModelField>> {
    match &schema.schema_kind {
        SchemaKind::Type(openapiv3::Type::Object(obj)) => {
            let required: BTreeSet<String> = obj.required.iter().cloned().collect();
            let mut fields = Vec::new();
            for (prop_name, prop_schema) in obj.properties.iter() {
                let normalized = clone_schema_reference(prop_schema);
                let param_type = infer_param_type_from_schema(&normalized);
                let rust_type =
                    param_type_to_rust(&param_type, Some(module_name_map), Some(available_modules));
                let field_name = sanitize_field_name(prop_name);
                fields.push(ModelField {
                    name: field_name,
                    ty: rust_type,
                    required: required.contains(prop_name),
                    flatten: false,
                });
            }
            Some(merge_model_fields(fields))
        }
        SchemaKind::AllOf { all_of } => {
            let mut fields = Vec::new();
            append_all_of_fields(
                &mut fields,
                all_of,
                resolver,
                module_name_map,
                struct_modules,
                available_modules,
                visited,
            );
            Some(merge_model_fields(fields))
        }
        SchemaKind::Any(any_schema) => {
            let mut fields = Vec::new();
            if !any_schema.properties.is_empty() {
                let required: BTreeSet<String> = any_schema.required.iter().cloned().collect();
                for (prop_name, prop_schema) in any_schema.properties.iter() {
                    let normalized = clone_schema_reference(prop_schema);
                    let param_type = infer_param_type_from_schema(&normalized);
                    let rust_type = param_type_to_rust(
                        &param_type,
                        Some(module_name_map),
                        Some(available_modules),
                    );
                    let field_name = sanitize_field_name(prop_name);
                    fields.push(ModelField {
                        name: field_name,
                        ty: rust_type,
                        required: required.contains(prop_name),
                        flatten: false,
                    });
                }
            }

            if !any_schema.all_of.is_empty() {
                append_all_of_fields(
                    &mut fields,
                    &any_schema.all_of,
                    resolver,
                    module_name_map,
                    struct_modules,
                    available_modules,
                    visited,
                );
            }

            Some(merge_model_fields(fields))
        }
        _ => None,
    }
}

fn append_all_of_fields(
    fields: &mut Vec<ModelField>,
    all_of: &[ReferenceOr<openapiv3::Schema>],
    resolver: &RefResolver<'_>,
    module_name_map: &HashMap<String, String>,
    struct_modules: &HashSet<String>,
    available_modules: &HashSet<String>,
    visited: &mut BTreeSet<String>,
) {
    for part in all_of {
        match part {
            ReferenceOr::Reference { reference } => {
                if let Some(name) = component_name_from_reference(reference) {
                    let module = module_name_map
                        .get(name)
                        .cloned()
                        .unwrap_or_else(|| sanitize_module_name(name));
                    if struct_modules.contains(&module) {
                        let ty = format!("crate::models::{module}::{}", name.to_upper_camel_case());
                        let ident = sanitize_identifier(&name.to_snake_case());
                        fields.push(ModelField {
                            name: ident,
                            ty,
                            required: true,
                            flatten: true,
                        });
                        continue;
                    }
                    let ty = format!("crate::models::{module}::{}", name.to_upper_camel_case());
                    let ident = sanitize_identifier(&name.to_snake_case());
                    fields.push(ModelField {
                        name: ident,
                        ty,
                        required: true,
                        flatten: false,
                    });
                }
            }
            ReferenceOr::Item(inline_schema) => {
                if let Some(mut nested) = collect_fields_from_schema(
                    inline_schema,
                    resolver,
                    module_name_map,
                    struct_modules,
                    available_modules,
                    visited,
                ) {
                    fields.append(&mut nested);
                }
            }
        }
    }
}

fn merge_model_fields(fields: Vec<ModelField>) -> Vec<ModelField> {
    let mut map: BTreeMap<String, ModelField> = BTreeMap::new();
    for field in fields {
        merge_field_entry(&mut map, field);
    }
    map.into_values().collect()
}

fn merge_field_entry(map: &mut BTreeMap<String, ModelField>, field: ModelField) {
    match map.entry(field.name.clone()) {
        Entry::Vacant(slot) => {
            slot.insert(field);
        }
        Entry::Occupied(mut entry) => {
            let existing = entry.get_mut();
            existing.required = existing.required && field.required;
            existing.flatten = existing.flatten || field.flatten;
            if existing.ty != field.ty {
                existing.ty = "serde_json::Value".into();
            }
        }
    }
}

fn schema_alias_type(
    schema_or_ref: &ReferenceOr<openapiv3::Schema>,
    resolver: &RefResolver<'_>,
    module_name_map: &HashMap<String, String>,
    available_modules: &HashSet<String>,
) -> String {
    match schema_or_ref {
        ReferenceOr::Reference { reference } => {
            if let Some(name) = component_name_from_reference(reference) {
                let module = module_name_map
                    .get(name)
                    .cloned()
                    .unwrap_or_else(|| sanitize_module_name(name));
                let ty = name.to_upper_camel_case();
                format!("crate::models::{module}::{ty}")
            } else {
                "serde_json::Value".into()
            }
        }
        ReferenceOr::Item(schema) => {
            schema_alias_type_from_schema(schema, resolver, module_name_map, available_modules)
        }
    }
}

fn schema_alias_type_from_schema(
    schema: &openapiv3::Schema,
    resolver: &RefResolver<'_>,
    module_name_map: &HashMap<String, String>,
    available_modules: &HashSet<String>,
) -> String {
    match &schema.schema_kind {
        SchemaKind::Type(openapiv3::Type::String(_)) => "String".into(),
        SchemaKind::Type(openapiv3::Type::Integer(_)) => "i64".into(),
        SchemaKind::Type(openapiv3::Type::Number(_)) => "f64".into(),
        SchemaKind::Type(openapiv3::Type::Boolean(_)) => "bool".into(),
        SchemaKind::Type(openapiv3::Type::Array(array)) => {
            if let Some(items) = &array.items {
                let normalized = clone_schema_reference(items);
                let inner = infer_param_type_from_schema(&normalized);
                let ty = param_type_to_rust(&inner, Some(module_name_map), Some(available_modules));
                format!("Vec<{ty}>")
            } else {
                "Vec<serde_json::Value>".into()
            }
        }
        SchemaKind::Type(openapiv3::Type::Object(obj)) => {
            if obj.properties.is_empty() {
                if let Some(additional) = &obj.additional_properties {
                    match additional {
                        openapiv3::AdditionalProperties::Any(_) => {
                            "std::collections::BTreeMap<String, serde_json::Value>".into()
                        }
                        openapiv3::AdditionalProperties::Schema(schema) => {
                            let normalized = match schema.as_ref() {
                                ReferenceOr::Reference { reference } => ReferenceOr::Reference {
                                    reference: reference.clone(),
                                },
                                ReferenceOr::Item(item) => ReferenceOr::Item((*item).clone()),
                            };
                            let param = infer_param_type_from_schema(&normalized);
                            let ty = param_type_to_rust(
                                &param,
                                Some(module_name_map),
                                Some(available_modules),
                            );
                            format!("std::collections::BTreeMap<String, {ty}>")
                        }
                    }
                } else {
                    "std::collections::BTreeMap<String, serde_json::Value>".into()
                }
            } else {
                "serde_json::Value".into()
            }
        }
        SchemaKind::AllOf { all_of } => {
            if let Some(first) = all_of.first() {
                schema_alias_type(first, resolver, module_name_map, available_modules)
            } else {
                "serde_json::Value".into()
            }
        }
        _ => "serde_json::Value".into(),
    }
}

fn clone_schema_reference(
    schema: &ReferenceOr<Box<openapiv3::Schema>>,
) -> ReferenceOr<openapiv3::Schema> {
    match schema {
        ReferenceOr::Reference { reference } => ReferenceOr::Reference {
            reference: reference.clone(),
        },
        ReferenceOr::Item(item) => ReferenceOr::Item((**item).clone()),
    }
}

fn component_name_from_reference(reference: &str) -> Option<&str> {
    reference.rsplit('/').next()
}

fn param_type_to_rust(
    ty: &ParamType,
    module_name_map: Option<&HashMap<String, String>>,
    available_models: Option<&HashSet<String>>,
) -> String {
    match ty {
        ParamType::String => "String".into(),
        ParamType::Integer => "i64".into(),
        ParamType::Boolean => "bool".into(),
        ParamType::Float => "f64".into(),
        ParamType::Array(inner) => format!(
            "Vec<{}>",
            param_type_to_rust(inner, module_name_map, available_models)
        ),
        ParamType::Map(inner) => format!(
            "std::collections::BTreeMap<String, {}>",
            param_type_to_rust(inner, module_name_map, available_models)
        ),
        ParamType::Enum(_) => "String".into(),
        ParamType::Object(name) => {
            let module = module_name_map
                .and_then(|map| map.get(name))
                .cloned()
                .unwrap_or_else(|| sanitize_module_name(name));
            if available_models
                .map(|set| set.contains(&module))
                .unwrap_or(true)
            {
                let struct_name = name.to_upper_camel_case();
                format!("crate::models::{module}::{struct_name}")
            } else {
                "serde_json::Value".into()
            }
        }
        ParamType::Optional(inner) => {
            format!(
                "Option<{}>",
                param_type_to_rust(inner, module_name_map, available_models)
            )
        }
        ParamType::Unknown => "serde_json::Value".into(),
    }
}

fn track_model_usage(
    ty: &ParamType,
    usages: &mut BTreeSet<ModelUsage>,
    module_name_map: &HashMap<String, String>,
) {
    match ty {
        ParamType::Object(name) => {
            let module = module_name_map
                .get(name)
                .cloned()
                .unwrap_or_else(|| sanitize_module_name(name));
            usages.insert(ModelUsage {
                module,
                ty: name.to_upper_camel_case(),
            });
        }
        ParamType::Array(inner) | ParamType::Optional(inner) | ParamType::Map(inner) => {
            track_model_usage(inner, usages, module_name_map);
        }
        _ => {}
    }
}

fn uniquify_name(base: String, method: &str, tracker: &mut HashMap<String, usize>) -> String {
    let mut candidate = base.clone();
    let mut suffix_counter = 2usize;

    while tracker.contains_key(&candidate) {
        let mut suffix = method.to_lowercase();
        if suffix_counter > 2 {
            suffix.push('_');
            suffix.push_str(&suffix_counter.to_string());
        }
        candidate = format!("{}_{}", base, suffix);
        suffix_counter += 1;
    }

    tracker.insert(candidate.clone(), 1);
    candidate
}

fn apply_model_aliases(
    functions: &mut [Function],
    aliases: &BTreeSet<ModelUsage>,
    available_modules: &HashSet<String>,
) {
    for function in functions.iter_mut() {
        function.return_type =
            replace_model_paths(&function.return_type, aliases, available_modules);
        if let Some(body) = function.request_body.as_mut() {
            body.ty = replace_model_paths(&body.ty, aliases, available_modules);
        }
    }
}

fn replace_model_paths(
    src: &str,
    aliases: &BTreeSet<ModelUsage>,
    available_modules: &HashSet<String>,
) -> String {
    let mut result = src.to_string();
    for alias in aliases {
        let full = format!("crate::models::{}::{}", alias.module, alias.ty);
        if available_modules.contains(&alias.module) {
            result = result.replace(&full, &alias.ty);
        } else {
            result = result.replace(&full, "serde_json::Value");
        }
    }
    result
}

fn sanitize_field_name(raw: &str) -> String {
    let snake = raw.to_snake_case();
    sanitize_identifier(&snake)
}

fn sanitize_module_name(raw: &str) -> String {
    let mut module = raw.to_snake_case();
    if module.is_empty() {
        module = "model".into();
    }
    if module
        .chars()
        .next()
        .map(|c| c.is_ascii_digit())
        .unwrap_or(false)
    {
        module = format!("model_{module}");
    }
    if is_rust_keyword(&module) {
        module.push_str("_mod");
    }
    module
}

fn sanitize_identifier(value: &str) -> String {
    let mut ident = if value.is_empty() {
        "field".to_string()
    } else {
        value.to_string()
    };

    if ident
        .chars()
        .next()
        .map(|c| c.is_ascii_digit())
        .unwrap_or(false)
    {
        ident = format!("field_{ident}");
    }

    if is_rust_keyword(&ident) {
        ident = format!("r#{}", ident);
    }

    ident
}

fn sanitize_struct_name(value: &str) -> String {
    let trimmed = value.trim_start_matches("r#");
    let mut ident = if trimmed.is_empty() {
        "Endpoint".to_string()
    } else {
        trimmed.to_upper_camel_case()
    };
    if ident
        .chars()
        .next()
        .map(|c| c.is_ascii_digit())
        .unwrap_or(false)
    {
        ident = format!("Struct{ident}");
    }
    if is_rust_keyword(&ident.to_lowercase()) {
        ident.push_str("Struct");
    }
    ident
}

fn is_rust_keyword(name: &str) -> bool {
    matches!(
        name,
        "as" | "break"
            | "const"
            | "continue"
            | "crate"
            | "else"
            | "enum"
            | "extern"
            | "false"
            | "fn"
            | "for"
            | "if"
            | "impl"
            | "in"
            | "let"
            | "loop"
            | "match"
            | "mod"
            | "move"
            | "mut"
            | "pub"
            | "ref"
            | "return"
            | "self"
            | "static"
            | "struct"
            | "super"
            | "trait"
            | "true"
            | "type"
            | "unsafe"
            | "use"
            | "where"
            | "while"
            | "async"
            | "await"
            | "dyn"
            | "abstract"
            | "become"
            | "box"
            | "do"
            | "final"
            | "macro"
            | "override"
            | "priv"
            | "try"
            | "typeof"
            | "unsized"
            | "virtual"
            | "yield"
    )
}

fn sanitize_method_suffix(value: &str) -> String {
    let snake = value.to_snake_case();
    let trimmed = snake.trim_matches('_');
    let mut ident = if trimmed.is_empty() {
        "param".to_string()
    } else {
        trimmed.to_string()
    };
    if is_rust_keyword(&ident) {
        ident.push_str("_param");
    }
    ident
}

fn format_description_lines(text: Option<&str>) -> Vec<String> {
    match text {
        Some(val) => val.lines().map(|line| hyperlinkize(line.trim())).collect(),
        None => Vec::new(),
    }
}

fn hyperlinkize(line: &str) -> String {
    fn find_url_start(s: &str) -> Option<usize> {
        match (s.find("http://"), s.find("https://")) {
            (Some(http), Some(https)) => Some(if http < https { http } else { https }),
            (Some(pos), None) | (None, Some(pos)) => Some(pos),
            (None, None) => None,
        }
    }

    let mut result = String::new();
    let mut rest = line;
    while let Some(start) = find_url_start(rest) {
        let (prefix, tail) = rest.split_at(start);
        result.push_str(prefix);
        let end = tail
            .find(|c: char| c.is_whitespace())
            .unwrap_or_else(|| tail.len());
        let (url, remainder) = tail.split_at(end);
        if url.starts_with('<') && url.ends_with('>') {
            result.push_str(url);
        } else {
            result.push('<');
            result.push_str(url);
            result.push('>');
        }
        rest = remainder;
    }
    result.push_str(rest);
    result
}

fn sanitize_crate_version(raw: &str) -> String {
    let trimmed = raw.trim();
    let stripped = trimmed.trim_start_matches(|c| c == 'v' || c == 'V');
    let candidate = if stripped.is_empty() {
        trimmed
    } else {
        stripped
    };

    if candidate.is_empty() {
        tracing::warn!("OpenAPI info.version missing; defaulting to 0.1.0");
        return "0.1.0".to_string();
    }

    if Version::parse(candidate).is_ok() {
        return append_alpha(candidate);
    }

    if let Some(padded) = pad_semver_components(candidate) {
        if Version::parse(&padded).is_ok() {
            tracing::warn!("normalized OpenAPI version '{}' to '{}'", raw, padded);
            return append_alpha(&padded);
        }
    }

    tracing::warn!(
        "unable to parse OpenAPI version '{}' - falling back to 0.1.0",
        raw
    );
    append_alpha("0.1.0")
}

fn pad_semver_components(value: &str) -> Option<String> {
    let mut core = value;
    let mut build = "";
    if let Some(idx) = core.find('+') {
        build = &core[idx + 1..];
        core = &core[..idx];
    }

    let mut pre_release = "";
    if let Some(idx) = core.find('-') {
        pre_release = &core[idx + 1..];
        core = &core[..idx];
    }

    if core.is_empty() {
        return None;
    }

    let mut numbers: Vec<u64> = Vec::new();
    for (idx, part) in core
        .split('.')
        .filter(|segment| !segment.is_empty())
        .enumerate()
    {
        let parsed = part.parse::<u64>().ok()?;
        numbers.push(parsed);
        if idx == 2 {
            break;
        }
    }

    if numbers.is_empty() {
        return None;
    }

    while numbers.len() < 3 {
        numbers.push(0);
    }

    let mut version = format!("{}.{}.{}", numbers[0], numbers[1], numbers[2]);
    if !pre_release.is_empty() {
        version.push('-');
        version.push_str(pre_release);
    }
    if !build.is_empty() {
        version.push('+');
        version.push_str(build);
    }

    Some(version)
}

fn append_alpha(version: &str) -> String {
    if version.ends_with("-alpha") {
        version.to_string()
    } else {
        format!("{}-alpha", version.trim_end_matches('-'))
    }
}

fn sanitize_crate_name(raw: &str) -> String {
    let mut slug = raw
        .to_lowercase()
        .replace("public_api", "")
        .replace("api", "")
        .replace("swgger", "")
        .replace('_', "")
        .replace(' ', "")
        .replace("-", "");

    slug = slug.trim_matches('_').to_string();
    if slug.is_empty() {
        panic!("empty name");
    }
    // for pattern in ["public_api", "api", "swagger"] {
    //     if let Some(stripped) = slug.strip_suffix(pattern) {
    //         slug = stripped.trim_end_matches('_').to_string();
    //     }
    // }
    // slug = slug.trim_matches('_').to_string();
    // if slug.is_empty() {
    //     panic!("empty name");
    // }
    slug
}

async fn create_rust_project(title: String, version: String, path: impl AsRef<Path>) -> Result<()> {
    let cargo_template = TEMPLATES
        .get_file("cargo.toml.tera")
        .with_context(|| "failed to retrieve cargo tera template")?;
    let mut tera = tera::Tera::default();
    let mut context = tera::Context::new();
    let crate_name = sanitize_crate_name(&title.to_snake_case());
    let skygen_name = format!("{}_skygen", &crate_name);
    context.insert("crate_name", &skygen_name);
    context.insert("normalized_name", &crate_name);
    context.insert("version", &version);
    let contents = cargo_template
        .contents_utf8()
        .with_context(|| "failed to read cargo template")?;
    tera.add_raw_template("cargo.toml.tera", contents)?;
    let render = tera
        .render("cargo.toml.tera", &context)
        .with_context(|| "failed to render cargo template")?;
    tokio::fs::write(format!("{}/Cargo.toml", path.as_ref().display()), render)
        .await
        .with_context(|| "failed to write Cargo.toml file")
}
