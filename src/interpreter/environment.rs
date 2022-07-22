use crate::Value;
use std::collections::HashMap;

pub struct EnvironmentStack {
    global_environment: Environment,
}

impl EnvironmentStack {
    pub fn new() -> Self {
        Self {
            global_environment: Environment::new(),
        }
    }

    pub fn set(&mut self, name: &str, value: Value) {
        self.global_environment.set(name, value)
    }

    pub fn get(&mut self, name: &str) -> Option<Value> {
        self.global_environment.get(name)
    }
}

struct Environment {
    variables: HashMap<String, Value>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    pub fn set(&mut self, name: &str, value: Value) {
        self.variables.insert(name.to_string(), value);
    }

    pub fn get(&self, name: &str) -> Option<Value> {
        return self.variables.get(name).cloned();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_setting_and_getting_values() {
        let mut environment_stack = EnvironmentStack::new();

        environment_stack.set("my-var", Value::Number(3));
        assert_eq!(environment_stack.get("my-var"), Some(Value::Number(3)));
    }
}
