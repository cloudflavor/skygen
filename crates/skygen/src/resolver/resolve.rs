use crate::ResolverError;
use anyhow::{anyhow, Result};
use serde_json::Value;
use std::collections::HashSet;

#[derive(Debug)]
pub struct Resolver {
    root: Value,
}

impl Resolver {
    pub fn new(root: Value) -> Self {
        Self { root }
    }

    pub fn resolve(&self) -> Result<Value> {
        let mut visited = HashSet::new();
        self.walk(&self.root, &mut visited)
    }

    pub fn walk(&self, current: &Value, visited: &mut HashSet<String>) -> Result<Value> {
        match current {
            Value::Object(map) => {
                if let Some(Value::String(ref_str)) = map.get("ref") {
                    return self.resolver_ref(ref_str, visited);
                }

                let mut new_map = serde_json::Map::new();
                for (k, v) in map {
                    new_map.insert(k.clone(), self.walk(v, visited)?);
                }

                Ok(Value::Object(new_map))
            }

            Value::Array(arr) => {
                let mut new_arr = Vec::new();

                for v in arr {
                    new_arr.push(self.walk(v, visited)?);
                }

                Ok(Value::Array(new_arr))
            }
            _ => Ok(current.clone()),
        }
    }

    pub fn resolver_ref(&self, ref_str: &str, visited: &mut HashSet<String>) -> Result<Value> {
        if visited.contains(ref_str) {
            return Err(ResolverError::CycleDetected(ref_str.to_string()).into());
        }
        visited.insert(ref_str.to_string());

        let tokens = super::pointer::parse_json_pointer_ref(ref_str)?;

        let mut target = &self.root;

        for token in tokens {
            target = target.get(&token).ok_or_else(|| {
                anyhow!(
                    "could not find path '{}' in document for ref '{}'",
                    token,
                    ref_str
                )
            })?;
        }
        let resolved = self.walk(target, visited)?;

        visited.remove(ref_str);

        Ok(resolved)
    }
}
