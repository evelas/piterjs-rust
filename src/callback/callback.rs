use std::sync::{Arc, Mutex};

use std::fs;
use rayon::prelude::*;
use csv::ReaderBuilder;
use std::path::Path;

pub type ProgressCallback = Box<dyn Fn(String, u64, u64, Option<String>) -> () + Sync + Send>;

pub fn loading_files_with_progress(progress_callback: ProgressCallback) -> String {

    let files: Vec<String> = vec![String::from("list1.csv"), String::from("list2.csv")]
        .into_iter()
        .map(|f| String::from(Path::new("./__test__/testdata").join(f).to_str().unwrap()))
        .collect();

    let mutex_progress_callback = Arc::new(Mutex::new(progress_callback));
    Vec::from_iter(0..files.len()).into_par_iter()
        .for_each(|i| {
            let file = &files[i];
            let cb = |processed: u64, total: u64| {
                mutex_progress_callback
                .lock()
                .unwrap()(
                    String::from("Loading"), 
                    processed, 
                    total, 
                    Some(format!("File #{}", i)),
                );
            };
            
            let mut rdr = ReaderBuilder::new()
                .has_headers(false)
                .delimiter(0x09)
                .from_path(& file).unwrap();
        
            let total_file_sizes = fs::metadata(& file).unwrap().len();  
            let records = rdr.byte_records();
         
            for result in records {
                let row = result.unwrap();
                let position = row.position().unwrap();
                cb(position.byte(), total_file_sizes);
            }
           
        });
        "loading files done".to_string()
}


#[cfg(test)]
mod tests {
    use crate::callback::loading_files_with_progress;

    fn progress_callback(stage: String, processed: u64, total: u64, label: Option<String>) {
        println!("{}: {} / {} {}", stage, processed, total, label.unwrap_or("".to_string()));
    }
    #[test]
    fn test() {
        loading_files_with_progress(Box::new(progress_callback));
    }

}
