use std::{path::PathBuf, time::UNIX_EPOCH};

use chrono::NaiveDateTime;
use fancy_regex::Regex;
use tanoshi_lib::prelude::{Chapter, Extension, ExtensionResult, Manga, Source};
use zip::ZipArchive;

pub static ID: i64 = 1;

pub struct Local {
    path: String,
}

impl Local {
    pub fn new(path: String) -> Box<dyn Extension> {
        Box::new(Self { path })
    }
}

impl Extension for Local {
    fn detail(&self) -> Source {
        Source {
            id: ID,
            name: "local".to_string(),
            url: self.path.clone(),
            version: "1.0.0".to_string(),
            icon: "".to_string(),
            need_login: false,
        }
    }

    fn get_manga_list(&self, param: tanoshi_lib::prelude::Param) -> ExtensionResult<Vec<Manga>> {
        let read_dir = match std::fs::read_dir(&self.path) {
            Ok(read_dir) => read_dir,
            Err(e) => {
                return ExtensionResult::err(format!("{}", e).as_str());
            }
        };

        let mut data = vec![];
        for entry in read_dir.into_iter().filter_map(|entry| entry.ok()) {
            if let Ok(entry_read_dir) = entry.path().read_dir() {
                let mut cover_url = "".to_string();
                if let Some(entry) = entry_read_dir
                    .into_iter()
                    .filter_map(|entry| match entry {
                        Ok(res) => {
                            if res.path().is_file() {
                                Some(res)
                            } else {
                                None
                            }
                        }
                        Err(_) => None,
                    })
                    .next()
                {
                    match std::fs::File::open(entry.path()) {
                        Ok(file) => {
                            let mut archive = ZipArchive::new(file).unwrap();
                            let f = archive.by_index(0).unwrap();
                            cover_url = entry
                                .path()
                                .join(f.name().clone())
                                .to_str()
                                .unwrap()
                                .to_string();
                        }
                        Err(e) => {
                            return ExtensionResult::err(format!("{}", e).as_str());
                        }
                    }
                }

                data.push(Manga {
                    source_id: ID,
                    title: entry.file_name().to_str().unwrap().to_string(),
                    author: vec![],
                    genre: vec![],
                    status: None,
                    description: None,
                    path: entry.path().to_str().unwrap().to_string(),
                    cover_url,
                })
            }
        }

        ExtensionResult::ok(data)
    }

    fn get_manga_info(&self, path: String) -> ExtensionResult<Manga> {
        let path = PathBuf::from(&path);

        let mut cover_url = "".to_string();
        if let Ok(entry_read_dir) = path.read_dir() {
            if let Some(entry) = entry_read_dir
                .into_iter()
                .filter_map(|entry| match entry {
                    Ok(res) => {
                        if res.path().is_file() {
                            Some(res)
                        } else {
                            None
                        }
                    }
                    Err(_) => None,
                })
                .next()
            {
                match std::fs::File::open(entry.path()) {
                    Ok(file) => {
                        let mut archive = ZipArchive::new(file).unwrap();
                        let f = archive.by_index(0).unwrap();
                        cover_url = entry
                            .path()
                            .join(f.name().clone())
                            .to_str()
                            .unwrap()
                            .to_string();
                    }
                    Err(e) => {
                        return ExtensionResult::err(format!("{}", e).as_str());
                    }
                }
            }
        }

        ExtensionResult::ok(Manga {
            source_id: ID,
            title: path.file_name().unwrap().to_str().unwrap().to_string(),
            author: vec![],
            genre: vec![],
            status: None,
            description: None,
            path: path.to_str().unwrap().to_string(),
            cover_url,
        })
    }

    fn get_chapters(&self, path: String) -> ExtensionResult<Vec<Chapter>> {
        let read_dir = match std::fs::read_dir(&path) {
            Ok(read_dir) => read_dir,
            Err(e) => {
                return ExtensionResult::err(format!("{}", e).as_str());
            }
        };

        let mut data = vec![];
        for entry in read_dir.into_iter().filter_map(|entry| entry.ok()) {
            let created = entry
                .metadata()
                .unwrap()
                .created()
                .unwrap()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
            let number_re =
                Regex::new(r"(?i)(?<=v)(\d+)|(?<=volume)\s*(\d+)|(?<=vol)\s*(\d+)|(?<=\s)(\d+)")
                    .unwrap();
            let file_name = entry
                .path()
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();
            info!("local: {}", file_name);
            let mat = number_re.find(&file_name).unwrap();
            let number = mat
                .map(|m| m.as_str().parse().unwrap_or(0_f64))
                .unwrap_or(10000_f64);

            data.push(Chapter {
                source_id: ID,
                title: file_name,
                path: entry.path().to_str().unwrap().to_string(),
                number,
                scanlator: "".to_string(),
                uploaded: NaiveDateTime::from_timestamp(created as i64, 0),
            })
        }

        ExtensionResult::ok(data)
    }

    fn get_pages(&self, path: String) -> ExtensionResult<Vec<String>> {
        let path = PathBuf::from(path);
        match std::fs::File::open(path.clone()) {
            Ok(file) => {
                let mut archive = ZipArchive::new(file).unwrap();
                let mut pages = vec![];
                for i in 0..archive.len() {
                    let f = archive.by_index(i).unwrap();
                    pages.push(path.join(f.name()).to_str().unwrap().to_string());
                }

                ExtensionResult::ok(pages)
            }
            Err(e) => ExtensionResult::err(format!("{}", e).as_str()),
        }
    }
}
