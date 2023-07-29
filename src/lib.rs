use mdbook::book::{Book, BookItem, Chapter};
use mdbook::errors::Error;
use mdbook::preprocess::PreprocessorContext;
use std::ffi::OsStr;
use std::io::Write;
use std::process::{Command as ProcessCommand, Stdio};

fn nixdoc(chapter: &mut Chapter) -> Result<(), Error> {
    let mut child = ProcessCommand::new("nixdoc")
        .arg("--category")
        .arg(chapter.name.clone())
        .arg("--description")
        .arg(chapter.name.clone())
        .arg("--file")
        .arg("/dev/stdin")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn child process");

    let content = chapter.content.clone();

    let mut stdin = child.stdin.take().expect("Failed to open stdin");
    std::thread::spawn(move || {
        stdin
            .write_all(content.as_bytes())
            .expect("Failed to write to stdin");
    });

    let output = child.wait_with_output().expect("Failed to read stdout");

    chapter.content = String::from_utf8(output.stdout).expect("Found invalid UTF-8");

    Ok(())
}

pub struct Preprocessor;

impl mdbook::preprocess::Preprocessor for Preprocessor {
    fn name(&self) -> &str {
        "nixdoc"
    }

    fn run(&self, _ctx: &PreprocessorContext, mut book: Book) -> Result<Book, Error> {
        book.for_each_mut(|item: &mut BookItem| {
            if let BookItem::Chapter(ref mut chapter) = *item {
                if let Some(source_path) = &chapter.source_path {
                    if let Some(extension) = source_path.extension().and_then(OsStr::to_str) {
                        if extension == "nix" {
                            nixdoc(chapter).expect("Nixdoc failed");
                        }
                    }
                }
            }
        });

        Ok(book)
    }

    fn supports_renderer(&self, renderer: &str) -> bool {
        renderer != "not-supported"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mdbook::preprocess::Preprocessor;

    #[test]
    fn nop_preprocessor_run() {
        let input_json = r##"[
            {
              "root": "/home/adisbladis/sauce/github.com/adisbladis/pyproject.nix/doc",
              "config": {
                "book": {
                  "authors": [
                    "adisbladis"
                  ],
                  "language": "en",
                  "multilingual": false,
                  "src": "src",
                  "title": "Pyproject.nix"
                },
                "output": {
                  "html": {
                    "git-branch": "master",
                    "git-repository-url": "https://github.com/adisbladis/pyproject.nix",
                    "open-on-text": "Found an issue? [Edit this page on GitHub.]"
                  }
                },
                "preprocessor": {
                  "nixdoc": {
                    "command": "mdbook-nixdoc"
                  }
                }
              },
              "renderer": "html",
              "mdbook_version": "0.4.31"
            },
            {
              "sections": [
                {
                  "Chapter": {
                    "name": "Get wiggly with it",
                    "content": "{\n  /* One wiggly donker a day keeps the doctor at bay.\n\n  Type: funcy :: AttrSet -> string\n\n  Example:\n    funcy \"UUU\" \"www\" \"UUU\"\n  */\n  funcy =\n    im:\n    an:\n    arg: \"UwU\";\n}\n",
                    "number": [
                      1
                    ],
                    "sub_items": [
                      {
                        "Chapter": {
                          "name": "Some of that nested wiggle",
                          "content": "{\n  /* One wiggly donker a day keeps the doctor at bay.\n\n  Type: funcy :: AttrSet -> string\n\n  Example:\n    funcy \"UUU\" \"www\" \"UUU\"\n  */\n  funcy =\n    im:\n    an:\n    arg: \"UwU\";\n}\n",
                          "number": [
                            1,
                            1
                          ],
                          "sub_items": [],
                          "path": "wiggly.nix",
                          "source_path": "wiggly.nix",
                          "parent_names": [
                            "Get wiggly with it"
                          ]
                        }
                      }
                    ],
                    "path": "wiggly.nix",
                    "source_path": "wiggly.nix",
                    "parent_names": []
                  }
                }
              ],
              "__non_exhaustive": null
            }
        ]"##;
        let input_json = input_json.as_bytes();

        let (ctx, book) = mdbook::preprocess::CmdPreprocessor::parse_input(input_json).unwrap();
        let expected_book = book.clone();
        let preprocessor = Preprocessor;
        let result = preprocessor.run(&ctx, book);
        assert!(result.is_ok());

        // Assert that nixdoc did _some_ changes
        // This test should be better but for now it's what I have the energy for.
        let actual_book = result.unwrap();
        assert_ne!(actual_book, expected_book);
    }
}
