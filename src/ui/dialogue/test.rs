use std::{
    collections::HashSet,
    fs::{self, DirEntry},
    io::Error,
};

use strum::IntoEnumIterator;

use crate::aspect::Aspect;

const PATH_TO_DIR: &str = "assets/dialogue";

fn try_read_yarn_contents(entry: Result<DirEntry, Error>) -> Option<(String, String)> {
    let entry = entry.expect("Can't get entry in current dir");
    let npc_file_name = entry
        .file_name()
        .into_string()
        .expect("Can't convert OsString to String")
        .split('.')
        .collect::<Vec<&str>>()[0]
        .to_string();
    let path = entry.path();

    if !path.is_file() {
        return None;
    }

    if let Some(ext) = path.extension() {
        if ext == "yarn" {
            return Some((
                (fs::read_to_string(path).expect("Should have been able to read the file")),
                npc_file_name,
            ));
        }
    }
    None
}

/// Loop over all yarn files in `PATH_TO_DIR` and apply the predicate on each line.
fn validate_lines<F>(mut predicate: F)
where
    F: FnMut(&str, &str),
{
    for entry in fs::read_dir(PATH_TO_DIR).expect("Can't read entries in current dir") {
        let (contents, npc_file_name) = match try_read_yarn_contents(entry) {
            Some(r) => r,
            None => continue,
        };

        for line in contents.lines().map(str::trim) {
            predicate(line.trim(), &npc_file_name)
        }
    }
}

#[test]
fn validate_node_title_uniqueness() {
    let mut titles = HashSet::new();

    validate_lines(|line, npc_file_name| {
        if let Some(title) = line.strip_prefix("title: ") {
            assert!(
                titles.insert(title.to_string()),
                "Title already exists! title: {title}, in file: {npc_file_name}",
            );
        }
    });
}

#[test]
fn validate_node_title_aspect_matching() {
    let mut aspect_titles = Vec::new();
    for aspect in Aspect::iter() {
        aspect_titles.push(aspect.to_string());
    }
    let mut aspect_hashset = HashSet::new();

    validate_lines(|line, _| {
        if let Some(title) = line.strip_prefix("title: ") {
            if aspect_titles.contains(&title.to_string()) {
                aspect_hashset.insert(title.to_string());
            }
        }
    });
    assert!(aspect_titles.len() == aspect_hashset.len(), "Length mismatch, not all aspect have their own title in yarn files, total of {} aspects exist, but only {} of those have a title in yarn", aspect_titles.len(), aspect_hashset.len());
}

/// This test ensures that all yarn files only jump to nodes that are within that file.
/// While it works to jump to nodes that are definied in other files,
/// I don't see any reason to make use of this feature in this game.
/// It seems like it will just lead to bugs.
#[test]
fn validate_node_exists() {
    fn format_title_with_file(title: &str, file: &str) -> String {
        format!("{title}-{file}")
    }

    let mut titles_with_file = HashSet::new();

    validate_lines(|line, npc_file_name| {
        if let Some(title) = line.strip_prefix("title: ") {
            titles_with_file.insert(format_title_with_file(title, npc_file_name));
        }
    });

    validate_lines(|line, npc_file_name| {
        if let Some(command) = line.strip_prefix("<<jump ") {
            let title = command.trim_end_matches(">>");
            let title_with_file = format_title_with_file(title, npc_file_name);
            assert!(
                titles_with_file.contains(&title_with_file),
                "Referenced title: '{title}' in file: '{npc_file_name}' doesn't exist! (jump command)"
            );
        }
    });
}
