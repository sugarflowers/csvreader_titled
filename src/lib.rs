use csv::ReaderBuilder;
use linked_hash_map::LinkedHashMap;
use sjis::read_text;

#[derive(Default, Debug)]
pub struct CsvReader {
    file_path: String,
    counter: usize,
}

impl CsvReader {
    pub fn open(file_path: &str) -> CsvReader {
        CsvReader {
            file_path: file_path.to_string(),
            ..CsvReader::default()
        }
    }

    pub fn rows_count(&self) -> usize {
        self.counter
    }
    
    pub fn read<F>(&mut self, mut process_row: F)
    where
        F: FnMut(LinkedHashMap<String, String>),
    {
        self.counter = 0;
        let content = read_text(&self.file_path);

        let mut rdr = ReaderBuilder::new().has_headers(false).from_reader(content.as_bytes());
        let mut title:Vec<String> = Vec::new();
        
        for (idx, result) in rdr.records().enumerate() {
            let processed_record: Vec<String> = result.unwrap()
                .iter().map(|field| field.to_string()).collect();

            if idx == 0 {
                title = processed_record.clone();
            } else {
                let mut lhm = LinkedHashMap::new();
                for (col, val) in processed_record.iter().enumerate() {
                    lhm.insert(title[col].clone(), val.to_string());
                }
                process_row(lhm);
                self.counter += 1;
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::CsvReader;

    #[test]
    fn csv_read_test() {
        CsvReader::open("data.csv").read(|row| {
            //println!(">>> {:?}", row);
            println!("{}", row["発注管理ID"]);
        });
    }

    #[test]
    fn get_count() {
        let mut cr = CsvReader::open("data.csv");
        cr.read(|row| {
            println!("{:?}", row);
        });
        println!("{}", cr.rows_count());
    }
}

