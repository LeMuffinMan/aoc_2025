use mockall::predicate::*;
use mockall::*;

#[allow(dead_code)]
struct Foo;
#[allow(dead_code)]
struct Bar;

#[allow(dead_code)]
#[automock]
trait TraitExample {
    fn do_something(&self) -> String;
}

impl TraitExample for Foo {
    fn do_something(&self) -> String {
        "Foo!".to_string()
    }
}

impl TraitExample for Bar {
    fn do_something(&self) -> String {
        "Bar!".to_string()
    }
}

#[allow(dead_code)]
fn append_string(base: impl TraitExample, string_to_append: &str) -> String {
    base.do_something() + string_to_append
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn this_will_test_append_string_but_relies_on_foo_and_bar_to_work_as_expected() {
        assert_eq!(append_string(Foo, " is great"), "Foo! is great");
        assert_eq!(append_string(Bar, " is great"), "Bar! is great");
    }

    #[test]
    fn this_test_will_test_append_string_independently_of_the_rest_thanks_to_mock() {
        let mut mock = MockTraitExample::new();
        mock.expect_do_something()
            .return_const("Mocked!".to_string());

        assert_eq!(append_string(mock, " is great"), "Mocked! is great");
    }
}
