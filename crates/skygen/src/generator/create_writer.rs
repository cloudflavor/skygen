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

use crate::ir::{
    Function, FunctionParam, HttpMethod, Operation, ParamLocation, ParamType, PathParam,
    RequestBodyConfig,
};
use crate::transformers::{
    convert_param, extract_body, extract_response, fallback_operation_id, schemas, RefResolver,
};
use crate::TEMPLATES;
use crate::{deserialize_data, Config};
use anyhow::{ensure, Context, Result};
use heck::{ToSnakeCase, ToUpperCamelCase};
use openapiv3::{Info as OApiInfo, OpenAPI};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::path::Path;
use tokio::process::Command;

pub async fn generate(
    config: &Config,
    schema: impl AsRef<Path>,
    output_dir: impl AsRef<Path>,
) -> Result<()> {
    match tokio::fs::read_to_string(schema).await {
        Ok(content) => {
            let openapi: OpenAPI = deserialize_data(content.as_str()).await?;
            let reqs = walk_paths(&openapi).await?;
            generate_lib(config, reqs, output_dir, &openapi).await?;

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
    (structural || identifier_tokens) && position > 0
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
    for (idx, ch) in path.char_indices() {
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
    openapi: &OpenAPI,
) -> Result<BTreeMap<String, Vec<Operation>>> {
    let mut grouped: BTreeMap<String, Vec<Operation>> = BTreeMap::new();
    let resolver = openapi.components.as_ref().map(RefResolver::new);

    for (path, item) in &openapi.paths.paths {
        if let Some(item) = item.as_item() {
            let shared_params: Vec<_> = item
                .parameters
                .iter()
                .filter_map(|p| p.as_item())
                .cloned()
                .collect();

            for (method_str, op_opt) in [
                ("get", &item.get),
                ("post", &item.post),
                ("put", &item.put),
                ("delete", &item.delete),
                ("patch", &item.patch),
                ("options", &item.options),
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
    config: &Config,
    reqs: BTreeMap<String, Vec<Operation>>,
    path: impl AsRef<Path>,
    openapi: &OpenAPI,
) -> Result<()> {
    let output_root = path.as_ref().to_path_buf();
    let src_dir = output_root.join("src");
    let apis_dir = src_dir.join("apis");
    let models_dir = src_dir.join("models");

    tokio::fs::create_dir_all(&apis_dir).await?;
    tokio::fs::create_dir_all(&models_dir).await?;

    let mut api_modules: Vec<String> = Vec::new();

    let models_code = schemas::generate_models(openapi);
    tokio::fs::write(models_dir.join("mod.rs"), models_code).await?;

    let OApiInfo {
        title,
        description,
        version,
        .. 
    } = &openapi.info;
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
    let crate_name_slug = &config.name;
    let sample_function_name = sample_metadata
        .as_ref()
        .map(|(_, func)| func.clone())
        .unwrap_or_else(|| "health_check".to_string());

    for (tag, ops) in reqs.into_iter() {
        let module_name = sanitize_module_name(&tag);
        let mut module_functions: Vec<Function> = Vec::new();
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

            let return_type = param_type_to_rust(&op.response);
            let request_body = op.request_body.as_ref().map(|body| RequestBodyConfig {
                ty: param_type_to_rust(&body.ty),
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
        context.insert("crate_name", &crate_name_slug);
        context.insert("module_name", &module_name);
        context.insert("functions", &module_functions);
        context.insert("model_uses", &Vec::<String>::new());
        context.insert("api_url", &config.api_url);
        let render = tera
            .render("operations.rs", &context)
            .with_context(|| "failed to render module template")?;

        tokio::fs::write(apis_dir.join(format!("{}.rs", module_name)), render)
            .await
            .with_context(|| format!("failed to write module {}", module_name))?;
            ;

        api_modules.push(module_name);
    }

    render_mod_file(&api_modules, apis_dir.join("mod.rs")).await?;
    let crate_title = title.clone();
    let mut docs = format!("/// {} API version: {}
", crate_title, version);

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
    context.insert("api_url", &config.api_url);
    let render = tera
        .render("lib.rs", &context)
        .with_context(|| "failed to render lib template")?;

    tokio::fs::write(src_dir.join("lib.rs"), render)
        .await
        .with_context(|| "failed to write lib.rs")?;

    create_rust_project(config, &output_root).await?;
    if let Err(err) = run_post_op(&output_root).await {
        tracing::warn!("cargo post-processing failed: {}", err);
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

async fn run_post_op(path: &Path) -> Result<()> {
    run_cargo_command(path, &["fmt"]).await?;
    run_cargo_command(path, &["check"]).await?;
    run_toml_format(path).await?;

    Ok(())
}

async fn run_toml_format(path: impl AsRef<Path>) -> Result<()> {
    Command::new("taplo")
        .current_dir(path)
        .args(["format"])
        .status()
        .await?;

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
        "cargo {} failed with status {}",
        args.join(" "),
        status
    );
    Ok(())
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

fn param_type_to_rust(ty: &ParamType) -> String {
    match ty {
        ParamType::String => "String".into(),
        ParamType::Integer => "i64".into(),
        ParamType::Boolean => "bool".into(),
        ParamType::Float => "f64".into(),
        ParamType::Array(inner) => format!("Vec<{}>", param_type_to_rust(inner)),
        ParamType::Map(inner) => {
            format!("std::collections::BTreeMap<String, {}>", param_type_to_rust(inner))
        }
        ParamType::Enum(_) => "String".into(),
        ParamType::Object(name) => {
            let struct_name = name.to_upper_camel_case();
            format!("crate::models::{}", struct_name)
        }
        ParamType::Optional(inner) => format!("Option<{}>", param_type_to_rust(inner)),
        ParamType::Unknown => "serde_json::Value".into(),
    }
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
        module = format!("model_{}", module);
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
        ident = format!("field_{}", ident);
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
        ident = format!("Struct{}", ident);
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
            .unwrap_or(tail.len());
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

async fn create_rust_project(config: &Config, path: impl AsRef<Path>) -> Result<()> {
    let cargo_template = TEMPLATES
        .get_file("cargo.toml.tera")
        .with_context(|| "failed to retrieve cargo tera template")?;
    let mut tera = tera::Tera::default();
    let mut context = tera::Context::new();
    context.insert("crate_name", &config.name);
    context.insert("version", &config.version);
    context.insert("authors", &config.authors);
    context.insert("description", &config.description);
    context.insert("keywords", &config.keywords);

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