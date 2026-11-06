use super::{KeyValStore, KeyValStoreError};

#[test]
fn new_store_is_empty_and_missing_keys_are_reported() {
    let store = KeyValStore::new();

    assert!(store.list().is_empty());
    assert!(matches!(
        store.get("missing".into()),
        Err(KeyValStoreError::KeyNotFoundError { key }) if key == "missing"
    ));
}

#[test]
fn insert_and_get_keep_values_for_independent_keys() {
    let mut store = KeyValStore::new();
    assert!(store.insert("first".into(), "one".into()).is_ok());
    assert!(store.insert("second".into(), "two".into()).is_ok());

    assert_eq!(store.get("first".into()).unwrap(), "one");
    assert_eq!(store.get("second".into()).unwrap(), "two");
}

#[test]
fn duplicate_insert_reports_key_and_preserves_existing_value() {
    let mut store = KeyValStore::new();
    store.insert("key".into(), "original".into()).unwrap();

    assert!(matches!(
        store.insert("key".into(), "replacement".into()),
        Err(KeyValStoreError::KeyAlreadyInStore { key }) if key == "key"
    ));
    assert_eq!(store.get("key".into()).unwrap(), "original");
}

#[test]
fn update_returns_previous_value_and_changes_only_the_requested_key() {
    let mut store = KeyValStore::new();
    store.insert("key".into(), "old".into()).unwrap();
    store.insert("other".into(), "untouched".into()).unwrap();

    assert_eq!(store.update("key".into(), "new".into()).unwrap(), "old");
    assert_eq!(store.get("key".into()).unwrap(), "new");
    assert_eq!(store.get("other".into()).unwrap(), "untouched");
}

#[test]
fn updating_a_missing_key_reports_it_without_creating_it() {
    let mut store = KeyValStore::new();

    assert!(matches!(
        store.update("missing".into(), "value".into()),
        Err(KeyValStoreError::KeyNotFoundError { key }) if key == "missing"
    ));
    assert!(store.list().is_empty());
}

#[test]
fn delete_removes_only_the_requested_key_and_allows_reinsertion() {
    let mut store = KeyValStore::new();
    store.insert("key".into(), "old".into()).unwrap();
    store.insert("other".into(), "kept".into()).unwrap();

    assert!(store.delete("key".into()).is_ok());
    assert!(matches!(
        store.get("key".into()),
        Err(KeyValStoreError::KeyNotFoundError { key }) if key == "key"
    ));
    assert_eq!(store.get("other".into()).unwrap(), "kept");
    assert!(store.insert("key".into(), "new".into()).is_ok());
    assert_eq!(store.get("key".into()).unwrap(), "new");
}

#[test]
fn deleting_a_missing_key_reports_it_and_leaves_existing_entries() {
    let mut store = KeyValStore::new();
    store.insert("present".into(), "value".into()).unwrap();

    assert!(matches!(
        store.delete("missing".into()),
        Err(KeyValStoreError::KeyNotFoundError { key }) if key == "missing"
    ));
    assert_eq!(store.get("present".into()).unwrap(), "value");
}

#[test]
fn list_reflects_insertions_and_deletions_without_requiring_an_order() {
    let mut store = KeyValStore::new();
    store.insert("b".into(), "two".into()).unwrap();
    store.insert("a".into(), "one".into()).unwrap();
    store.insert("c".into(), "three".into()).unwrap();
    store.delete("b".into()).unwrap();

    let mut keys = store.list();
    keys.sort();
    assert_eq!(keys, vec!["a", "c"]);
}

#[test]
fn empty_and_unicode_keys_and_values_round_trip() {
    let mut store = KeyValStore::new();
    store.insert("".into(), "".into()).unwrap();
    store.insert("鍵 🔑".into(), "値 🌊".into()).unwrap();

    assert_eq!(store.get("".into()).unwrap(), "");
    assert_eq!(store.get("鍵 🔑".into()).unwrap(), "値 🌊");
}
