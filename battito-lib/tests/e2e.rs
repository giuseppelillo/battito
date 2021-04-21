use battito_lib::interpreter::interpret;
use battito_lib::max::Payload;

#[test]
fn one_measure() {
    let out = interpret("a > 1 2 3");
    let expected = Ok(Payload {
        target: "a".to_string(),
        steps: "1 1 100 100, 641 2 100 100, 1281 3 100 100".to_string(),
        length: 1,
        subdivision: 1920,
    });
    assert_eq!(expected, out);
}

#[test]
fn two_measures() {
    let out = interpret("a > 1 2 3 | 4 5 6 7");
    let expected = Ok(
        Payload {
            target: "a".to_string(),
            steps: "1 1 100 100, 641 2 100 100, 1281 3 100 100, 1921 4 100 100, 2401 5 100 100, 2881 6 100 100, 3361 7 100 100".to_string(),
            length: 2,
            subdivision: 1920
        }
    );
    assert_eq!(expected, out);
}

#[test]
fn alternate() {
    let out = interpret("a > 1 <2,4> 3 | 5 6");
    let expected = Ok(
        Payload {
            target: "a".to_string(),
            steps: "1 1 100 100, 641 2 100 100, 1281 3 100 100, 1921 1 100 100, 2561 4 100 100, 3201 3 100 100, 3841 5 100 100, 4801 6 100 100".to_string(),
            length: 3,
            subdivision: 1920
        }
    );
    assert_eq!(expected, out);

    let out = interpret("a > 1 <2,4> <3,6>");
    let expected = Ok(
        Payload {
            target: "a".to_string(),
            steps: "1 1 100 100, 641 2 100 100, 1281 3 100 100, 1921 1 100 100, 2561 4 100 100, 3201 6 100 100".to_string(),
            length: 2,
            subdivision: 1920
        }
    );
    assert_eq!(expected, out);

    // 1 3 | 2 3 | 1 4 | 2 4
    let out = interpret("a > <1,2> <3,3,4,4>");
    let expected = Ok(
        Payload {
            target: "a".to_string(),
            steps: "1 1 100 100, 961 3 100 100, 1921 2 100 100, 2881 3 100 100, 3841 1 100 100, 4801 4 100 100, 5761 2 100 100, 6721 4 100 100".to_string(),
            length: 4,
            subdivision: 1920
        }
    );
    assert_eq!(expected, out);

    // 1 3 | 1 4 | 2 3 | 2 4
    let out = interpret("a > <1,1,2,2> <3,4>");
    let expected = Ok(
        Payload {
            target: "a".to_string(),
            steps: "1 1 100 100, 961 3 100 100, 1921 1 100 100, 2881 4 100 100, 3841 2 100 100, 4801 3 100 100, 5761 2 100 100, 6721 4 100 100".to_string(),
            length: 4,
            subdivision: 1920
        }
    );
    assert_eq!(expected, out);

    // 1 3 | 1 4 | 2 3 | 2 4
    let out = interpret("a > <1,2,3> <4,5>");
    let expected = Ok(
        Payload {
            target: "a".to_string(),
            steps: "1 1 100 100, 961 4 100 100, 1921 2 100 100, 2881 5 100 100, 3841 3 100 100, 4801 4 100 100, 5761 1 100 100, 6721 5 100 100, 7681 2 100 100, 8641 4 100 100, 9601 3 100 100, 10561 5 100 100".to_string(),
            length: 6,
            subdivision: 1920
        }
    );
    assert_eq!(expected, out);
}

#[test]
fn polymetric() {
    let out = interpret("a > {1 2 3 4}%5");
    let expected = Ok(
        Payload {
            target: "a".to_string(),
            steps: "1 1 100 100, 385 2 100 100, 769 3 100 100, 1153 4 100 100, 1537 1 100 100, 1921 2 100 100, 2305 3 100 100, 2689 4 100 100, 3073 1 100 100, 3457 2 100 100, 3841 3 100 100, 4225 4 100 100, 4609 1 100 100, 4993 2 100 100, 5377 3 100 100, 5761 4 100 100, 6145 1 100 100, 6529 2 100 100, 6913 3 100 100, 7297 4 100 100".to_string(),
            length: 4,
            subdivision: 1920
        }
    );
    assert_eq!(expected, out);

    let out = interpret("a > {1 2 3 4 5 6}%5");
    let expected = Ok(
        Payload {
            target: "a".to_string(),
            steps: "1 1 100 100, 385 2 100 100, 769 3 100 100, 1153 4 100 100, 1537 5 100 100, 1921 6 100 100, 2305 1 100 100, 2689 2 100 100, 3073 3 100 100, 3457 4 100 100, 3841 5 100 100, 4225 6 100 100, 4609 1 100 100, 4993 2 100 100, 5377 3 100 100, 5761 4 100 100, 6145 5 100 100, 6529 6 100 100, 6913 1 100 100, 7297 2 100 100, 7681 3 100 100, 8065 4 100 100, 8449 5 100 100, 8833 6 100 100, 9217 1 100 100, 9601 2 100 100, 9985 3 100 100, 10369 4 100 100, 10753 5 100 100, 11137 6 100 100".to_string(),
            length: 6,
            subdivision: 1920
        }
    );
    assert_eq!(expected, out);
}

#[test]
fn polymetric_even() {
    let out = interpret("a > {1 2}%4");
    let expected = Ok(Payload {
        target: "a".to_string(),
        steps: "1 1 100 100, 481 2 100 100, 961 1 100 100, 1441 2 100 100".to_string(),
        length: 1,
        subdivision: 1920,
    });
    assert_eq!(expected, out);

    let out = interpret("a > {1 2 3 4}%4");
    let expected = Ok(Payload {
        target: "a".to_string(),
        steps: "1 1 100 100, 481 2 100 100, 961 3 100 100, 1441 4 100 100".to_string(),
        length: 1,
        subdivision: 1920,
    });
    assert_eq!(expected, out);

    let out = interpret("a > {1 2 3 4 5 6 7 8}%4");
    let expected = Ok(
        Payload {
            target: "a".to_string(),
            steps: "1 1 100 100, 481 2 100 100, 961 3 100 100, 1441 4 100 100, 1921 5 100 100, 2401 6 100 100, 2881 7 100 100, 3361 8 100 100".to_string(),
            length: 2,
            subdivision: 1920
        }
    );
    assert_eq!(expected, out);
}

#[test]
fn polymetric_alternate() {
    // 1 2 3 4 1 | 2 5 4 1 2 | 6 4 1 2 3 | 4 1 2 5 4 | 1 2 6 4 1 | 2 3 4 1 2 | 5 4 1 2 6 | 4 1 2 3 4 | 1 2 5 4 1 | 2 3 6 1 2 | 3 4 1 2 5 | 4 1 2 6 4
    let out = interpret("a > {1 2 <3,5,6> 4}%5");
    let expected = Ok(
        Payload {
            target: "a".to_string(),
            steps: "1 1 100 100, 385 2 100 100, 769 3 100 100, 1153 4 100 100, 1537 1 100 100, 1921 2 100 100, 2305 5 100 100, 2689 4 100 100, 3073 1 100 100, 3457 2 100 100, 3841 6 100 100, 4225 4 100 100, 4609 1 100 100, 4993 2 100 100, 5377 3 100 100, 5761 4 100 100, 6145 1 100 100, 6529 2 100 100, 6913 5 100 100, 7297 4 100 100, 7681 1 100 100, 8065 2 100 100, 8449 6 100 100, 8833 4 100 100, 9217 1 100 100, 9601 2 100 100, 9985 3 100 100, 10369 4 100 100, 10753 1 100 100, 11137 2 100 100, 11521 5 100 100, 11905 4 100 100, 12289 1 100 100, 12673 2 100 100, 13057 6 100 100, 13441 4 100 100, 13825 1 100 100, 14209 2 100 100, 14593 3 100 100, 14977 4 100 100, 15361 1 100 100, 15745 2 100 100, 16129 5 100 100, 16513 4 100 100, 16897 1 100 100, 17281 2 100 100, 17665 6 100 100, 18049 4 100 100, 18433 1 100 100, 18817 2 100 100, 19201 3 100 100, 19585 4 100 100, 19969 1 100 100, 20353 2 100 100, 20737 5 100 100, 21121 4 100 100, 21505 1 100 100, 21889 2 100 100, 22273 6 100 100, 22657 4 100 100".to_string(),
            length: 12,
            subdivision: 1920
        }
    );
    assert_eq!(expected, out);

    // 1 2 3 4 1 | 2 [5 6] 4 1 2 | 3 4 1 2 [5 6] | 4 1 2 3 4 | 1 2 [5 6] 4 1 | 2 3 4 1 2 | [5 6] 4 1 2 3 | 4 1 2 [5 6] 4
    let out = interpret("a > {1 2 <3,[5 6]> 4}%5");
    let expected = Ok(
        Payload {
            target: "a".to_string(),
            steps: "1 1 100 100, 385 2 100 100, 769 3 100 100, 1153 4 100 100, 1537 1 100 100, 1921 2 100 100, 2305 5 100 100, 2497 6 100 100, 2689 4 100 100, 3073 1 100 100, 3457 2 100 100, 3841 3 100 100, 4225 4 100 100, 4609 1 100 100, 4993 2 100 100, 5377 5 100 100, 5569 6 100 100, 5761 4 100 100, 6145 1 100 100, 6529 2 100 100, 6913 3 100 100, 7297 4 100 100, 7681 1 100 100, 8065 2 100 100, 8449 5 100 100, 8641 6 100 100, 8833 4 100 100, 9217 1 100 100, 9601 2 100 100, 9985 3 100 100, 10369 4 100 100, 10753 1 100 100, 11137 2 100 100, 11521 5 100 100, 11713 6 100 100, 11905 4 100 100, 12289 1 100 100, 12673 2 100 100, 13057 3 100 100, 13441 4 100 100, 13825 1 100 100, 14209 2 100 100, 14593 5 100 100, 14785 6 100 100, 14977 4 100 100".to_string(),
            length: 8,
            subdivision: 1920
        }
    );
    assert_eq!(expected, out);

    // 1 2 3 4 | 1 2 3 5
    let out = interpret("a > {1 2 <3,5> 4}%4");
    let expected = Ok(Payload {
        target: "a".to_string(),
        steps: "1 1 100 100, 481 2 100 100, 961 3 100 100, 1441 4 100 100, 1921 1 100 100, 2401 2 100 100, 2881 5 100 100, 3361 4 100 100".to_string(),
        length: 2,
        subdivision: 1920,
    });
    assert_eq!(expected, out);

    // 1 2 3 5 | 1 2 4 6 | 1 2 3 7 | 1 2 4 5 | 1 2 3 6 | 1 2 4 7
    let out = interpret("a > {1 2 <3,4> <5,6,7>}%4");
    let expected = Ok(Payload {
        target: "a".to_string(),
        steps: "1 1 100 100, 481 2 100 100, 961 3 100 100, 1441 5 100 100, \
                1921 1 100 100, 2401 2 100 100, 2881 4 100 100, 3361 6 100 100, \
                3841 1 100 100, 4321 2 100 100, 4801 3 100 100, 5281 7 100 100, \
                5761 1 100 100, 6241 2 100 100, 6721 4 100 100, 7201 5 100 100, \
                7681 1 100 100, 8161 2 100 100, 8641 3 100 100, 9121 6 100 100, \
                9601 1 100 100, 10081 2 100 100, 10561 4 100 100, 11041 7 100 100"
            .to_string(),
        length: 6,
        subdivision: 1920,
    });
    assert_eq!(expected, out);

    // 1 2 9 | 3 5 1 | 2 9 4 | 6 1 2 | 9 3 7 | 1 2 9 | 4 5 1 | 2 9 3 | 6 1 2 | 9 4 7
    let out = interpret("a > {1 2 9 <3,4> <5,6,7>}%3");
    let expected = Ok(Payload {
        target: "a".to_string(),
        steps: "1 1 100 100, 641 2 100 100, 1281 9 100 100, \
                1921 3 100 100, 2561 5 100 100, 3201 1 100 100, \
                3841 2 100 100, 4481 9 100 100, 5121 4 100 100, \
                5761 6 100 100, 6401 1 100 100, 7041 2 100 100, \
                7681 9 100 100, 8321 3 100 100, 8961 7 100 100, \
                9601 1 100 100, 10241 2 100 100, 10881 9 100 100, \
                11521 4 100 100, 12161 5 100 100, 12801 1 100 100, \
                13441 2 100 100, 14081 9 100 100, 14721 3 100 100, \
                15361 6 100 100, 16001 1 100 100, 16641 2 100 100, \
                17281 9 100 100, 17921 4 100 100, 18561 7 100 100"
            .to_string(),
        length: 10,
        subdivision: 1920,
    });
    assert_eq!(expected, out);
}

/*
#[test]
fn euclidian() {
    // [b ~ ~ b ~ ~ b ~] h
    let out = interpret("a > b(3,8,0) h");
    let expected = Ok(
        Payload {
            target: "a".to_string(),
            steps: "1 1 100 100, 385 2 100 100, 769 3 100 100, 1153 4 100 100, 1537 1 100 100".to_string(),
            length: 1,
            subdivision: 1920
        }
    );
    assert_eq!(expected, out);

    // [~ b ~ ~ b ~ ~ b] h
    let out = interpret("a > b(3,8,1) h");
    let expected = Ok(
        Payload {
            target: "a".to_string(),
            steps: "1 1 100 100, 385 2 100 100, 769 3 100 100, 1153 4 100 100, 1537 1 100 100".to_string(),
            length: 1,
            subdivision: 1920
        }
    );
    assert_eq!(expected, out);

    // [b ~ b ~] | [[h s] ~ [h s] ~]
    let out = interpret("a > <b,[h s]>(2,4) h");
    let expected = Ok(
        Payload {
            target: "a".to_string(),
            steps: "1 1 100 100, 385 2 100 100, 769 3 100 100, 1153 4 100 100, 1537 1 100 100".to_string(),
            length: 2,
            subdivision: 1920
        }
    );
    assert_eq!(expected, out);


    // [b ~ ~ b ~ ~ b ~] h | [b ~ b ~ b ~ b ~] h
    let out = interpret("a > b(<3,4>,8,0) h");
    let expected = Ok(
        Payload {
            target: "a".to_string(),
            steps: "1 1 100 100, 385 2 100 100, 769 3 100 100, 1153 4 100 100, 1537 1 100 100".to_string(),
            length: 2,
            subdivision: 1920
        }
    );
    assert_eq!(expected, out);
}
*/
