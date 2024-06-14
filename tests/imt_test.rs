use imt::Builder;

#[test]
fn something() {
    let _imt = Builder::default()
        .height(32)
        .expect("expected valid height")
        .build();
}
