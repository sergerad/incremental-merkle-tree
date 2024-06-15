use imt::Builder;
use imt::Height;

#[test]
fn something() {
    let _imt: imt::Imt = Builder::default()
        .height(Height::try_from(32).expect("todo"))
        .build()
        .expect("build");
}
