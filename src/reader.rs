#![allow(dead_code)]

pub struct Reader {
    text: String,
}

impl Reader {
    pub fn text(self) -> String {
        self.text
    }

    pub fn lines(self) -> Vec<String> {
        self.text.lines().map(|line| line.to_string()).collect()
    }

    pub fn split_on_empty_line(self) -> Vec<String> {
        self.text
            .replace("\r\n", "\n")
            .split("\n\n")
            .map(|part| part.to_string())
            .collect()
    }

    pub fn split_line_into<T>(self, pattern: &str) -> Vec<T>
    where
        T: core::str::FromStr,
    {
        self.text
            .split(pattern)
            .map(|part| match part.parse::<T>() {
                Ok(value) => value,
                _ => panic!(
                    "Unable to parse \"{:?}\" into the given type \"{:?}\".",
                    part,
                    std::any::type_name::<T>()
                ),
            })
            .collect()
    }

    pub fn lines_as<T>(self) -> Vec<T>
    where
        T: core::str::FromStr,
    {
        self.text
            .lines()
            .map(|line| match line.parse::<T>() {
                Ok(value) => value,
                _ => panic!(
                    "Unable to parse \"{:?}\" into the given type \"{:?}\".",
                    line,
                    std::any::type_name::<T>()
                ),
            })
            .collect()
    }

    pub fn parse_lines<T>(self, f: fn(&str) -> T) -> Vec<T> {
        self.text.lines().map(f).collect()
    }
}

pub fn open(path: &str) -> Reader {
    Reader {
        text: std::fs::read_to_string(path).expect("file not found"),
    }
}
