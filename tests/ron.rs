#![cfg(feature = "ron")]
use nanoserde::{DeRon, DeRonErrReason, SerRon};

use std::{
    collections::{BTreeMap, BTreeSet, LinkedList},
    sync::atomic::AtomicBool,
};

#[cfg(feature = "std")]
use std::collections::{HashMap, HashSet};

#[test]
fn ron_de() {
    #[derive(DeRon)]
    pub struct Test {
        a: i32,
        b: f32,
        c: Option<String>,
        d: Option<String>,
    }

    let ron = r#"(
        a: 1,
        b: 2.0,
        d: "hello",
    )"#;

    let test: Test = DeRon::deserialize_ron(ron).unwrap();
    assert_eq!(test.a, 1);
    assert_eq!(test.b, 2.);
    assert_eq!(test.c, None);
    assert_eq!(test.d.unwrap(), "hello");
}

#[test]
fn de_container_default() {
    #[derive(DeRon)]
    #[nserde(default)]
    pub struct Test {
        pub a: i32,
        pub b: f32,
        c: Option<String>,
        d: Option<String>,
    }

    let ron = r#"(
        a: 1,
        d: "hello",
    )"#;

    let test: Test = DeRon::deserialize_ron(ron).unwrap();
    assert_eq!(test.a, 1);
    assert_eq!(test.b, 0.);
    assert_eq!(test.d.unwrap(), "hello");
    assert_eq!(test.c, None);
}

#[test]
fn rename() {
    #[derive(DeRon, SerRon, PartialEq)]
    #[nserde(default)]
    pub struct Test {
        #[nserde(rename = "fooField")]
        pub a: i32,
        #[nserde(rename = "barField")]
        pub b: i32,
    }

    let ron = r#"(
        fooField: 1,
        barField: 2,
    )"#;

    let test: Test = DeRon::deserialize_ron(ron).unwrap();
    assert_eq!(test.a, 1);
    assert_eq!(test.b, 2);

    let bytes = SerRon::serialize_ron(&test);
    let test_deserialized = DeRon::deserialize_ron(&bytes).unwrap();
    assert!(test == test_deserialized);
}

#[test]
fn de_field_default() {
    #[derive(DeRon)]
    struct Foo {
        x: i32,
    }
    impl Default for Foo {
        fn default() -> Foo {
            Foo { x: 23 }
        }
    }
    fn foo3_default() -> Foo {
        Foo { x: 15 }
    }

    #[derive(DeRon)]
    pub struct Test {
        a: i32,
        #[nserde(default)]
        foo: Foo,
        foo2: Foo,
        #[nserde(default_with = "foo3_default")]
        foo3: Foo,
        b: f32,
    }

    let ron = r#"(
        a: 1,
        b: 2.,
        foo2: (x: 3)
    )"#;

    let test: Test = DeRon::deserialize_ron(ron).unwrap();
    assert_eq!(test.a, 1);
    assert_eq!(test.b, 2.);
    assert_eq!(test.foo.x, 23);
    assert_eq!(test.foo2.x, 3);
    assert_eq!(test.foo3.x, 15);
}

#[test]
fn de_ser_field_skip() {
    #[derive(DeRon, SerRon, PartialEq, Debug)]
    struct Foo {
        x: i32,
    }
    impl Default for Foo {
        fn default() -> Foo {
            Foo { x: 23 }
        }
    }
    fn foo3_default() -> Foo {
        Foo { x: 15 }
    }

    #[derive(DeRon, SerRon, PartialEq, Debug)]
    pub struct Test {
        a: i32,
        #[nserde(skip)]
        foo: Foo,
        foo2: Foo,
        #[nserde(skip, default_with = "foo3_default")]
        foo3: Foo,
        b: f32,
        #[nserde(skip)]
        c: Option<i32>,
    }

    let ron = r#"(
        a: 1,
        b: 2.,
        foo2: (x: 3)
    )"#;

    let mut test: Test = DeRon::deserialize_ron(ron).unwrap();
    assert_eq!(test.a, 1);
    assert_eq!(test.b, 2.);
    assert_eq!(test.foo.x, 23);
    assert_eq!(test.foo2.x, 3);
    assert_eq!(test.foo3.x, 15);

    test.c = Some(2);
    let serialized = SerRon::serialize_ron(&test);

    let test: Test = DeRon::deserialize_ron(ron).unwrap();
    let deserialized: Test = DeRon::deserialize_ron(&serialized).unwrap();
    assert_eq!(deserialized, test);
}

#[test]
fn doctests() {
    /// This is test
    /// second doc comment
    #[derive(DeRon)]
    pub struct Test {
        /// with documented field
        pub a: i32,
        pub b: f32,
        /// or here
        /// Or here
        c: Option<String>,
        /// more doc comments
        /// and more
        d: Option<String>,
    }

    let ron = r#"(
        a: 1,
        b: 2.0,
        d: "hello"
    )"#;

    let test: Test = DeRon::deserialize_ron(ron).unwrap();
    assert_eq!(test.a, 1);
    assert_eq!(test.b, 2.);
    assert_eq!(test.d.unwrap(), "hello");
    assert_eq!(test.c, None);
}

#[test]
fn empty() {
    #[derive(DeRon)]
    pub struct Empty {}

    let ron = r#"(
    )"#;

    let _: Empty = DeRon::deserialize_ron(ron).unwrap();
}

#[test]
fn one_field() {
    #[derive(DeRon, SerRon, PartialEq)]
    pub struct OneField {
        field: f32,
    }

    let test = OneField { field: 23. };
    let bytes = SerRon::serialize_ron(&test);
    let test_deserialized = DeRon::deserialize_ron(&bytes).unwrap();
    assert!(test == test_deserialized);
}

#[test]
fn one_field_map() {
    #[derive(DeRon, SerRon, PartialEq)]
    pub struct OneField {
        field: BTreeMap<String, f32>,
    }

    let test = OneField {
        field: BTreeMap::new(),
    };
    let bytes = SerRon::serialize_ron(&test);
    let test_deserialized = DeRon::deserialize_ron(&bytes).unwrap();
    assert!(test == test_deserialized);
}

#[test]
fn array() {
    #[derive(DeRon)]
    pub struct Foo {
        x: i32,
    }

    #[derive(DeRon)]
    pub struct Bar {
        foos: Vec<Foo>,
        ints: Vec<i32>,
        floats_a: Option<Vec<f32>>,
        floats_b: Option<Vec<f32>>,
    }

    let ron = r#"(
       foos: [(x: 1), (x: 2)],
       ints: [1, 2, 3, 4],
       floats_b: [4., 3., 2., 1.]
    )"#;

    let bar: Bar = DeRon::deserialize_ron(ron).unwrap();

    assert_eq!(bar.foos.len(), 2);
    assert_eq!(bar.foos[0].x, 1);
    assert_eq!(bar.ints.len(), 4);
    assert_eq!(bar.ints[2], 3);
    assert_eq!(bar.floats_b.unwrap()[2], 2.);
    assert_eq!(bar.floats_a, None);
}

#[test]
fn collections() {
    #[derive(DeRon, SerRon, PartialEq, Debug)]
    pub struct Test {
        pub a: Vec<i32>,
        pub b: LinkedList<f32>,
        pub c: BTreeMap<i32, i32>,
        pub d: BTreeSet<i32>,
    }

    let test: Test = Test {
        a: vec![1, 2, 3],
        b: vec![1.0, 2.0, 3.0, 4.0].into_iter().collect(),
        c: vec![(1, 2), (3, 4)].into_iter().collect(),
        d: vec![1, 2, 3, 4, 5, 6].into_iter().collect(),
    };

    let bytes = SerRon::serialize_ron(&test);

    let test_deserialized = DeRon::deserialize_ron(&bytes).unwrap();

    assert_eq!(test, test_deserialized);
}

#[test]
fn path_type() {
    #[derive(DeRon)]
    struct Foo {
        a: i32,
        b: std::primitive::i32,
        c: Option<std::primitive::i32>,
        d: Option<Vec<std::vec::Vec<std::primitive::i32>>>,
    }

    let ron = r#"(
       a: 0,
       b: 1,
       c: 2,
       d: [[1, 2], [3, 4]]
    )"#;

    let bar: Foo = DeRon::deserialize_ron(ron).unwrap();

    assert_eq!(bar.a, 0);
    assert_eq!(bar.b, 1);
    assert_eq!(bar.c, Some(2));
    assert_eq!(bar.d, Some(vec![vec![1, 2], vec![3, 4]]));
}

#[cfg(feature = "std")]
#[test]
fn hashmaps() {
    #[derive(DeRon)]
    struct Foo {
        map: HashMap<String, i32>,
    }

    let ron = r#"(
       map: {
          "asd": 1,
          "qwe": 2,
       }
    )"#;

    let foo: Foo = DeRon::deserialize_ron(ron).unwrap();

    assert_eq!(foo.map["asd"], 1);
    assert_eq!(foo.map["qwe"], 2);
}

#[test]
fn exponents() {
    #[derive(DeRon)]
    struct Foo {
        a: f64,
        b: f64,
        c: f64,
        d: f64,
        e: f64,
        f: f64,
        g: f64,
        h: f64,
    }

    let ron = r#"(
        a: 1e2,
        b: 1e-2,
        c: 1E2,
        d: 1E-2,
        e: 1.0e2,
        f: 1.0e-2,
        g: 1.0E2,
        h: 1.0E-2
    )"#;

    let foo: Foo = DeRon::deserialize_ron(ron).unwrap();

    assert_eq!(foo.a, 100.);
    assert_eq!(foo.b, 0.01);
    assert_eq!(foo.c, 100.);
    assert_eq!(foo.d, 0.01);
    assert_eq!(foo.e, 100.);
    assert_eq!(foo.f, 0.01);
    assert_eq!(foo.g, 100.);
    assert_eq!(foo.h, 0.01);
}

#[test]
fn ronerror() {
    #[derive(DeRon)]
    #[allow(dead_code)]
    struct Foo {
        i: i32,
    }

    let ron = r#"(
       i: "string"
    )"#;

    let res: Result<Foo, _> = DeRon::deserialize_ron(ron);
    match res {
        Ok(_) => assert!(false),
        Err(e) => {
            let _dyn_e: Box<dyn std::error::Error> = std::convert::From::from(e);
        }
    }
}

#[test]
fn de_enum() {
    #[derive(DeRon, PartialEq, Debug)]
    pub enum Foo {
        A,
        B(i32, String),
        C { a: i32, b: String },
    }

    #[derive(DeRon, PartialEq, Debug)]
    pub struct Bar {
        foo1: Foo,
        foo2: Foo,
        foo3: Foo,
    }

    let ron = r#"
       (
          foo1: A,
          foo2: B(1, "asd"),
          foo3: C(a: 2, b: "qwe"),
       )
    "#;

    let test: Bar = DeRon::deserialize_ron(ron).unwrap();

    assert_eq!(test.foo1, Foo::A);
    assert_eq!(test.foo2, Foo::B(1, "asd".to_string()));
    assert_eq!(
        test.foo3,
        Foo::C {
            a: 2,
            b: "qwe".to_string()
        }
    );
}

#[test]
fn de_ser_enum() {
    #[derive(SerRon, DeRon, PartialEq, Debug)]
    pub enum Fud {
        A = 0,
        B = 1,
        C = 2,
    }

    #[derive(SerRon, DeRon, PartialEq, Debug)]
    pub struct Bar {
        foo1: Fud,
        foo2: Fud,
        foo3: Fud,
    }

    let ron = "(foo1:A,foo2:B,foo3:C,)";

    let data = Bar {
        foo1: Fud::A,
        foo2: Fud::B,
        foo3: Fud::C,
    };
    let serialized = SerRon::serialize_ron(&data);

    assert_eq!(serialized, ron);

    let deserialized: Bar = DeRon::deserialize_ron(&serialized).unwrap();
    assert_eq!(deserialized, data);
}

#[test]
fn ser_enum_complex() {
    #[derive(SerRon, DeRon, PartialEq, Debug)]
    pub enum Foo {
        A,
        B(i32, String),
        C { a: i32, b: String },
    }

    #[derive(SerRon, DeRon, PartialEq, Debug)]
    pub struct Bar {
        foo1: Foo,
        foo2: Foo,
        foo3: Foo,
    }

    let ron = "(foo1:A,foo2:B(1,\"asd\"),foo3:C(a:2,b:\"qwe\",),)";

    let data = Bar {
        foo1: Foo::A,
        foo2: Foo::B(1, String::from("asd")),
        foo3: Foo::C {
            a: 2,
            b: String::from("qwe"),
        },
    };
    let serialized = SerRon::serialize_ron(&data);
    assert_eq!(serialized, ron);

    let deserialized: Bar = DeRon::deserialize_ron(&serialized).unwrap();
    assert_eq!(deserialized, data);
}

#[test]
fn test_various_escapes() {
    let ron = r#""\n\t\u0020\f\b\\\"\/\ud83d\uDE0B\r""#;
    let unescaped: String = DeRon::deserialize_ron(ron).unwrap();
    assert_eq!(unescaped, "\n\t\u{20}\x0c\x08\\\"/😋\r");
}

#[test]
fn test_various_floats() {
    #[derive(Debug, SerRon, DeRon, PartialEq)]
    struct FloatWrapper {
        f32: f32,
        f64: f64,
    }

    impl From<&(f32, f64)> for FloatWrapper {
        fn from(value: &(f32, f64)) -> Self {
            Self {
                f32: value.0,
                f64: value.1,
            }
        }
    }

    let cases: &[(f32, f64)] = &[
        (0.0, 0.0),
        (f32::MAX, f64::MAX),
        (f32::MIN, f64::MIN),
        (f32::MIN_POSITIVE, f64::MIN_POSITIVE),
    ];

    for case in cases {
        assert_eq!(
            FloatWrapper::from(case),
            <FloatWrapper as DeRon>::deserialize_ron(&dbg!(
                FloatWrapper::from(case).serialize_ron()
            ))
            .unwrap()
        )
    }
}

// there are only 1024*1024 surrogate pairs, so we can do an exhautive test.
#[test]
#[cfg_attr(miri, ignore)]
fn test_surrogate_pairs_exhaustively() {
    for lead in 0xd800..0xdc00 {
        for trail in 0xdc00..0xe000 {
            // find the scalar value represented by the [lead, trail] pair.
            let combined: Vec<char> = core::char::decode_utf16([lead, trail].iter().copied())
                .collect::<Result<_, _>>()
                .unwrap_or_else(|e| {
                    panic!(
                        "[{:#04x}, {:#04x}] not valid surrogate pair? {:?}",
                        lead, trail, e,
                    );
                });
            assert_eq!(combined.len(), 1);
            let expected_string = format!("{}", combined[0]);

            let ron = format!(r#""\u{:04x}\u{:04x}""#, lead, trail);
            let result: String = DeRon::deserialize_ron(&ron).unwrap_or_else(|e| {
                panic!("should be able to parse {}: {:?}", &ron, e);
            });
            assert_eq!(result, expected_string, "failed on input {}", ron);
            assert_eq!(result.chars().count(), 1);
        }
    }
}

#[test]
fn tuple_struct() {
    #[derive(DeRon, SerRon, PartialEq)]
    pub struct Test(i32, pub i32, pub(crate) String, f32);

    #[derive(DeRon, SerRon, PartialEq)]
    pub struct Vec2(pub(crate) f32, pub(crate) f32);

    let test = Test(0, 1, "asd".to_string(), 2.);
    let bytes = SerRon::serialize_ron(&test);

    let test_deserialized = DeRon::deserialize_ron(&bytes).unwrap();

    assert!(test == test_deserialized);
}

#[test]
fn array_leak_test() {
    static TOGGLED_ON_DROP: AtomicBool = AtomicBool::new(false);

    #[derive(Default, Clone, DeRon, SerRon)]
    struct IncrementOnDrop {
        inner: u64,
    }

    impl Drop for IncrementOnDrop {
        fn drop(&mut self) {
            TOGGLED_ON_DROP.store(true, std::sync::atomic::Ordering::SeqCst);
        }
    }

    let items: [_; 2] = core::array::from_fn(|_| IncrementOnDrop::default());
    let serialized = nanoserde::SerRon::serialize_ron(&items);
    let corrupted_serialized = &serialized[..serialized.len() - 1];

    if let Ok(_) = <[IncrementOnDrop; 2] as nanoserde::DeRon>::deserialize_ron(corrupted_serialized)
    {
        panic!("Unexpected success")
    }

    assert!(TOGGLED_ON_DROP.load(std::sync::atomic::Ordering::SeqCst))
}

// https://github.com/not-fl3/nanoserde/issues/89
#[test]
fn test_deser_oversized_value() {
    use nanoserde::DeRon;

    #[derive(DeRon, Clone, PartialEq, Debug)]
    pub struct EnumConstant {
        value: i32,
    }

    let max_ron = format!(r#"(value:{})"#, i32::MAX);
    let wrap_ron = format!(r#"(value:{})"#, i32::MAX as i64 + 1);
    assert_eq!(
        <EnumConstant as DeRon>::deserialize_ron(&max_ron).unwrap(),
        EnumConstant { value: i32::MAX }
    );

    assert!(matches!(
        <EnumConstant as DeRon>::deserialize_ron(&wrap_ron)
            .unwrap_err()
            .msg,
        DeRonErrReason::OutOfRange(v)
            if v == format!("{}>{}", (i32::MAX as i64 + 1), i32::MAX),
    ));
}

#[test]
fn ron_crate() {
    use nanoserde as renamed;

    #[derive(renamed::DeRon)]
    #[nserde(crate = "renamed")]
    pub struct Test {
        a: i32,
        b: f32,
        c: Option<String>,
        d: Option<String>,
    }

    let ron = r#"(
        a: 1,
        b: 2.0,
        d: "hello",
    )"#;

    let test: Test = renamed::DeRon::deserialize_ron(ron).unwrap();
    assert_eq!(test.a, 1);
    assert_eq!(test.b, 2.);
    assert_eq!(test.c, None);
    assert_eq!(test.d.unwrap(), "hello");
}

#[test]
fn no_whitespace_when_serialized() {
    // A vec of every type which implements `SerRon`. Actual values were picked arbitrarily.
    let mut rons: Vec<Box<dyn SerRon>> = vec![
        Box::new(()),
        Box::new((0, 1.0)),
        Box::new((0, 1.0, [2])),
        Box::new((0, 1.0, [2], false)),
        Box::new((0..5).collect::<BTreeSet<i32>>()),
        Box::new((0..5).map(|x| (x, true)).collect::<BTreeMap<i32, bool>>()),
        Box::new(Box::new(12_usize)),
        Box::new((0..5).collect::<LinkedList<i32>>()),
        Box::new(Some(false)),
        Box::new(None::<bool>),
        Box::new(String::from("a_string!")),
        Box::new(vec![false, true, false]),
        Box::new([true, false, true]),
        Box::new(true),
        Box::new(-32.0_f32),
        Box::new(64.0_f64),
        Box::new(-8_i8),
        Box::new(-16_i16),
        Box::new(-32_i32),
        Box::new(-64_i64),
        Box::new(8_u8),
        Box::new(16_u16),
        Box::new(32_u32),
        Box::new(64_u64),
        Box::new(usize::MAX),
    ];

    #[cfg(feature = "std")]
    {
        rons.push(Box::new((0..5).collect::<HashSet<i32>>()));
        rons.push(Box::new(
            (0..5).map(|x| (x, true)).collect::<HashMap<i32, bool>>(),
        ));
    }

    for ron in rons {
        let serialized = ron.serialize_ron();
        let no_whitespace = serialized.chars().all(|char| !char.is_whitespace());

        assert!(no_whitespace);
    }
}

#[test]
fn generic_enum() {
    #[derive(DeRon, PartialEq, Debug)]
    pub enum Foo<T, U>
    where
        T: Copy,
        U: Clone,
    {
        A,
        B(T, String),
        C { a: U, b: String },
    }

    #[derive(DeRon, PartialEq, Debug)]
    pub struct Bar<T, U>
    where
        T: Copy,
        U: Clone,
    {
        foo1: Foo<T, U>,
        foo2: Foo<T, U>,
        foo3: Foo<T, U>,
    }

    let ron = r#"
       (
          foo1: A,
          foo2: B(1, "asd"),
          foo3: C(a: 2, b: "qwe"),
       )
    "#;

    let test: Bar<i32, u64> = DeRon::deserialize_ron(ron).unwrap();

    assert_eq!(test.foo1, Foo::A);
    assert_eq!(test.foo2, Foo::B(1, "asd".to_string()));
    assert_eq!(
        test.foo3,
        Foo::C {
            a: 2,
            b: "qwe".to_string()
        }
    );
}

#[cfg(feature = "std")]
#[test]
fn std_time() {
    use std::time::{Duration, SystemTime};

    // Duration round trip
    let durations = [
        Duration::new(0, 0),
        Duration::new(42, 123_456_789),
        Duration::new(u64::MAX, 999_999_999),
    ];
    for dur in durations {
        let serialized = SerRon::serialize_ron(&dur);
        let deserialized: Duration = DeRon::deserialize_ron(&serialized).unwrap();
        assert_eq!(dur, deserialized);
    }

    // Duration error cases
    assert!(Duration::deserialize_ron(r#""invalid""#).is_err());
    assert!(Duration::deserialize_ron(r#"(secs: 1000, nanos: 1000000001)"#).is_err()); // Nanos = 1B (invalid)
    assert!(Duration::deserialize_ron(r#""""#).is_err()); // Empty string

    // SystemTime round trip
    let times = [
        SystemTime::UNIX_EPOCH,
        SystemTime::UNIX_EPOCH + Duration::new(42, 0),
        SystemTime::UNIX_EPOCH + Duration::new(1_640_995_200, 500_000_000),
    ];
    for time in times {
        let serialized = SerRon::serialize_ron(&time);
        let deserialized: SystemTime = DeRon::deserialize_ron(&serialized).unwrap();
        assert_eq!(time, deserialized);
    }

    // SystemTime error cases
    assert!(SystemTime::deserialize_ron(r#""invalid""#).is_err());
    assert!(SystemTime::deserialize_ron(r#""""#).is_err()); // Empty string

    // Combined struct test
    #[derive(DeRon, SerRon, PartialEq, Debug)]
    pub struct Test {
        pub duration: Duration,
        pub system_time: SystemTime,
    }

    let test = Test {
        duration: Duration::new(1000, 999_999_999),
        system_time: SystemTime::UNIX_EPOCH + Duration::new(1633072800, 500_000_000),
    };
    let serialized = SerRon::serialize_ron(&test);
    let deserialized = DeRon::deserialize_ron(&serialized).unwrap();
    assert_eq!(test, deserialized);

    // Deserialize none test for SystemTime
    let none = r#"None"#;
    let deserialized_none: SystemTime = DeRon::deserialize_ron(none).unwrap();
    assert_eq!(deserialized_none, SystemTime::UNIX_EPOCH);
}
