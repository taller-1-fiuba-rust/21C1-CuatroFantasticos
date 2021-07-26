use regex::Regex;

pub struct RedisPattern {
    regex: Regex,
}

pub enum RedisPatternError {
    ErrorParsingPattern,
}

impl RedisPattern {
    pub fn new(pattern: &str) -> Result<RedisPattern, RedisPatternError> {
        let pattern = pattern.replace("?", ".").replace("*", ".*");
        let pattern = format!("^{}$", pattern);
        let regex = match Regex::new(&pattern) {
            Ok(v) => v,
            Err(_) => return Err(RedisPatternError::ErrorParsingPattern),
        };
        Ok(RedisPattern { regex })
    }
    pub fn is_match(&self, str: &str) -> bool {
        self.regex.is_match(str)
    }
}
