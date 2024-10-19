use std::path::PathBuf;

use arbor::tree;
use rstest::*;

#[fixture]
pub fn partial_tree() -> tree::Tree {
    tree::TreeClimber::new().path("./tests/1").climb().unwrap()
}

#[fixture]
pub fn full_tree() -> tree::Tree {
    tree::TreeClimber::new()
        .path("./tests/1")
        .all(true)
        .climb()
        .unwrap()
}

#[rstest]
fn test_default_tree() {
    // Default tree should work
    let t = tree::TreeClimber::new().climb();
    assert!(t.is_ok());

    // And have at least some path
    let t = t.unwrap();
    assert!(t.path_count() > 0);

    // And that first path should be
    println!("{}", t.paths[0].display());
    assert!(t.paths[0].to_str().unwrap() == ".");
}

#[rstest]
fn test_partial_tree(partial_tree: tree::Tree) {
    assert!(partial_tree.path_count() == 3);
}

#[rstest]
fn test_full_tree(full_tree: tree::Tree) {
    assert!(full_tree.path_count() == 4);
}
