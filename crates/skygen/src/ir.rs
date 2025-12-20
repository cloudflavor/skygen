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

use std::fmt;

use serde::Serialize;

#[derive(Clone, Debug)]
pub struct Operation {
    pub id: String,
    pub summary: Option<String>,
    pub method: HttpMethod,
    pub path: String,
    pub tags: Vec<String>,
    pub params: Vec<Parameter>,
    pub request_body: Option<RequestBody>,
    pub response: ParamType,
}

#[derive(Clone, Debug)]
pub struct RequestBody {
    pub ty: ParamType,
    pub required: bool,
}

#[derive(Clone, Debug)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
    Patch,
    Options,
}

impl fmt::Display for HttpMethod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HttpMethod::Get => write!(f, "GET"),
            HttpMethod::Post => write!(f, "POST"),
            HttpMethod::Put => write!(f, "PUT"),
            HttpMethod::Delete => write!(f, "DELETE"),
            HttpMethod::Patch => write!(f, "PATCH"),
            HttpMethod::Options => write!(f, "OPTIONS"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Parameter {
    pub name: String,
    // pub ty: ParamType,
    pub location: ParamLocation,
    pub required: bool,
}

#[derive(Clone, Debug, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ParamLocation {
    Path,
    Query,
    Header,
}

#[derive(Clone, Debug, Serialize)]
pub enum ParamType {
    String,
    Integer,
    Boolean,
    Float,
    Array(Box<ParamType>),
    Map(Box<ParamType>),
    Enum(Vec<String>),
    Object(String),
    Optional(Box<ParamType>),
    Unknown,
}

#[derive(Clone, Debug, Serialize)]
pub struct Function {
    pub name: String,
    pub builder_struct: String,
    pub doc: String,
    pub params: Vec<FunctionParam>,
    pub path_params: Vec<PathParam>,
    pub return_type: String,
    pub method: String,
    pub path: String,
    pub request_body: Option<RequestBodyConfig>,
}

#[derive(Clone, Debug, Serialize)]
pub struct FunctionParam {
    pub name: String,
    pub setter: String,
    pub wire_name: String,
    pub required: bool,
    pub location: ParamLocation,
}

#[derive(Clone, Debug, Serialize)]
pub struct PathParam {
    pub wire_name: String,
    pub setter: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct RequestBodyConfig {
    pub ty: String,
    pub required: bool,
}
