use serde_json::Value;
use tasky::models::assignment::AnswerType;

#[test]
fn test_match_string_str_contains() {
    let value = Value::String("Hello World".to_string());
    assert_eq!(match_question_type(AnswerType::String, value), true);
    assert_eq!(match_question_type(AnswerType::StrContains, value), true);
    assert_eq!(match_question_type(AnswerType::Boolean, value), false);
    assert_eq!(match_question_type(AnswerType::Number, value), false);
}

#[test]
fn test_match_bool() {
    let value = Value::Bool(false);
    assert_eq!(match_question_type(AnswerType::String, value), false);
    assert_eq!(match_question_type(AnswerType::StrContains, value), false);
    assert_eq!(match_question_type(AnswerType::Boolean, value), true);
    assert_eq!(match_question_type(AnswerType::Number, value), false);
}

#[test]
fn test_match_number() {
    let value = Value::Number(1);
    assert_eq!(match_question_type(AnswerType::String, value), false);
    assert_eq!(match_question_type(AnswerType::StrContains, value), false);
    assert_eq!(match_question_type(AnswerType::Boolean, value), false);
    assert_eq!(match_question_type(AnswerType::Number, value), true);
}
