use std::collections::{HashMap, VecDeque};

use crate::Value;

pub struct EnvironmentStack {
    global_environment: Environment,
    environments: VecDeque<Environment>,
}

impl EnvironmentStack {
    pub fn new() -> Self {
        Self {
            global_environment: Environment::new(),
            environments: VecDeque::new(),
        }
    }

    pub fn set(&mut self, name: &str, value: Value) {
        if let Some(environment) = self.environments.back_mut() {
            environment.set(name, value);
        } else {
            self.global_environment.set(name, value)
        }
    }

    pub fn get(&mut self, name: &str) -> Option<Value> {
        for environment in self.environments.iter() {
            if environment.contains(name) {
                return environment.get(name);
            }
        }

        self.global_environment.get(name)
    }

    pub fn push_environment(&mut self, variables: HashMap<String, Value>) {
        self.environments.push_front(Environment { variables });
    }

    pub fn pop_environment(&mut self) {
        self.environments.pop_front();
    }
}

#[derive(Debug)]
struct Environment {
    variables: HashMap<String, Value>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    pub fn contains(&self, name: &str) -> bool {
        self.variables.contains_key(name)
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

    #[test]
    fn test_shadowing_variables() {
        let mut environment_stack = EnvironmentStack::new();

        environment_stack.set("my-var", Value::Number(3));
        assert_eq!(environment_stack.get("my-var"), Some(Value::Number(3)));

        {
            let mut arguments = HashMap::new();
            arguments.insert("my-var".to_string(), Value::Number(2));
            environment_stack.push_environment(arguments);
            assert_eq!(environment_stack.get("my-var"), Some(Value::Number(2)));
        }

        {
            let mut arguments = HashMap::new();
            arguments.insert("my-var".to_string(), Value::Number(5));
            environment_stack.push_environment(arguments);
            assert_eq!(environment_stack.get("my-var"), Some(Value::Number(5)));
        }

        environment_stack.pop_environment();
        environment_stack.pop_environment();

        environment_stack.pop_environment();
        assert_eq!(environment_stack.get("my-var"), Some(Value::Number(3)));
    }
}
