use oxedize_fe2o3_text::{
    pattern::{
        MatcherNode,
        MatcherOp,
        Matcher,
    },
};

use oxedize_fe2o3_core::{
    prelude::*,
    test::test_it,
};


pub fn test_pattern(filter: &'static str) -> Outcome<()> {

    res!(test_it(filter, &["Starts with", "all", "match"], || {

        let root = MatcherNode::new_leaf(MatcherOp::StartsWith("he".to_string()));
        let mut matcher = Matcher::new(root);

        let input = "hello world hello";
        let mut all_results = Vec::new();

        for c in input.chars() {
            let results = matcher.process_char(c);
            all_results.extend(results);
        }

        test!("Buffer: {}", matcher.buffer);
        test!("Results: {:?}", all_results);

        Ok(())

    }));

    Ok(())
}