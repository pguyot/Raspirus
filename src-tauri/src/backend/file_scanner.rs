use std::{
    fs::{File, self},
    io::{BufReader, Error, ErrorKind, Read},
    path::Path,
    process::exit,
    time,
};

use chrono::{DateTime, Local};
use log::{debug, error, info, warn};
use tauri::Manager;
use terminal_size::terminal_size;
use walkdir::WalkDir;

use super::{db_ops::DBOps, file_log::FileLog};

#[derive(Clone, serde::Serialize)]
struct TauriEvent {
    message: String,
}


/// Struct representing a file scanner that is capable of searching through a specified directory and its subdirectories for malicious files.
pub struct FileScanner {
    /// A reference to a `DBOps` object that allows the `FileScanner` to access and manipulate the database.
    pub db_conn: DBOps,
    /// A vector of file paths for files that have been identified as malicious.
    pub dirty_files: Vec<String>,
    /// The file path of the directory that the `FileScanner` should search through.
    pub scanloc: String,
    /// A `FileLog` object that the `FileScanner` can use to log information about the search process.
    pub log: FileLog,
    /// An array containing false positives MD5 Hashes
    pub false_positive: Vec<String>,
    /// Defines the scanning size in bytes
    folder_size: u64,
    /// Amount scanned in bytes
    scanned_size: u64,
    /// Tauri window for events
    tauri_window: tauri::Window
}

impl FileScanner {

    /// Creates a new `FileScanner` object.
    ///
    /// # Arguments
    ///
    /// * `scanloc` - The file path of the directory that the `FileScanner` should search through.
    /// * `db_file` - The file path of the database that the `FileScanner` should use to store information about the files it has scanned.
    ///
    /// # Returns
    ///
    /// A `Result` object containing a new `FileScanner` object on success or an `Error` on failure.
    ///
    /// # Errors
    ///
    /// This function will return an `Error` with an `ErrorKind` of `Other` if the `scanloc` file path does not exist.
    pub fn new(scanloc: &str, db_file: &str, t_win: tauri::Window) -> Result<Self, Error> {
        //check path
        if Path::new(&scanloc).exists() {
            let tmpconf = match DBOps::new(db_file) {
                Ok(db_conn) => db_conn,
                Err(err) => {
                    error!("{err}");
                    exit(-1);
                }
            };

            let now: DateTime<Local> = Local::now();
            let now_str = now.format("%Y_%m_%d_%H_%M_%S").to_string();
            let log_str = format!("{}.log", now_str);

            // Add all false positives here
            let false_pos: Vec<String> = vec!["7dea362b3fac8e00956a4952a3d4f474".to_owned(), "81051bcc2cf1bedf378224b0a93e2877".to_owned()];

            Ok(FileScanner {
                db_conn: tmpconf,
                dirty_files: Vec::new(),
                scanloc: scanloc.to_owned(),
                log: FileLog::new(log_str),
                false_positive: false_pos,
                folder_size: 0,
                scanned_size: 0,
                tauri_window: t_win,
            })
        } else {
            Err(Error::new(ErrorKind::Other, "Invalid Path"))
        }
    }

    /// Searches the given file location for infected files.
    ///
    /// # Arguments
    ///
    /// * `&mut self` - the `FileScanner` instance
    ///
    /// # Examples
    ///
    /// ```
    /// let mut scanner = FileScanner::new("/path/to/scan", "database.db").unwrap();
    /// scanner.search_files();
    /// ```
    pub fn search_files(&mut self, stop_early:bool) -> Result<Vec<String>, String> {
        let mut analysed: i128 = 0;
        let mut skipped: i128 = 0;
        let last_percentage: &mut f64 = &mut -1.0;
        let big_tic = time::Instant::now();
        if self.get_folder_size(Path::new(self.scanloc.as_str()).to_owned().as_ref()).is_err() {
            return Err("Can't get folder size".to_string());
        }
        for file in WalkDir::new(&self.scanloc)
            .into_iter()
            .filter_map(|file| file.ok())
        {
            if (match file.metadata() {
                Ok(md) => md,
                Err(err) => {
                    error!("Failed getting file metadata: {}", err);
                    return Err("Failed getting file metadata.".to_owned());
                }
            })
            .is_file()
            {
                let hash = match self.create_hash(&file.path().display().to_string()) {
                    Some(hash) => {
                        analysed += 1;
                        hash
                    }
                    None => {
                        skipped += 1;
                        "".to_owned()
                    }
                };
                if Self::calculate_progress(self, last_percentage, file.metadata().unwrap().len()).is_err() {
                    error!("Progress calculation is broken");
                    break;
                }
                if match self.db_conn.hash_exists(hash.as_str()) {
                    Ok(exists) => {
                        self.dirty_files.push(file.path().display().to_string());
                        if stop_early {
                            warn!("Stopping early");
                            break;
                        }
                        exists
                    }
                    Err(_) => false,
                } {
                    info!(
                        "Found hash {} for file {}",
                        hash,
                        file.path().display().to_string()
                    );
                    self.log.log(hash, file.path().display().to_string());
                }
            }
        }
        let big_toc = time::Instant::now();
        info!(
            "=> Analysed: {}, Skipped: {},  Infected: {}, Time: {} seconds",
            analysed,
            skipped,
            self.dirty_files.len(),
            big_toc.duration_since(big_tic).as_secs_f64()
        );
        Ok(self.dirty_files.clone())
    }

    /// Creates the MD5 hash of a file.
    ///
    /// # Arguments
    ///
    /// * `&mut self` - the `FileScanner` instance
    /// * `path` - the path to the file to create the hash for
    ///
    /// # Examples
    ///
    /// ```
    /// let mut scanner = FileScanner::new("/path/to/scan", "database.db").unwrap();
    /// let hash = scanner.create_hash("/path/to/file.exe");
    /// ```
    pub fn create_hash(&mut self, path: &str) -> Option<String> {
        let mut context = md5::Context::new();
        let mut buffer = [0; 65536]; // 64KB

        let file = match File::open(path) {
            Ok(file) => file,
            Err(err) => {
                println!("{}", err);
                return None;
            }
        };
        let mut reader = BufReader::new(file);
        let mut it = 0;
        loop {
            let count = match reader.read(&mut buffer) {
                Ok(count) => count,
                Err(_) => return None,
            };

            if count == 0 {
                if it == 0 {
                    return None;
                }
                break;
            }
            context.consume(&buffer[..count]);
            it += 1;
        }
        let ret = format!("{:?}", context.compute());

        if self.false_positive.contains(&ret) {
            return None;
        }

        match terminal_size() {
            Some((width, _)) => {
                match width.0.checked_sub(path.len() as u16 + 2) {
                    Some(spacing) => {
                        debug!("\n {}{:>width$} ", path, ret, width = spacing as usize);
                    }
                    None => {}
                };
            }
            None => {}
        };
        Some(ret)
    }

    
    fn get_folder_size(&mut self, path: &Path) -> Result<u64, std::io::Error> {
        let metadata = fs::metadata(path)?;
        if metadata.is_file() {
            debug!("Added file: {} with size: {}", path.to_str().unwrap(), metadata.len());
            return Ok(metadata.len());
        }
    
        let mut size: u64 = 0;
    
        for entry in WalkDir::new(path).follow_links(true) {
            let entry = entry?;
            let entry_metadata = entry.metadata()?;
            if entry_metadata.is_file() {
                size += entry_metadata.len();
            }
        }
        self.folder_size = size;
        Ok(size)
    }
    

    fn calculate_progress(&mut self, last_percentage: &mut f64, file_size: u64) -> Result<f64, String> {
        self.scanned_size = self.scanned_size + file_size;
        let scanned_percentage = (self.scanned_size as f64 / self.folder_size as f64 * 100.0).round();
        // Check if folder is empty, because that would return infinity percentage
        if self.folder_size <= 0 {
            if self.tauri_window.emit_all("progerror", TauriEvent {message: "Calculated foldersize is 0".to_string()}).is_err() {
                return Err("Couldn't send progress update to frontend".to_string());
            }
            return Err("Calculated foldersize is 0".to_string());
        }
        info!("Scanned: {}%", scanned_percentage);
        if scanned_percentage != *last_percentage {
            if self.tauri_window.emit_all("progress", TauriEvent {message: scanned_percentage.to_string()}).is_err() {
                return Err("Couldn't send progress update to frontend".to_string());
            };
            *last_percentage = scanned_percentage;
        }
        Ok(scanned_percentage)
    }
}