use std::{
    collections::HashSet,
    fs::{self, DirEntry},
    io::Error,
    str::FromStr,
};

use strum::IntoEnumIterator;

use crate::{
    aspect::Aspect,
    npc::{narrator::NarratorDialogue, NpcDialogue},
    ui::dialogue::runner::{IMA_FINAL_DIALOGUE, IMA_FIRST_ENCOUNTER, IMA_FIRST_ENCOUNTER_SHORT},
};

const PATH_TO_DIR: &str = "assets/dialogue";
const VALID_COMMANDS: [&str; 2] = ["<<trigger_ending>>", "<<game_over>>"];

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
fn validate_npc_names() {
    validate_lines(|line, _| {
        if let Some((possible_name, possible_message)) = line.split_once(' ') {
            if let Some(name) = possible_name.strip_suffix(':') {
                let name = name.trim_start_matches('_');
                if name == "title" {
                    return;
                }
                NpcDialogue::from_str(name).expect(&format!("Not a valid npc name, {}", name));
            } else {
                if possible_message.contains(':') {
                    panic!("The line, '{line}', contains a ':' in the message but has no character title. This is really bad.");
                }
            }
        }
    });
}

#[test]
fn validate_commands() {
    validate_lines(|line, _| {
        if line.starts_with("<<") {
            assert!(
                VALID_COMMANDS.contains(&line),
                "Only supported commands are {:?}, but got '{line}'",
                VALID_COMMANDS
            );
        }
    });
}

#[test]
fn validate_node_title_aspect_matching() {
    let mut titles = Vec::new();
    for title in Aspect::iter() {
        titles.push(title.to_string());
    }
    let mut titles_hashset = HashSet::new();

    validate_lines(|line, _| {
        if let Some(title) = line.strip_prefix("title: ") {
            if titles.contains(&title.to_string()) {
                titles_hashset.insert(title.to_string());
            }
        }
    });
    assert!(titles.len() == titles_hashset.len(), "Length mismatch, not all aspects have their own title in yarn files, total of {} aspects exist, but only {} of those have a title in yarn", titles.len(), titles_hashset.len());
}

#[test]
fn validate_node_title_narrator_matching() {
    let mut titles = Vec::new();
    for title in NarratorDialogue::iter() {
        titles.push(title.to_string());
    }
    let mut titles_hashset = HashSet::new();

    validate_lines(|line, _| {
        if let Some(title) = line.strip_prefix("title: ") {
            if titles.contains(&title.to_string()) {
                titles_hashset.insert(title.to_string());
            }
        }
    });
    assert!(titles.len() == titles_hashset.len(), "Length mismatch, not all endings have their own title in yarn files, total of {} endings exist, but only {} of those have a title in yarn", titles.len(), titles_hashset.len());
}

#[test]
fn validate_ima_titles_exist() {
    let titles = vec![
        IMA_FINAL_DIALOGUE,
        IMA_FIRST_ENCOUNTER,
        IMA_FIRST_ENCOUNTER_SHORT,
    ];
    let mut titles_hashset = HashSet::new();

    validate_lines(|line, _| {
        if let Some(title) = line.strip_prefix("title: ") {
            if titles.contains(&title.to_string().as_str()) {
                titles_hashset.insert(title.to_string());
            }
        }
    });
    assert!(titles.len() == titles_hashset.len(), "Length mismatch, not all endings have their own title in yarn files, total of {} endings exist, but only {} of those have a title in yarn", titles.len(), titles_hashset.len());
}

/// This test ensures that all yarn files only jump to nodes that are within that file.
/// While it works to jump to nodes that are definied in other files,
/// I don't see any reason to make use of this feature in this game.
/// It seems like it will just lead to bugs.
#[test]
fn validate_jump_node_exists() {
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
