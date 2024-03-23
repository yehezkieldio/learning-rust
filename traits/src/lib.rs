use std::io::{Read, Write};

pub struct Config {
    values: Vec<(String, String)>,
}

impl Config {
    pub fn new(values: Vec<(String, String)>) -> Config {
        Config { values: values }
    }
}

pub struct KeyValueConfigService {}

impl KeyValueConfigService {
    pub fn new() -> KeyValueConfigService {
        KeyValueConfigService {}
    }
}

pub trait ValueGetter {
    fn get(&self, key: &str) -> Option<String>;
}

impl ValueGetter for Config {
    fn get(&self, s: &str) -> Option<String> {
        self.values.iter().find_map(|tuple| {
            if &tuple.0 == s {
                Some(tuple.1.clone())
            } else {
                None
            }
        })
    }
}

pub trait ConfigWriter {
    fn write(&self, config: Config, to: &mut impl Write) -> std::io::Result<()>;
}

impl ConfigWriter for KeyValueConfigService {
    fn write(&self, config: Config, to: &mut impl Write) -> std::io::Result<()> {
        for v in config.values {
            writeln!(to, "{0}={1}", v.0, v.1)?;
        }
        Ok(())
    }
}

pub trait ConfigReader {
    fn read(&self, from: &mut impl Read) -> std::io::Result<Config>;
}

impl ConfigReader for KeyValueConfigService {
    fn read(&self, from: &mut impl Read) -> std::io::Result<Config> {
        let mut buffer = String::new();
        from.read_to_string(&mut buffer)?;

        let values: Vec<(String, String)> = buffer
            .split_terminator("\n")
            .map(|line| line.trim())
            .filter(|line| {
                let pos = line.find("=").unwrap_or(0);
                pos > 0 && pos < line.len() - 1
            })
            .map(|line| {
                let parts = line.split("=").collect::<Vec<&str>>();
                (parts[0].to_string(), parts[1].to_string())
            })
            .collect();

        Ok(Config::new(values))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn config_get_value() {
        let config = Config::new(vec![("hello".to_string(), "world".to_string())]);

        assert_eq!(config.get("hello"), Some("world".to_string()));
        assert_eq!(config.get("HELLO"), None);
    }

    #[test]
    fn keyvalueservice_write_config() {
        let config = Config::new(vec![
            ("hello".to_string(), "world".to_string()),
            ("foo".to_string(), "bar".to_string()),
        ]);

        let service = KeyValueConfigService::new();
        let mut target = vec![];
        assert!(service.write(config, &mut target).is_ok());
        assert_eq!(
            String::from_utf8(target).unwrap(),
            "hello=world\nfoo=bar\n".to_string()
        );
    }

    #[test]
    fn keyvalueservice_read_config() {
        let input = "hello=world\nfoo=bar\n";
        let mut source = Cursor::new(input);

        let service = KeyValueConfigService::new();
        let config = service.read(&mut source).unwrap();

        assert_eq!(config.get("hello"), Some("world".to_string()));
        assert_eq!(config.get("foo"), Some("bar".to_string()));
    }
}
