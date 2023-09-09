use trycmd::TestCases;

const CSS: &str = include_str!("../src/github.css");

#[test]
fn trycmd() {
    let t = TestCases::new();

    t.case("examples/*.md");

    t.extend_vars([("[CSS]", CSS)]).unwrap();
}
