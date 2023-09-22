use trycmd::TestCases;

#[test]
fn trycmd() {
    let t = TestCases::new();
    t.case("examples/*.md");
}
