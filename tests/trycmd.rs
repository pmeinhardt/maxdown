use trycmd::TestCases;

#[test]
fn trycmd() {
    TestCases::new().case("examples/*.md");
}
