use std::process::Command;

use regex::Regex;
use serde::de;

pub fn deserialize_regex<'de, D>(deserializer: D) -> Result<Regex, D::Error>
where
    D: de::Deserializer<'de>,
{
    struct RegexStringVisitor;

    impl<'de> de::Visitor<'de> for RegexStringVisitor {
        type Value = Regex;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a string representing a regex")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Regex::new(v).map_err(E::custom)
        }
    }

    deserializer.deserialize_any(RegexStringVisitor)
}

pub fn command_from_string(s: &str) -> Command {
    if cfg!(target_os = "windows") {
        let mut command = Command::new("cmd");
        command.arg("/C").arg(s);
        command
    } else {
        let mut command = Command::new("sh");
        command.arg("-c").arg(s);
        command
    }
}
