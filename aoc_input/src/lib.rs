use std::collections::HashMap;
use std::fs;
use std::io;

pub struct InputBuilder<'a> {
    sources: Vec<Source<'a>>,
}

pub struct InputResults {
    pub values: HashMap<String, io::Result<String>>,
}

struct Source<'a>{
    name: &'a str,
    source_type: SourceType<'a>,
}

enum SourceType<'a> {
    FileByName(&'a str),
}

impl<'a> InputBuilder<'a> {
    pub fn new() -> Self {
        Self {
            sources: Vec::new(),
        }
    }

    pub fn file(&mut self, name: &'a str, filename: &'a str) {
        self.sources.push(
            Source {
                name: name,
                source_type: SourceType::FileByName(filename),
            })
    }

    pub fn get_inputs(self) -> InputResults {
        let mut values = HashMap::new();
        for source in self.sources {
            match source.source_type {
                SourceType::FileByName(path) => {
                    let file_contents = fs::read_to_string(path);
                    values.insert(String::from(source.name), file_contents);
                }
            }
        }
        InputResults {
            values: values,
        }
    }
}


