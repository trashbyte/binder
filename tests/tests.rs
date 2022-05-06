#![allow(dead_code)]

use binder::Property;

#[test]
fn initialization() {
    struct AStruct { x: f32, y: f32 }
    let _ = Property::new(1f32);
    let _ = Property::new(false);
    let _ = Property::new(String::from("test"));
    let _ = Property::new(vec![AStruct { x: 2.0, y: 3.0 }]);
}

#[test]
fn immutable_access() {
    #[derive(Debug, PartialEq)]
    struct AStruct { x: f32, y: f32 }
    let p1 = Property::new(1f32);
    let p2 = Property::new(false);
    let p3 = Property::new(String::from("test"));
    let p4 = Property::new(vec![AStruct { x: 2.0, y: *p1.bind() }]);

    fn immutable_reference(s: &str) {
        assert_eq!(s, "test");
    }
    immutable_reference(&p3.bind());

    assert_eq!(*p1.bind(), 1f32);
    assert_eq!(*p2.bind(), false);
    assert_eq!(p3.bind().as_str(), "test");
    assert_eq!(p4.bind()[0], AStruct { x: 2.0, y: 1.0 });
}

#[test]
fn mutable_access() {
    #[derive(Debug, PartialEq)]
    struct AStruct { pub x: f32, pub y: f32 }
    let p1 = Property::new(1f32);
    let p2 = Property::new(false);
    let p3 = Property::new(String::from("test"));
    let p4 = {
        let mut b = p1.bind();
        let p = Property::new(vec![AStruct { x: 2.0, y: *b }]);
        *b = 3f32;
        p
    };
    *p2.bind() = true;

    fn mutate(s: &mut String) {
        s.push_str(" test");
    }
    mutate(&mut p3.bind());

    assert_eq!(*p1.bind(), 3f32);
    assert_eq!(*p2.bind(), true);
    assert_eq!(p3.bind().as_str(), "test test");
    assert_eq!(p4.bind()[0], AStruct { x: 2.0, y: 1.0 });
}

#[test]
fn shared_immutable_access() {
    #[derive(Debug, PartialEq)]
    struct AStruct { pub x: f32, pub y: f32, pub z: f32 }
    let p = Property::new(AStruct { x: 1.0, y: 2.0, z: 3.0 });

    fn compare(a: &f32, b: &f32) -> bool { *a == *b }

    let bind = p.bind();
    let cmp_x = compare(&bind.x, &1.0);
    let cmp_y = compare(&bind.y, &2.0);
    let cmp_z = compare(&bind.z, &3.0);
    let cmp_add = compare(&(bind.x + 2.0), &bind.z);

    assert!(cmp_x);
    assert!(cmp_y);
    assert!(cmp_z);
    assert!(cmp_add);
}

#[test]
fn exclusive_mutable_access() {
    #[derive(Debug, PartialEq)]
    struct AStruct { pub x: i32, pub y: i32, pub z: i32 }
    let p = Property::new(AStruct { x: 1, y: 2, z: 3 });

    fn mutate(s: &mut AStruct) {
        s.x += 1;
    }
    let mut bind = p.bind();
    mutate(&mut *bind);
    mutate(&mut *bind);
    mutate(&mut *bind);
    assert_eq!(bind.x, 4);
}

#[test]
#[should_panic(expected = "PropertyBinding<i32>: Tried to bind a property that was already bound!")]
fn double_bind_panic() {
    let p = Property::new(1i32);
    let _bind = p.bind();
    p.bind();
}

#[test]
fn double_bind_result() {
    let p = Property::new(1i32);
    let res1 = p.try_bind();
    let res2 = p.try_bind();
    assert!(res1.is_ok());
    assert!(res2.is_err());
}

/// ```compile_fail
/// let p = binder::Property::new(1f32);
/// let bind = p.bind();
/// let _ = &mut bind;
/// let _ = &mut bind;
/// ```
struct _Doctest;