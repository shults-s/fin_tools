use std::collections::{HashMap, hash_map::Entry};

pub type ArgKey = String;
pub type ArgVal = Option<String>;
pub type ArgMap = HashMap<ArgKey, ArgVal>;

fn try_collect_arg(key: String, val: Option<String>, arg_map: &mut ArgMap) -> Result<(), String> {
    if !key.starts_with("--") {
        return Err(format!(
            "Invalid arg '{}'. Any arg must starts with double hyphen",
            &key
        ));
    }

    match arg_map.entry(key.trim_start_matches('-').into()) {
        Entry::Occupied(o) => Err(format!("Repeated arg '{}'.", o.key())),
        Entry::Vacant(v) => {
            v.insert(val);
            Ok(())
        }
    }
}

pub fn collect_args(args: impl IntoIterator<Item: AsRef<str>>) -> Result<ArgMap, String> {
    let mut parsed_args = HashMap::new();

    for arg in args {
        let arg = arg.as_ref();
        if let Some((key, val)) = arg.split_once('=') {
            try_collect_arg(key.to_ascii_lowercase(), Some(val.into()), &mut parsed_args)?;
        } else {
            try_collect_arg(arg.to_ascii_lowercase(), None, &mut parsed_args)?;
        }
    }

    Ok(parsed_args)
}

pub fn get_required_arg_val<'a>(key: &str, arg_map: &'a ArgMap) -> Result<&'a String, String> {
    let Some(val) = arg_map
        .get(key)
        .ok_or_else(|| format!("Missing required arg '{key}'."))?
    else {
        return Err(format!("Missing value for arg '{key}'."));
    };
    Ok(val)
}
