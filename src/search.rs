
use std::{collections::HashSet, fs};
use std::os::windows::fs::MetadataExt;
use walkdir::{DirEntry, WalkDir};

#[derive(Default, Debug)]
pub struct Argument {
    pub name: String,        // 待搜索的名称
    pub target: String,      // 指定目录
    pub search_hidden: bool, // 是否搜索隐藏文件夹
    pub file_only: bool,     // 是否仅搜索文件
    pub strict_mode: bool,   // 严格匹配模式
    pub case_miss: bool,     // 忽略大小写
    pub suffix: String,      // 文件后缀名
}

pub fn search(args: Argument) -> Vec<String> {
    println!("aargs: {:?}", args);
    let mut results = Vec::new();
    let search_path = if args.target.is_empty() {
        "."
    } else {
        &args.target
    };

    let aa: Box<dyn Iterator<Item = walkdir::DirEntry>> = if args.search_hidden {
        Box::new(WalkDir::new(search_path).into_iter().filter_map(Result::ok))
    } else {
        Box::new(
            WalkDir::new(search_path)
                .into_iter()
                .filter_entry(|e| is_visible(e))
                .filter_map(Result::ok),
        )
    };
    for entry in aa {
        if do_search_item(&entry, &args) {
            println!("path: {}", entry.path().to_string_lossy().to_string());
            results.push(entry.path().to_string_lossy().to_string());
        }
    }
    results
}

fn do_search_item(entry: &DirEntry, args: &Argument) -> bool {
    // 如果仅搜索文件，但当前是目录，则直接返回 false
    if args.file_only && entry.file_type().is_dir() {
        return false;
    }

    // 获取文件名并处理大小写匹配
    let file_name = entry.file_name().to_string_lossy();
    let name_matches = if args.strict_mode {
        if args.case_miss {
            file_name.to_lowercase() == args.name.to_lowercase()
        } else {
            file_name == args.name
        }
    } else {
        if args.case_miss {
            file_name.to_lowercase().contains(&args.name.to_lowercase())
        } else {
            file_name.contains(&args.name)
        }
    };

    let suffix_set: HashSet<String> = args.suffix.split(",").map(|s| s.trim().to_lowercase()).collect();
    // 检查后缀匹配
    let file_suffix = file_name.rsplit('.').map(|s| s.trim().to_lowercase()).next();
    let suffix_matchs = if let Some(suff) = file_suffix {
        args.suffix.is_empty() || suffix_set.contains(&suff)
    } else {
        args.suffix.is_empty()
    };

    // 最终判断
    name_matches && suffix_matchs
}

pub fn search_files(path: &str, target_name: &str) -> Vec<String> {
    let mut results = Vec::new();
    let search_path = if path.is_empty() { "." } else { path };
    let aa = WalkDir::new(search_path)
        .into_iter()
        .filter_entry(|e| is_visible(e))
        .filter_map(Result::ok);
    for entry in aa {
        if entry.file_name().to_string_lossy().contains(target_name) {
            results.push(entry.path().display().to_string());
        }
    }
    results
}

fn is_visible(entry: &DirEntry) -> bool {
    // 过滤掉以 '.' 开头的文件夹
    let file_name = entry.file_name().to_str().unwrap_or("");

    // println!("文件夹名称：{}", entry.path().display());
    // 判断是否为隐藏文件夹或文件
    if file_name.starts_with('.') {
        return false;
    }

    // 检查是否为驱动器根目录（如 D:\）
    if entry.path().parent().is_none() {
        return true; // 驱动器根目录始终视为可见
    }

    // 检查文件夹是否为隐藏文件夹（Windows平台）
    if cfg!(windows) {
        // 获取文件元数据
        if let Ok(metadata) = fs::metadata(entry.path()) {
            // 检查是否有 "hidden" 属性
            let file_attributes = metadata.file_attributes();
            if file_attributes & 0x2 != 0 {
                // FILE_ATTRIBUTE_HIDDEN (0x2)
                return false;
            }
        }
    }

    true
}
