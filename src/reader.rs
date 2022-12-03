#![allow(dead_code)]

/// Reader used to parse text file content.
///
/// Use `reader::open(path: &str)` to create a new `Reader` from a text file.
pub struct Reader {
    text: String,
}

impl Reader {
    /// Current content as a single `String`.
    pub fn text(self) -> String {
        self.text
    }

    /// Current content lines as a `Vec<String>`.
    pub fn lines(self) -> Vec<String> {
        self.text.lines().map(|line| line.to_string()).collect()
    }

    /// Current content divided into blocks by empty lines.
    /// Single block can still contain multiple lines.
    /// ## Example
    /// ```rs
    /// // file.txt
    /// // a
    /// // b
    /// //  
    /// // c
    /// // d
    /// open("file.txt").split_on_empty_line()
    /// // ["a\nb", "c\nd"]
    /// ```
    pub fn split_on_empty_line(self) -> Vec<String> {
        self.text
            .replace("\r\n", "\n")
            .split("\n\n")
            .map(|part| part.to_string())
            .collect()
    }

    /// Process the file content as a single string, splitting it on a given pattern, and
    /// parsing the resulting parts into the given type.
    /// ## Example
    /// ```rs
    /// // file.txt
    /// // 1,2,3,4,5,6,7,8,9
    /// open("file.txt").split_line_into::<usize>(",")
    /// // [1, 2, 3, 4, 5, 6, 7, 8, 9]
    /// ```
    /// ## Note
    /// You can provide support for custom structs by implementing the `FromStr` trait.
    /// ```
    /// use std::{str::FromStr, string::ParseError};
    ///
    /// struct Example {
    ///     a: usize,
    ///     b: usize,
    /// }
    ///
    /// impl FromStr for Example {
    ///     type Err = ParseError;
    ///     fn from_str(str: &str) -> Result<Self, Self::Err> {
    ///         let (a, b) = str.split_once(" ").unwrap();
    ///         Ok(Self {
    ///             a: a.parse::<usize>().unwrap(),
    ///             b: b.parse::<usize>().unwrap(),
    ///         })
    ///     }
    /// }
    /// // input.txt
    /// // 1 2,3 4
    /// open("input.txt").split_line_into::<Example>(",")
    /// // [Example { a: 1, b: 2 }, Example { a: 3, b: 4 }]
    /// ```
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

    /// Return the file content lines parsed into the given type.
    /// ## Example
    /// ```rs
    /// // file.txt
    /// // 1
    /// // 2
    /// // 3
    /// // 4
    /// open("file.txt").lines_as::<usize>()
    /// // [1, 2, 3, 4]
    /// ```
    /// ## Note
    /// You can provide support for custom structs by implementing the `FromStr` trait.
    /// ```
    /// use std::{str::FromStr, string::ParseError};
    ///
    /// struct Example {
    ///     a: usize,
    ///     b: usize,
    /// }
    ///
    /// impl FromStr for Example {
    ///     type Err = ParseError;
    ///     fn from_str(str: &str) -> Result<Self, Self::Err> {
    ///         let (a, b) = str.split_once(" ").unwrap();
    ///         Ok(Self {
    ///             a: a.parse::<usize>().unwrap(),
    ///             b: b.parse::<usize>().unwrap(),
    ///         })
    ///     }
    /// }
    /// // input.txt
    /// // 1 2
    /// // 3 4
    /// open("input.txt").lines_as::<Example>()
    /// // [Example { a: 1, b: 2 }, Example { a: 3, b: 4 }]
    /// ```
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

    /// Return the file content lines passed through the given function.
    /// ## Example
    /// ```
    /// fn my_parse_function(str: &str) -> usize {
    ///     str.parse::<usize>().unwrap_or(0)
    /// }
    ///
    /// // input.txt
    /// // 1
    /// // a
    /// // 3
    /// open("input.txt").parse_lines(my_parse_function)
    /// // [1, 0, 3]
    /// ```
    /// ## Note
    /// This provides an alternative way to create custom structs using a non-trait function.
    /// ```
    /// struct Example {
    ///     a: usize,
    ///     b: usize,
    /// }
    ///
    /// impl Example {
    ///     fn from_str(str: &str) -> Self {
    ///         let (a, b) = str.split_once(" ").unwrap();
    ///         Self {
    ///             a: a.parse::<usize>().unwrap(),
    ///             b: b.parse::<usize>().unwrap(),
    ///         }
    ///     }
    /// }
    /// // input.txt
    /// // 1 2
    /// // 3 4
    /// open("input.txt").parse_lines(Example::from_str)
    /// // [Example { a: 1, b: 2 }, Example { a: 3, b: 4 }]
    /// ```
    pub fn parse_lines<T>(self, f: fn(&str) -> T) -> Vec<T> {
        self.text.lines().map(f).collect()
    }
}

/// Open text file in the given path and return its contents as a `Reader`.
pub fn open(path: &str) -> Reader {
    Reader {
        text: std::fs::read_to_string(path).expect("file not found"),
    }
}
