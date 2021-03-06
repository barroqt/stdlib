#[macro_export]
macro_rules! avec {
    () => {
        Vec::new()
    };
    ($($element:expr),+ $(,)?) => {{
        let mut vs = Vec::new();
        $(vs.push($element);)*
        vs
    }};
    ($element:expr; $count:expr) => {{
        let count = $count;
        let mut vs = Vec::with_capacity(count);
        let x = $element;
        vs.extend(std::iter::repeat($element).take(count));
        vs
    }};
}

#[test]
fn empty_vec() {
    let x: Vec<u32> = avec![];
    assert!(x.is_empty());
}

#[test]
fn single() {
    let x: Vec<u32> = avec![42];
    assert!(!x.is_empty());
    assert_eq!(x.len(), 1);
    assert_eq!(x[0], 42);
}

#[test]
fn trailing() {
    let _: Vec<&'static str> = avec![
        "dlsacmjpoqjodpqwkopwd",    
        "dlsacmjpoqjodpqwkopwd",    
        "dlsacmjpoqjodpqwkopwd",    
        "dlsacmjpoqjodpqwkopwd",    
    ];
}