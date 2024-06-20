#[test]
fn default_tree_has_correct_root() {
    let imt: imt::Tree<sha2::Sha256> = imt::Builder::default().build().expect("Failed to build");

    assert_eq!(
        format!("{:x}", imt.root()),
        "985e929f70af28d0bdd1a90a808f977f597c7c778c489e98d3bd8910d31ac0f7"
    );
}

#[test]
fn add_leaves_tree_has_correct_roots() {
    let tests = [
        (
            "01",
            "3cfcfa4027036c7ba5a8ad02bfa8f15b14c5c7a98d93e8c37989b84123b4e861",
        ),
        (
            "02",
            "459355566f74cc35f00254a8218749d95498c1abe7599f125be3af46f4c14b7b",
        ),
        (
            "03",
            "6527710706c0095dd89714a229c192dbf0cc9377ad0cf25bfd3c3393a0e37012",
        ),
        (
            "04",
            "910a56f8a8da7dc2ce46055b53efb08e8df5bcdb3862b7a0f3d8aa0209019f87",
        ),
    ];
    let mut imt: imt::Tree<sha2::Sha256> = imt::Builder::default()
        .height(imt::Height::try_from(4).unwrap())
        .build()
        .expect("Failed to build");
    for test in tests {
        imt.add_leaf(test.0).expect("Failed to add leaf");
        assert_eq!(format!("{:x}", imt.root()), test.1);
    }
}
