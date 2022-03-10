use hyperloglog::Precision;

#[test]
fn size_of() {
    use std::mem::size_of;
    assert_eq!(size_of::<Precision>(), size_of::<u8>());
    assert_eq!(size_of::<Option<Precision>>(), size_of::<u8>());
}

#[test]
fn min_max() {
    assert_eq!(Precision::MIN.get(), 4);
    assert_eq!(Precision::MAX.get(), 18);
}

#[test]
fn in_range() {
    assert!(!Precision::in_range(0));
    assert!(Precision::in_range(4));
    assert!(Precision::in_range(12));
    assert!(Precision::in_range(18));
    assert!(!Precision::in_range(20));
}

#[test]
fn new() {
    assert_eq!(Precision::new(0), None);
    assert_eq!(Precision::new(4), Some(Precision::P4));
    assert_eq!(Precision::new(12), Some(Precision::P12));
    assert_eq!(Precision::new(18), Some(Precision::P18));
    assert_eq!(Precision::new(20), None);
}

#[test]
fn into_int() {
    assert_eq!(Into::<u8>::into(Precision::P12), 12);
}

#[test]
fn try_from_int() {
    assert!(TryInto::<Precision>::try_into(0).is_err());
    assert_eq!(TryInto::<Precision>::try_into(4), Ok(Precision::P4));
    assert_eq!(TryInto::<Precision>::try_into(12), Ok(Precision::P12));
    assert_eq!(TryInto::<Precision>::try_into(18), Ok(Precision::P18));
    assert!(TryInto::<Precision>::try_into(20).is_err());
}

#[test]
fn display() {
    assert_eq!(format!("{}", Precision::P12), "12");
}
