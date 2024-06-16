#[test]
fn something() {
    let mut imt: imt::Tree = imt::Builder::default()
        .height(imt::Height::try_from(32).expect("todo"))
        .build()
        .expect("build");

    imt.add_leaf(imt::Node::default()).expect("todo");
}
