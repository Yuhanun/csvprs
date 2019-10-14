use std::fs::File;
use std::io::Read;

#[allow(dead_code)]
pub struct CsvReader<'a> {
    file: &'a mut File,
    data: Vec<Vec<String>>,
}

impl<'a> CsvReader<'a> {
    #[allow(dead_code)]
    pub fn from_file(file: &'a mut File) -> Self {
        CsvReader {
            file: file,
            data: vec![vec![]],
        }
    }

    #[allow(dead_code)]
    pub fn headers(&self) -> Option<&Vec<String>> {
        self.data.first()
    }

    #[allow(dead_code)]
    pub fn data(&self) -> Option<&[Vec<String>]> {
        self.data.get(1..)
    }

    #[allow(dead_code)]
    pub fn process(&mut self) -> Option<&mut Self> {
        let mut content = vec![];
        match self.file.read_to_end(&mut content) {
            Ok(count) => {
                if count == 0 {
                    return None;
                }
            }
            Err(_) => {
                return None;
            }
        }
        let append_last = content.last().unwrap().clone() != '\n' as u8
            && content.last().unwrap().clone() != ',' as u8;
        let mut iterator = content.into_iter();
        let mut elem: String = String::new();
        while let Some(c) = iterator.next() {
            if c == '\n' as u8 {
                self.data.last_mut().unwrap().push(elem.clone());
                self.data.push(vec![]);
                elem = String::new();
                continue;
            }

            if c == ',' as u8 {
                self.data.last_mut().unwrap().push(elem.clone());
                elem = String::new();
            } else {
                elem.push(c as char);
            }
        }
        if append_last {
            self.data.last_mut().unwrap().push(elem.clone());
        }

        self.data.shrink_to_fit();
        Some(self)
    }
}
