#![allow(unused)]
pub fn main() {
    let a = ["a", "b", "c"];

    // Bad usage, without "copied()", on purpose
    let v: Vec<_> = a.iter().collect();

    let v_copied: Vec<_> = a.iter().copied().collect();

    // copied is the same as .map(|&x| x)
    let v_map: Vec<_> = a.iter().map(|&x| x).collect();

    assert_eq!(v, vec![&"a", &"b", &"c"]);
    assert_eq!(v_copied, vec!["a", "b", "c"]);
    assert_eq!(v_map, vec!["a", "b", "c"]);
}
