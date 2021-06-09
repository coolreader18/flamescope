#[test]
fn test_basic() {
    flame::clear();
    {
        let _guard = flame::start_guard("the main");
        {
            let _guard_two = flame::start_guard("foobar");
        }
    }
    let actual = flamescope::spans_to_speedscope(flame::spans());
    insta::assert_json_snapshot!(actual, {
        ".profiles[].events[].at" => 100,
        ".profiles[].startValue" => 50,
        ".profiles[].endValue" => 150,
    });
}
