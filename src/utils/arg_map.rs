use std::collections::{HashMap, hash_map::Entry};

pub type ArgKey = String;
pub type ArgVal = Option<String>;

pub struct ArgMap {
    arg_map: HashMap<ArgKey, ArgVal>,
}

impl ArgMap {
    /// Parses a sequence of command-line arguments into a key-value map.
    pub fn from(args: impl IntoIterator<Item: AsRef<str>>) -> Result<Self, String> {
        let mut arg_map = HashMap::new();

        for arg in args {
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

    /// Returns a value of required arg from `ArgMap`.
    pub fn get_required(&self, key: &str) -> Result<&String, String> {
        let Some(val) = self
            .arg_map
            .get(key)
            .ok_or_else(|| format!("Missing required arg '{key}'."))?
        else {
            return Err(format!("Missing value for arg '{key}'."));
        };
        Ok(val)
    }

    /// Checks whether an arg-flag with specified key exists.
    pub fn check_arg(&self, key: &str) -> bool {
        self.arg_map.contains_key(key)
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
