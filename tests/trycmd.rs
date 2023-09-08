use trycmd::TestCases;

#[test]
fn trycmd() {
    TestCases::new().case("tests/cmd/*.md");
}
