use battito_lib::interpreter::interpret;
use battito_lib::max::Payload;

#[test]
fn one_measure() {
    let out = interpret("a $ 1 2 3");
    let expected = Ok(Payload {
        target: "a".to_string(),
        steps: "1 1 100 100 100, 641 2 100 100 100, 1281 3 100 100 100".to_string(),
        length: 1,
        subdivision: 1920,
    });
    assert_eq!(expected, out);
}

#[test]
fn two_measures() {
    let out = interpret("a $ 1 2 3 | 4 5 6 7");
    let expected = Ok(
        Payload {
            target: "a".to_string(),
            steps: "1 1 100 100 100, 641 2 100 100 100, 1281 3 100 100 100, 1921 4 100 100 100, 2401 5 100 100 100, 2881 6 100 100 100, 3361 7 100 100 100".to_string(),
            length: 2,
            subdivision: 1920
        }
    );
    assert_eq!(expected, out);
}

#[test]
fn alternate() {
    let out = interpret("a $ 1 <2,4> 3 | 5 6");
    let expected = interpret("a $ 1 2 3 | 1 4 3 | 5 6");
    assert_eq!(expected, out);

    let out = interpret("a $ 1 <2,4> <3,6>");
    let expected = interpret("a $ 1 2 3 | 1 4 6");
    assert_eq!(expected, out);

    let out = interpret("a $ <1,2> <3,3,4,4>");
    let expected = interpret("a $ 1 3 | 2 3 | 1 4 | 2 4");
    assert_eq!(expected, out);

    let out = interpret("a $ <1,1,2,2> <3,4>");
    let expected = interpret("a $ 1 3 | 1 4 | 2 3 | 2 4");
    assert_eq!(expected, out);

    let out = interpret("a $ <1,2,3> <4,5>");
    let expected = interpret("a $ 1 4 | 2 5 | 3 4 | 1 5 | 2 4 | 3 5");
    assert_eq!(expected, out);
}

#[test]
fn polymetric() {
    let out = interpret("a $ {1 2 3 4}%5");
    let expected = interpret("a $ 1 2 3 4 1 | 2 3 4 1 2 | 3 4 1 2 3 | 4 1 2 3 4");
    assert_eq!(expected, out);

    let out = interpret("a $ {1 2 3 4 5 6}%5");
    let expected = interpret("a $ 1 2 3 4 5 | 6 1 2 3 4 | 5 6 1 2 3 | 4 5 6 1 2 | 3 4 5 6 1 | 2 3 4 5 6");
    assert_eq!(expected, out);
}

#[test]
fn polymetric_even() {
    let out = interpret("a $ {1 2}%4");
    let expected = interpret("a $ 1 2 1 2");
    assert_eq!(expected, out);

    let out = interpret("a $ {1 2 3 4}%4");
    let expected = interpret("a $ 1 2 3 4");
    assert_eq!(expected, out);

    let out = interpret("a $ {1 2 3 4 5 6 7 8}%4");
    let expected = interpret("a $ 1 2 3 4 | 5 6 7 8");
    assert_eq!(expected, out);
}

#[test]
fn polymetric_alternate() {
    let out = interpret("a $ {1 2 <3,5,6> 4}%5");
    let expected = interpret("a $ 1 2 3 4 1 | 2 5 4 1 2 | 6 4 1 2 3 | 4 1 2 5 4 | 1 2 6 4 1 | 2 3 4 1 2 | 5 4 1 2 6 | 4 1 2 3 4 | 1 2 5 4 1 | 2 6 4 1 2 | 3 4 1 2 5 | 4 1 2 6 4");
    assert_eq!(expected, out);

    let out = interpret("a $ {1 2 <3,[5 6]> 4}%5");
    let expected = interpret("a $ 1 2 3 4 1 | 2 [5 6] 4 1 2 | 3 4 1 2 [5 6] | 4 1 2 3 4 | 1 2 [5 6] 4 1 | 2 3 4 1 2 | [5 6] 4 1 2 3 | 4 1 2 [5 6] 4");
    assert_eq!(expected, out);

    let out = interpret("a $ {1 2 <3,5> 4}%4");
    let expected = interpret("a $ 1 2 3 4 | 1 2 5 4");
    assert_eq!(expected, out);

    let out = interpret("a $ {1 2 <3,4> <5,6,7>}%4");
    let expected = interpret("a $ 1 2 3 5 | 1 2 4 6 | 1 2 3 7 | 1 2 4 5 | 1 2 3 6 | 1 2 4 7");
    assert_eq!(expected, out);

    let out = interpret("a $ {1 2 9 <3,4> <5,6,7>}%3");
    let expected = interpret("a $ 1 2 9 | 3 5 1 | 2 9 4 | 6 1 2 | 9 3 7 | 1 2 9 | 4 5 1 | 2 9 3 | 6 1 2 | 9 4 7");
    assert_eq!(expected, out);
}

#[test]
fn euclidean() {
    let out = interpret("a $ b(3,8,0) h");
    let expected = interpret("a $ [b ~ ~ b ~ ~ b ~] h");
    assert_eq!(expected, out);

    let out = interpret("a $ b(3,8,1) h");
    let expected = interpret("a $ [~ b ~ ~ b ~ ~ b] h");
    assert_eq!(expected, out);

    let out = interpret("a $ [s [h b(3,8,1)]] h");
    let expected = interpret("a $ [s [h [~ b ~ ~ b ~ ~ b]]] h");
    assert_eq!(expected, out);

    let out = interpret("a $ b(<3,4>,8,0) h");
    let expected = interpret("a $ [b ~ ~ b ~ ~ b ~] h | [b ~ b ~ b ~ b ~] h");
    assert_eq!(expected, out);

    let out = interpret("a $ [s [h b(<3,4>,8,0)]] h");
    let expected = interpret("a $ [s [h [b ~ ~ b ~ ~ b ~]]] h | [s [h [b ~ b ~ b ~ b ~]]] h");
    assert_eq!(expected, out);

    let out = interpret("a $ b(<1,2,4>,<4,8>,<0,1>) h");
    let expected = interpret(
        "a $ [b ~ ~ ~] h | [~ b ~ ~ ~ b ~ ~] h | [b b b b] h | [~ b ~ ~ ~ ~ ~ ~] h | [b ~ b ~] h | [~ b ~ b ~ b ~ b] h",
    );
    assert_eq!(expected, out);
}

#[test]
fn euclidean_paper() {
    let out = interpret("a $ b(1,2)");
    let expected = interpret("a $ b ~");
    assert_eq!(expected, out);
    let out = interpret("a $ b(1,3)");
    let expected = interpret("a $ b ~ ~");
    assert_eq!(expected, out);
    let out = interpret("a $ b(1,4)");
    let expected = interpret("a $ b ~ ~ ~");
    assert_eq!(expected, out);
    let out = interpret("a $ b(4,12)");
    let expected = interpret("a $ b ~ ~ b ~ ~ b ~ ~ b ~ ~ ");
    assert_eq!(expected, out);
    let out = interpret("a $ b(2,3)");
    let expected = interpret("a $ b ~ b");
    assert_eq!(expected, out);
    let out = interpret("a $ b(2,5)");
    let expected = interpret("a $ b ~ b ~ ~");
    assert_eq!(expected, out);
    let out = interpret("a $ b(3,4)");
    let expected = interpret("a $ b ~ b b");
    assert_eq!(expected, out);
    let out = interpret("a $ b(3,5)");
    let expected = interpret("a $ b ~ b ~ b");
    assert_eq!(expected, out);
    let out = interpret("a $ b(3,7)");
    let expected = interpret("a $ b ~ b ~ b ~ ~");
    assert_eq!(expected, out);
    let out = interpret("a $ b(4,7)");
    let expected = interpret("a $ b ~ b ~ b ~ b");
    assert_eq!(expected, out);
    let out = interpret("a $ b(4,9)");
    let expected = interpret("a $ b ~ b ~ b ~ b ~ ~");
    assert_eq!(expected, out);
    let out = interpret("a $ b(4,11)");
    let expected = interpret("a $ b ~ ~ b ~ ~ b ~ ~ b ~");
    assert_eq!(expected, out);
    let out = interpret("a $ b(5,6)");
    let expected = interpret("a $ b ~ b b b b");
    assert_eq!(expected, out);
    // this is broken
    // let out = interpret("a $ b(5,7)");
    // let expected = interpret("a $ b ~ b b ~ b b");
    // assert_eq!(expected, out);
}

#[test]
fn repeated() {
    let out = interpret("a $ b*2");
    let expected = interpret("a $ b b");
    assert_eq!(expected, out);
    let out = interpret("a $ b*2 s");
    let expected = interpret("a $ [b b] s");
    assert_eq!(expected, out);
    let out = interpret("a $ [b [s h [h s] b]]*2 s");
    let expected = interpret("a $ [[b [s h [h s] b]] [b [s h [h s] b]]] s");
    assert_eq!(expected, out);
    let out = interpret("a $ [b [s h [h s]*3 b]]*2 s");
    let expected = interpret("a $ [[b [s h [[h s] [h s] [h s]] b]] [b [s h [[h s] [h s] [h s]] b]]] s");
    assert_eq!(expected, out);
    let out = interpret("a $ <b,h>*2 s");
    let expected = interpret("a $ [<b,h> <b,h>] s");
    assert_eq!(expected, out);
    let expected2 = interpret("a $ [b b] s | [h h] s");
    assert_eq!(expected2, out);
    let out = interpret("a $ [b(3,4)]*2");
    let expected = interpret("a $ [b(3,4) b(3,4)]");
    assert_eq!(expected, out);
    let expected2 = interpret("a $ b(3,4)*2");
    assert_eq!(expected2, out);
}

#[test]
fn replicated() {
    let out = interpret("a $ b!2");
    let expected = interpret("a $ b b");
    assert_eq!(expected, out);
    let out = interpret("a $ b!2 s");
    let expected = interpret("a $ b b s");
    assert_eq!(expected, out);
    let out = interpret("a $ [b [s h [h s] b]]!2 s");
    let expected = interpret("a $ [b [s h [h s] b]] [b [s h [h s] b]] s");
    assert_eq!(expected, out);
    let out = interpret("a $ [b [s h [h s]!3 b]]!2 s");
    let expected = interpret("a $ [b [s h [h s] [h s] [h s] b]] [b [s h [h s] [h s] [h s] b]] s");
    assert_eq!(expected, out);
    let out = interpret("a $ <b,h>!2 s");
    let expected = interpret("a $ <b,h> <b,h> s");
    assert_eq!(expected, out);
    let expected2 = interpret("a $ b b s | h h s");
    assert_eq!(expected2, out);
    let out = interpret("a $ [b(3,4)]!2");
    let expected = interpret("a $ b(3,4) b(3,4)");
    assert_eq!(expected, out);
    let expected2 = interpret("a $ b(3,4)!2");
    assert_eq!(expected2, out);
}

#[test]
fn probability() {
    let out = interpret("a $ 1 2?25 3 4");
    let expected = Ok(Payload {
        target: "a".to_string(),
        steps: "1 1 100 100 100, 481 2 100 100 25, 961 3 100 100 100, 1441 4 100 100 100".to_string(),
        length: 1,
        subdivision: 1920,
    });
    assert_eq!(expected, out);

    let out = interpret("a $ 1 [3?22 4] 3 | 5 6");
    let expected = Ok(
        Payload {
            target: "a".to_string(),
            steps: "1 1 100 100 100, 641 3 100 100 22, 961 4 100 100 100, 1281 3 100 100 100, 1921 5 100 100 100, 2881 6 100 100 100".to_string(),
            length: 2,
            subdivision: 1920
        }
    );
    assert_eq!(expected, out);

    let out = interpret("a $ 1 2?40!2 3");
    let expected = interpret("a $ 1 2?40 2?40 3");
    assert_eq!(expected, out);

    let out = interpret("a $ 1 2?40*2 3");
    let expected = interpret("a $ 1 [2?40 2?40] 3");
    assert_eq!(expected, out);

    let out = interpret("a $ 1 [2?40*2 5] 3");
    let expected = interpret("a $ 1 [[2?40 2?40] 5] 3");
    assert_eq!(expected, out);

    /* Not yet implemented:

    let out = interpret("a $ 1 [2 4]?22 3 | 5 6");
    let expected = interpret("a $ 1 [2?22 4?22] 3 | 5 6");
    assert_eq!(expected, out);

    let out = interpret("a $ 1 <2,4>?22 3 | 5 6");
    let expected = interpret("a $ 1 2?22 3 | 1 4?22 3 | 5 6");
    assert_eq!(expected, out);

    */
}
