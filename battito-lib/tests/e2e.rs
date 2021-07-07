#[cfg(test)]
mod tests {
    use battito_lib::error::Error;
    use battito_lib::interpreter::interpret;
    use battito_lib::interpreter::RunConfig;
    use battito_lib::max::Payload;
    use battito_lib::SUBDIVISION_DEFAULT;

    fn run(input: &str) -> Result<Payload, Error> {
        let run_config = RunConfig {
            subdivision: SUBDIVISION_DEFAULT,
        };
        interpret(input, &run_config)
    }

    fn test(expected: &Result<Payload, Error>, out: &Result<Payload, Error>) {
        assert_eq!(expected.as_ref().unwrap().steps, out.as_ref().unwrap().steps);
        assert_eq!(expected.as_ref().unwrap().length, out.as_ref().unwrap().length);
    }

    #[test]
    fn one_measure() {
        let out = run("a $ 1 2 3");
        let expected = Ok(Payload {
            target: "a".to_string(),
            steps: "1 1 100, 641 2 100, 1281 3 100".to_string(),
            length: 1,
            pattern: "1 2 3".to_string(),
        });
        assert_eq!(expected, out);
    }

    #[test]
    fn two_measures() {
        let out = run("a $ 1 2 3 | 4 5 6 7");
        let expected = Ok(
            Payload {
                target: "a".to_string(),
                steps: "1 1 100, 641 2 100, 1281 3 100, 1921 4 100, 2401 5 100, 2881 6 100, 3361 7 100".to_string(),
                length: 2,
                pattern: "1 2 3 | 4 5 6 7".to_string(),
            }
        );
        assert_eq!(expected, out);
    }

    #[test]
    fn alternate() {
        let out = run("a $ 1 <2,4> 3 | 5 6");
        let expected = run("a $ 1 2 3 | 1 4 3 | 5 6");
        test(&expected, &out);

        let out = run("a $ 1 <2,4> <3,6>");
        let expected = run("a $ 1 2 3 | 1 4 6");
        test(&expected, &out);

        let out = run("a $ <1,2> <3,3,4,4>");
        let expected = run("a $ 1 3 | 2 3 | 1 4 | 2 4");
        test(&expected, &out);

        let out = run("a $ <1,1,2,2> <3,4>");
        let expected = run("a $ 1 3 | 1 4 | 2 3 | 2 4");
        test(&expected, &out);

        let out = run("a $ <1,2,3> <4,5>");
        let expected = run("a $ 1 4 | 2 5 | 3 4 | 1 5 | 2 4 | 3 5");
        test(&expected, &out);
    }

    #[test]
    fn polymetric() {
        let out = run("a $ {1 2 3 4}%5");
        let expected = run("a $ 1 2 3 4 1 | 2 3 4 1 2 | 3 4 1 2 3 | 4 1 2 3 4");
        test(&expected, &out);

        let out = run("a $ {1 2 3 4 5 6}%5");
        let expected = run("a $ 1 2 3 4 5 | 6 1 2 3 4 | 5 6 1 2 3 | 4 5 6 1 2 | 3 4 5 6 1 | 2 3 4 5 6");
        test(&expected, &out);
    }

    #[test]
    fn polymetric_even() {
        let out = run("a $ {1 2}%4");
        let expected = run("a $ 1 2 1 2");
        test(&expected, &out);

        let out = run("a $ {1 2 3 4}%4");
        let expected = run("a $ 1 2 3 4");
        test(&expected, &out);

        let out = run("a $ {1 2 3 4 5 6 7 8}%4");
        let expected = run("a $ 1 2 3 4 | 5 6 7 8");
        test(&expected, &out);
    }

    #[test]
    fn polymetric_alternate() {
        let out = run("a $ {1 2 <3,5,6> 4}%5");
        let expected = run("a $ 1 2 3 4 1 | 2 5 4 1 2 | 6 4 1 2 3 | 4 1 2 5 4 | 1 2 6 4 1 | 2 3 4 1 2 | 5 4 1 2 6 | 4 1 2 3 4 | 1 2 5 4 1 | 2 6 4 1 2 | 3 4 1 2 5 | 4 1 2 6 4");
        test(&expected, &out);

        let out = run("a $ {1 2 <3,[5 6]> 4}%5");
        let expected = run("a $ 1 2 3 4 1 | 2 [5 6] 4 1 2 | 3 4 1 2 [5 6] | 4 1 2 3 4 | 1 2 [5 6] 4 1 | 2 3 4 1 2 | [5 6] 4 1 2 3 | 4 1 2 [5 6] 4");
        test(&expected, &out);

        let out = run("a $ {1 2 <3,5> 4}%4");
        let expected = run("a $ 1 2 3 4 | 1 2 5 4");
        test(&expected, &out);

        let out = run("a $ {1 2 <3,4> <5,6,7>}%4");
        let expected = run("a $ 1 2 3 5 | 1 2 4 6 | 1 2 3 7 | 1 2 4 5 | 1 2 3 6 | 1 2 4 7");
        test(&expected, &out);

        let out = run("a $ {1 2 9 <3,4> <5,6,7>}%3");
        let expected = run("a $ 1 2 9 | 3 5 1 | 2 9 4 | 6 1 2 | 9 3 7 | 1 2 9 | 4 5 1 | 2 9 3 | 6 1 2 | 9 4 7");
        test(&expected, &out);
    }

    #[test]
    fn euclidean() {
        let out = run("a $ b(3,8,0) h");
        let expected = run("a $ [b ~ ~ b ~ ~ b ~] h");
        test(&expected, &out);

        let out = run("a $ b(3,8,1) h");
        let expected = run("a $ [~ b ~ ~ b ~ ~ b] h");
        test(&expected, &out);

        let out = run("a $ [s [h b(3,8,1)]] h");
        let expected = run("a $ [s [h [~ b ~ ~ b ~ ~ b]]] h");
        test(&expected, &out);

        let out = run("a $ b(<3,4>,8,0) h");
        let expected = run("a $ [b ~ ~ b ~ ~ b ~] h | [b ~ b ~ b ~ b ~] h");
        test(&expected, &out);

        let out = run("a $ [s [h b(<3,4>,8,0)]] h");
        let expected = run("a $ [s [h [b ~ ~ b ~ ~ b ~]]] h | [s [h [b ~ b ~ b ~ b ~]]] h");
        test(&expected, &out);

        let out = run("a $ b(<1,2,4>,<4,8>,<0,1>) h");
        let expected = run(
            "a $ [b ~ ~ ~] h | [~ b ~ ~ ~ b ~ ~] h | [b b b b] h | [~ b ~ ~ ~ ~ ~ ~] h | [b ~ b ~] h | [~ b ~ b ~ b ~ b] h",
        );
        test(&expected, &out);
    }



    #[test]
    fn euclidean_paper() {
        let out = run("a $ b(1,2)");
        let expected = run("a $ b ~");
        test(&expected, &out);
        let out = run("a $ b(1,3)");
        let expected = run("a $ b ~ ~");
        test(&expected, &out);
        let out = run("a $ b(1,4)");
        let expected = run("a $ b ~ ~ ~");
        test(&expected, &out);
        let out = run("a $ b(4,12)");
        let expected = run("a $ b ~ ~ b ~ ~ b ~ ~ b ~ ~ ");
        test(&expected, &out);
        let out = run("a $ b(2,3)");
        let expected = run("a $ b ~ b");
        test(&expected, &out);
        let out = run("a $ b(2,5)");
        let expected = run("a $ b ~ b ~ ~");
        test(&expected, &out);
        let out = run("a $ b(3,4)");
        let expected = run("a $ b ~ b b");
        test(&expected, &out);
        let out = run("a $ b(3,5)");
        let expected = run("a $ b ~ b ~ b");
        test(&expected, &out);
        let out = run("a $ b(3,7)");
        let expected = run("a $ b ~ b ~ b ~ ~");
        test(&expected, &out);
        let out = run("a $ b(4,7)");
        let expected = run("a $ b ~ b ~ b ~ b");
        test(&expected, &out);
        let out = run("a $ b(4,9)");
        let expected = run("a $ b ~ b ~ b ~ b ~ ~");
        test(&expected, &out);
        let out = run("a $ b(4,11)");
        let expected = run("a $ b ~ ~ b ~ ~ b ~ ~ b ~");
        test(&expected, &out);
        let out = run("a $ b(5,6)");
        let expected = run("a $ b ~ b b b b");
        test(&expected, &out);
        // this is broken
        // let out = run("a $ b(5,7)");
        // let expected = run("a $ b ~ b b ~ b b");
        // test(&expected, &out);
    }

    #[test]
    fn repeated() {
        let out = run("a $ b*2");
        let expected = run("a $ b b");
        test(&expected, &out);
        let out = run("a $ b*2 s");
        let expected = run("a $ [b b] s");
        test(&expected, &out);
        let out = run("a $ [b [s h [h s] b]]*2 s");
        let expected = run("a $ [[b [s h [h s] b]] [b [s h [h s] b]]] s");
        test(&expected, &out);
        let out = run("a $ [b [s h [h s]*3 b]]*2 s");
        let expected = run("a $ [[b [s h [[h s] [h s] [h s]] b]] [b [s h [[h s] [h s] [h s]] b]]] s");
        test(&expected, &out);
        let out = run("a $ <b,h>*2 s");
        let expected = run("a $ [<b,h> <b,h>] s");
        test(&expected, &out);
        let expected2 = run("a $ [b b] s | [h h] s");
        test(&expected2, &out);
        let out = run("a $ [b(3,4)]*2");
        let expected = run("a $ [b(3,4) b(3,4)]");
        test(&expected, &out);
        let expected2 = run("a $ b(3,4)*2");
        test(&expected2, &out);
    }

    #[test]
    fn replicated() {
        let out = run("a $ b!2");
        let expected = run("a $ b b");
        test(&expected, &out);
        let out = run("a $ b!2 s");
        let expected = run("a $ b b s");
        test(&expected, &out);
        let out = run("a $ [b [s h [h s] b]]!2 s");
        let expected = run("a $ [b [s h [h s] b]] [b [s h [h s] b]] s");
        test(&expected, &out);
        let out = run("a $ [b [s h [h s]!3 b]]!2 s");
        let expected = run("a $ [b [s h [h s] [h s] [h s] b]] [b [s h [h s] [h s] [h s] b]] s");
        test(&expected, &out);
        let out = run("a $ <b,h>!2 s");
        let expected = run("a $ <b,h> <b,h> s");
        test(&expected, &out);
        let expected2 = run("a $ b b s | h h s");
        test(&expected2, &out);
        let out = run("a $ [b(3,4)]!2");
        let expected = run("a $ b(3,4) b(3,4)");
        test(&expected, &out);
        let expected2 = run("a $ b(3,4)!2");
        test(&expected2, &out);
    }

    #[test]
    fn probability() {
        let out = run("a $ 1 2?25 3 4");
        let expected = Ok(Payload {
            target: "a".to_string(),
            steps: "1 1 100, 481 2 25, 961 3 100, 1441 4 100".to_string(),
            length: 1,
            pattern: "1 2?25 3 4".to_string(),
        });
        assert_eq!(expected, out);

        let out = run("a $ 1 [3?22 4] 3 | 5 6");
        let expected = Ok(
            Payload {
                target: "a".to_string(),
                steps: "1 1 100, 641 3 22, 961 4 100, 1281 3 100, 1921 5 100, 2881 6 100".to_string(),
                length: 2,
                pattern: "1 [3?22 4] 3 | 5 6".to_string(),
            }
        );
        assert_eq!(expected, out);

        let out = run("a $ 1 2?40!2 3");
        let expected = run("a $ 1 2?40 2?40 3");
        test(&expected, &out);

        let out = run("a $ 1 2?40*2 3");
        let expected = run("a $ 1 [2?40 2?40] 3");
        test(&expected, &out);

        let out = run("a $ 1 [2?40*2 5] 3");
        let expected = run("a $ 1 [[2?40 2?40] 5] 3");
        test(&expected, &out);

        let out = run("a $ 1 <2,4?22> 3 | 5 6");
        let expected = run("a $ 1 2 3 | 1 4?22 3 | 5 6");
        test(&expected, &out);

        let out = run("a $ {1 2 3?22 4}%5");
        let expected = run("a $ 1 2 3?22 4 1 | 2 3?22 4 1 2 | 3?22 4 1 2 3?22 | 4 1 2 3?22 4");
        test(&expected, &out);

        let out = run("a $ {1 2 <3,4?21> <5,6,7>}%4");
        let expected = run("a $ 1 2 3 5 | 1 2 4?21 6 | 1 2 3 7 | 1 2 4?21 5 | 1 2 3 6 | 1 2 4?21 7");
        test(&expected, &out);

        let out = run("a $ b?30(3,8,0) h");
        let expected = run("a $ [b?30 ~ ~ b?30 ~ ~ b?30 ~] h");
        test(&expected, &out);

        /* Not yet implemented:

        let out = run("a $ 1 [2 4]?22 3 | 5 6");
        let expected = run("a $ 1 [2?22 4?22] 3 | 5 6");
        test(&expected, &out);

        let out = run("a $ 1 <2,4>?22 3 | 5 6");
        let expected = run("a $ 1 2?22 3 | 1 4?22 3 | 5 6");
        test(&expected, &out);

        */
    }
}
