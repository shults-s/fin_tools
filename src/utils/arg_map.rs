use std::collections::{HashMap, hash_map::Entry};

/// Represents command-line argument key.
pub type ArgKey = String;

type ArgVal = Option<String>;

/// Represents command-line arguments key-value storage.
pub struct ArgMap {
    arg_map: HashMap<ArgKey, ArgVal>,
}

impl ArgMap {
    /// Creates an instance of `ArgMap` from a sequence of command-line arguments.\
    /// Returns error when found:
    /// - repeated args with same key,
    /// - args not leading with double hyphen.
    pub fn from(args: impl IntoIterator<Item: AsRef<str>>) -> Result<Self, String> {
        let mut arg_map = HashMap::new();

        for arg in args.into_iter().skip(1) {
            let arg = arg.as_ref();
            if let Some((key, val)) = arg.split_once('=') {
                Self::try_add_arg(
                    key.to_ascii_lowercase(),
                    Some(val.trim_matches('"').into()),
                    &mut arg_map,
                )?;
            } else {
                Self::try_add_arg(arg.to_ascii_lowercase(), None, &mut arg_map)?;
            }
        }

        Ok(Self { arg_map })
    }

    /// Gets a value of required arg.\
    /// Returns error when:
    /// - arg with specified key does not exists,
    /// - value for arg is not set.
    pub fn get_required(&self, key: &str) -> Result<&str, String> {
        let Some(val) = self
            .arg_map
            .get(key)
            .ok_or_else(|| format!("Missing required arg '{key}'."))?
        else {
            return Err(format!("Missing value for arg '{key}'."));
        };
        Ok(val)
    }

    /// Gets a value of arg if arg with specified key exists.\
    /// Returns `None` when:
    /// - arg with specified key does not exists,
    /// - value for arg is not set.
    pub fn get_arg(&self, key: &str) -> Option<&str> {
        self.arg_map.get(key)?.as_deref()
    }

    /// Checks whether an flag-arg with specified key exists. Flag-arg has no value.\
    /// Returns `true` when arg with specified key exists and it's value is **not**
    /// set, otherwise, `false`.
    pub fn check_flag_arg(&self, key: &str) -> bool {
        let Some(val) = self.arg_map.get(key) else {
            return false;
        };
        val.is_none()
    }

    /// Checks whether an value-arg with specified key exists.\
    /// Returns `true` when arg with specified key exists and it's value is set,
    /// otherwise, `false`.
    pub fn check_val_arg(&self, key: &str) -> bool {
        let Some(val) = self.arg_map.get(key) else {
            return false;
        };
        val.is_some()
    }

    fn try_add_arg(
        key: String,
        val: Option<String>,
        arg_map: &mut HashMap<ArgKey, ArgVal>,
    ) -> Result<(), String> {
        if !key.starts_with("--") {
            return Err(format!(
                "Invalid arg key '{}'. Any arg must starts with double hyphen.",
                &key
            ));
        }

        match arg_map.entry(key.trim_start_matches('-').into()) {
            Entry::Occupied(o) => Err(format!("Repeated arg key '{}'.", o.key())),
            Entry::Vacant(v) => {
                v.insert(val);
                Ok(())
            }
        }
    }
}
