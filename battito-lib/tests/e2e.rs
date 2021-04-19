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
}

#[test]
fn polymetric_even() {
    let out = interpret("a > {1 2}%4");
    let expected = Ok(
        Payload {
            target: "a".to_string(),
            steps: "1 1 100 100, 481 2 100 100, 961 1 100 100, 1441 2 100 100, 1921 1 100 100, 2401 2 100 100, 2881 1 100 100, 3361 2 100 100".to_string(),
            length: 2,
            subdivision: 1920
        }
    );
    assert_eq!(expected, out);

    let out = interpret("a > {1 2 3 4}%4");
    let expected = Ok(
        Payload {
            target: "a".to_string(),
            steps: "1 1 100 100, 481 2 100 100, 961 3 100 100, 1441 4 100 100".to_string(),
            length: 1,
            subdivision: 1920
        }
    );
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