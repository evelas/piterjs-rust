use std::sync::{Arc, Mutex};
use std::fs;
use rayon::prelude::*;
use csv::ReaderBuilder;
use std::path::Path;
pub type ProgressCallback = Box<dyn Fn(u64, u64, usize) -> () + Sync + Send>;


pub fn loading_files_with_progress(progress_callback: ProgressCallback) -> String {

    let files: Vec<String> = vec![String::from("list4.csv"), String::from("list5.csv")]
        .into_iter()
        .map(|f| String::from(Path::new("./__test__/testdata").join(f).to_str().unwrap()))
        .collect();

    let mutex_progress_callback = Arc::new(Mutex::new(progress_callback));
    Vec::from_iter(0..files.len()).into_par_iter()
        .for_each(|i| {
            let file = &files[i];
            let cb = |processed: u64, total: u64, file_iteration: usize| {
                mutex_progress_callback
                .lock()
                .unwrap()(
                    processed, 
                    total,
                    file_iteration,
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
                cb(position.byte(), total_file_sizes, i);
            }
           
        });
        "loading files done".to_string()
}


#[cfg(test)]
mod tests {
    use crate::rust_mod::loading_files_with_progress;

    fn progress_callback(processed: u64, total: u64, file_iteration: usize) {
        println!("{} / {}. File# {}", processed, total, file_iteration);
    }
    #[test]
    fn test() {
        loading_files_with_progress(Box::new(progress_callback));
    }

}
