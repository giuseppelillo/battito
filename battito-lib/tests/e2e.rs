#[cfg(test)]
mod tests {
    use battito_lib::pattern::pattern::{Pattern, TimedEvent};
    use battito_lib::pattern::transform;

    fn test(first: &str, second: &str) {
        let one = transform(first, None);
        let two = transform(second, None);
        assert_eq!(one, two);
    }

    #[test]
    fn one_measure() {
        let out = transform("1 2 3", None);
        let expected = Ok(Pattern {
            steps: vec![
                TimedEvent::new(1, "1", 100),
                TimedEvent::new(641, "2", 100),
                TimedEvent::new(1281, "3", 100),
            ],
            length: 1,
            subdivision: 1920,
        });
        assert_eq!(expected, out);
    }

    #[test]
    fn two_measures() {
        let out = transform("1 2 3 | 4 5 6 7", None);
        let expected = Ok(Pattern {
            steps: vec![
                TimedEvent::new(1, "1", 100),
                TimedEvent::new(641, "2", 100),
                TimedEvent::new(1281, "3", 100),
                TimedEvent::new(1921, "4", 100),
                TimedEvent::new(2401, "5", 100),
                TimedEvent::new(2881, "6", 100),
                TimedEvent::new(3361, "7", 100),
            ],
            length: 2,
            subdivision: 1920,
        });
        assert_eq!(expected, out);
    }

    #[test]
    fn alternate() {
        test("1 <2,4> 3 | 5 6", "1 2 3 | 1 4 3 | 5 6");
        test("1 <2,4> <3,6>", "1 2 3 | 1 4 6");
        test("<1,2> <3,3,4,4>", "1 3 | 2 3 | 1 4 | 2 4");
        test("<1,1,2,2> <3,4>", "1 3 | 1 4 | 2 3 | 2 4");
        test("<1,2,3> <4,5>", "1 4 | 2 5 | 3 4 | 1 5 | 2 4 | 3 5");
    }

    #[test]
    fn polymetric() {
        test("{1 2 3 4}%5", "1 2 3 4 1 | 2 3 4 1 2 | 3 4 1 2 3 | 4 1 2 3 4");
        test(
            "{1 2 3 4 5 6}%5",
            "1 2 3 4 5 | 6 1 2 3 4 | 5 6 1 2 3 | 4 5 6 1 2 | 3 4 5 6 1 | 2 3 4 5 6",
        );
    }

    #[test]
    fn polymetric_even() {
        test("{1 2}%4", "1 2 1 2");
        test("{1 2 3 4}%4", "1 2 3 4");
        test("{1 2 3 4 5 6 7 8}%4", "1 2 3 4 | 5 6 7 8");
    }

    #[test]
    fn polymetric_alternate() {
        test("{1 2 <3,5,6> 4}%5", "1 2 3 4 1 | 2 5 4 1 2 | 6 4 1 2 3 | 4 1 2 5 4 | 1 2 6 4 1 | 2 3 4 1 2 | 5 4 1 2 6 | 4 1 2 3 4 | 1 2 5 4 1 | 2 6 4 1 2 | 3 4 1 2 5 | 4 1 2 6 4");
        test("{1 2 <3,[5 6]> 4}%5", "1 2 3 4 1 | 2 [5 6] 4 1 2 | 3 4 1 2 [5 6] | 4 1 2 3 4 | 1 2 [5 6] 4 1 | 2 3 4 1 2 | [5 6] 4 1 2 3 | 4 1 2 [5 6] 4");
        test("{1 2 <3,5> 4}%4", "1 2 3 4 | 1 2 5 4");
        test(
            "{1 2 <3,4> <5,6,7>}%4",
            "1 2 3 5 | 1 2 4 6 | 1 2 3 7 | 1 2 4 5 | 1 2 3 6 | 1 2 4 7",
        );
        test(
            "{1 2 9 <3,4> <5,6,7>}%3",
            "1 2 9 | 3 5 1 | 2 9 4 | 6 1 2 | 9 3 7 | 1 2 9 | 4 5 1 | 2 9 3 | 6 1 2 | 9 4 7",
        );
    }

    #[test]
    fn euclidean() {
        test("b(3,8,0) h", "[b ~ ~ b ~ ~ b ~] h");
        test("b(3,8,1) h", "[~ b ~ ~ b ~ ~ b] h");
        test("[s [h b(3,8,1)]] h", "[s [h [~ b ~ ~ b ~ ~ b]]] h");
        test("b(<3,4>,8,0) h", "[b ~ ~ b ~ ~ b ~] h | [b ~ b ~ b ~ b ~] h");
        test(
            "[s [h b(<3,4>,8,0)]] h",
            "[s [h [b ~ ~ b ~ ~ b ~]]] h | [s [h [b ~ b ~ b ~ b ~]]] h",
        );
        test(
            "b(<1,2,4>,<4,8>,<0,1>) h",
            "[b ~ ~ ~] h | [~ b ~ ~ ~ b ~ ~] h | [b b b b] h | [~ b ~ ~ ~ ~ ~ ~] h | [b ~ b ~] h | [~ b ~ b ~ b ~ b] h",
        );
    }

    #[test]
    fn euclidean_paper() {
        test("b(1,2)", "b ~");
        test("b(1,3)", "b ~ ~");
        test("b(1,4)", "b ~ ~ ~");
        test("b(4,12)", "b ~ ~ b ~ ~ b ~ ~ b ~ ~");
        test("b(2,3)", "b ~ b");
        test("b(2,5)", "b ~ b ~ ~");
        test("b(3,4)", "b ~ b b");
        test("b(3,5)", "b ~ b ~ b");
        test("b(3,7)", "b ~ b ~ b ~ ~");
        test("b(4,7)", "b ~ b ~ b ~ b");
        test("b(4,9)", "b ~ b ~ b ~ b ~ ~");
        test("b(4,11)", "b ~ ~ b ~ ~ b ~ ~ b ~");
        test("b(5,6)", "b ~ b b b b");
        // this is broken
        // test("b(5,7)", "b ~ b b ~ b b");
    }

    #[test]
    fn repeated() {
        test("b*2", "b b");
        test("b*2 s", "[b b] s");
        test("[b [s h [h s] b]]*2 s", "[[b [s h [h s] b]] [b [s h [h s] b]]] s");
        test(
            "[b [s h [h s]*3 b]]*2 s",
            "[[b [s h [[h s] [h s] [h s]] b]] [b [s h [[h s] [h s] [h s]] b]]] s",
        );
        test("<b,h>*2 s", "[<b,h> <b,h>] s");
        test("<b,h>*2 s", "[b b] s | [h h] s");
        test("[b(3,4)]*2", "[b(3,4) b(3,4)]");
        test("[b(3,4)]*2", "b(3,4)*2");
    }

    #[test]
    fn replicated() {
        test("b!2", "b b");
        test("b!2 s", "b b s");
        test("[b [s h [h s] b]]!2 s", "[b [s h [h s] b]] [b [s h [h s] b]] s");
        test(
            "[b [s h [h s]!3 b]]!2 s",
            "[b [s h [h s] [h s] [h s] b]] [b [s h [h s] [h s] [h s] b]] s",
        );
        test("<b,h>!2 s", "<b,h> <b,h> s");
        test("<b,h>!2 s", "b b s | h h s");
        test("[b(3,4)]!2", "b(3,4) b(3,4)");
        test("[b(3,4)]!2", "b(3,4)!2");
    }

    #[test]
    fn probability() {
        let out = transform("1 2?25 3 4", None);
        let expected = Ok(Pattern {
            steps: vec![
                TimedEvent::new(1, "1", 100),
                TimedEvent::new(481, "2", 25),
                TimedEvent::new(961, "3", 100),
                TimedEvent::new(1441, "4", 100),
            ],
            length: 1,
            subdivision: 1920,
        });
        assert_eq!(expected, out);

        let out = transform("1 [3?22 4] 3 | 5 6", None);
        let expected = Ok(Pattern {
            steps: vec![
                TimedEvent::new(1, "1", 100),
                TimedEvent::new(641, "3", 22),
                TimedEvent::new(961, "4", 100),
                TimedEvent::new(1281, "3", 100),
                TimedEvent::new(1921, "5", 100),
                TimedEvent::new(2881, "6", 100),
            ],
            length: 2,
            subdivision: 1920,
        });
        assert_eq!(expected, out);

        test("1 2?40!2 3", "1 2?40 2?40 3");
        test("1 2?40*2 3", "1 [2?40 2?40] 3");
        test("1 [2?40*2 5] 3", "1 [[2?40 2?40] 5] 3");
        test("1 <2,4?22> 3 | 5 6", "1 2 3 | 1 4?22 3 | 5 6");
        test(
            "{1 2 3?22 4}%5",
            "1 2 3?22 4 1 | 2 3?22 4 1 2 | 3?22 4 1 2 3?22 | 4 1 2 3?22 4",
        );
        test(
            "{1 2 <3,4?21> <5,6,7>}%4",
            "1 2 3 5 | 1 2 4?21 6 | 1 2 3 7 | 1 2 4?21 5 | 1 2 3 6 | 1 2 4?21 7",
        );
        test("b?30(3,8,0) h", "[b?30 ~ ~ b?30 ~ ~ b?30 ~] h");

        /* Not yet implemented: */
        // test("1 [2 4]?22 3 | 5 6", "1 [2?22 4?22] 3 | 5 6");
        // test("1 <2,4>?22 3 | 5 6", "1 2?22 3 | 1 4?22 3 | 5 6");
    }
}
