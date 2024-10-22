#[allow(unused_macros)]
#[macro_export]
macro_rules! avec {
    () => {
        Vec::new()
    };
    ($elem:expr) => {{
        let mut vs = Vec::new();
        vs.push($elem);
        vs
    }};
    ($($elem:expr),+ $(,)?) => {{
        let mut vs = Vec::new();
        $(vs.push($elem);)+
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
fn multi() {
    let x: Vec<u32> = avec![17, 21, 35];
    assert!(!x.is_empty());
    assert_eq!(x.len(), 3);
    assert_eq!(x[0], 17);
    assert_eq!(x[1], 21);
    assert_eq!(x[2], 35);
}