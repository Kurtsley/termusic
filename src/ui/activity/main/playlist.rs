use super::{MainActivity, COMPONENT_TREEVIEW};

use std::path::Path;
use std::thread;
use tui_realm_treeview::TreeViewPropsBuilder;
use tui_realm_treeview::{Node, Tree};
use tuirealm::{Payload, PropsBuilder, Value};
use ytd_rs::{Arg, ResultType, YoutubeDL};

impl MainActivity {
    pub fn scan_dir(&mut self, p: &Path) {
        self.path = p.to_path_buf();
        self.tree = Tree::new(Self::dir_tree(p, 3));
    }

    pub fn upper_dir(&self) -> Option<&Path> {
        self.path.parent()
    }

    pub fn dir_tree(p: &Path, depth: usize) -> Node {
        let name: String = match p.file_name() {
            None => "/".to_string(),
            Some(n) => n.to_string_lossy().into_owned().to_string(),
        };
        let mut node: Node = Node::new(p.to_string_lossy().into_owned(), name);
        if depth > 0 && p.is_dir() {
            if let Ok(e) = std::fs::read_dir(p) {
                e.flatten()
                    .for_each(|x| node.add_child(Self::dir_tree(x.path().as_path(), depth - 1)));
            }
        }
        node
    }

    pub fn dir_children(p: &Path) -> Vec<String> {
        let mut children: Vec<String> = vec![];
        if p.is_dir() {
            if let Ok(e) = std::fs::read_dir(p) {
                e.flatten().for_each(|x| {
                    if x.path().is_dir() {
                    } else {
                        children.push(String::from(x.path().to_string_lossy()));
                    }
                });
            }
        }
        children
    }

    pub fn refresh_playlist(&mut self) {
        self.tree = Tree::new(Self::dir_tree(self.path.as_ref(), 3));
        let props = TreeViewPropsBuilder::from(self.view.get_props(COMPONENT_TREEVIEW).unwrap())
            .with_tree(self.tree.root())
            .with_title(Some(String::from(self.path.to_string_lossy())))
            .build();

        let msg = self.view.update(COMPONENT_TREEVIEW, props);
        self.update(msg);
    }

    pub fn youtube_dl(&mut self, link: String) {
        let mut path: String = String::from("abc");
        match self.view.get_state(COMPONENT_TREEVIEW) {
            Some(Payload::One(Value::Str(node_id))) => {
                let p: &Path = Path::new(node_id.as_str());
                if p.is_dir() {
                    path = String::from(p.to_string_lossy());
                } else {
                    path = String::from(p.parent().unwrap().to_string_lossy());
                }
            }
            _ => {}
        }

        let args = vec![
            // Arg::new("--quiet"),
            Arg::new("--extract-audio"),
            Arg::new_with_arg("--audio-format", "mp3"),
            Arg::new("--add-metadata"),
            Arg::new("--embed-thumbnail"),
            Arg::new_with_arg("--metadata-from-title", "%(artist) - %(title)s"),
            Arg::new("--write-sub"),
            Arg::new("--all-subs"),
            Arg::new_with_arg("--convert-subs", "lrc"),
            Arg::new_with_arg("--output", "%(title).90s.%(ext)s"),
        ];
        let ytd = YoutubeDL::new(path.as_ref(), args, link.as_ref()).unwrap();
        let tx = self.sender.clone();

        thread::spawn(move || {
            tx.send(super::TransferState::Running).unwrap();
            // start download
            let download = ytd.download();

            // check what the result is and print out the path to the download or the error
            match download.result_type() {
                ResultType::SUCCESS => {
                    // println!("Your download: {}", download.output_dir().to_string_lossy())
                    tx.send(super::TransferState::Completed).unwrap();
                }
                ResultType::IOERROR | ResultType::FAILURE => {
                    // println!("Couldn't start download: {}", download.output())
                    tx.send(super::TransferState::ErrDownload).unwrap();
                }
            };
        });
    }
}