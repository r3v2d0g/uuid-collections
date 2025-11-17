use std::array;

use super::*;

#[test]
fn v4() {
    let uuids: [_; 3] = array::from_fn(|_| Uuid::new_v4());

    let mut map = UuidMap::new();
    let mut set = UuidSet::new();

    map.insert(uuids[0], 0);
    map.insert(uuids[2], 2);

    set.insert(uuids[0]);
    set.insert(uuids[2]);

    assert_eq!(map.get(&uuids[0]), Some(&0));
    assert!(!map.contains_key(&uuids[1]));
    assert_eq!(map.get(&uuids[2]), Some(&2));

    assert!(set.contains(&uuids[0]));
    assert!(!set.contains(&uuids[1]));
    assert!(set.contains(&uuids[2]));

    map.remove(&uuids[2]);
    set.remove(&uuids[2]);

    assert_eq!(map.get(&uuids[0]), Some(&0));
    assert!(!map.contains_key(&uuids[1]));
    assert!(!map.contains_key(&uuids[2]));

    assert!(set.contains(&uuids[0]));
    assert!(!set.contains(&uuids[1]));
    assert!(!set.contains(&uuids[2]));
}

#[test]
fn v7() {
    let uuids: [_; 3] = array::from_fn(|_| Uuid::now_v7());

    let mut map = UuidMap::new();
    let mut set = UuidSet::new();

    map.insert(uuids[0], 0);
    map.insert(uuids[2], 2);

    set.insert(uuids[0]);
    set.insert(uuids[2]);

    assert_eq!(map.get(&uuids[0]), Some(&0));
    assert!(!map.contains_key(&uuids[1]));
    assert_eq!(map.get(&uuids[2]), Some(&2));

    assert!(set.contains(&uuids[0]));
    assert!(!set.contains(&uuids[1]));
    assert!(set.contains(&uuids[2]));

    map.remove(&uuids[2]);
    set.remove(&uuids[2]);

    assert_eq!(map.get(&uuids[0]), Some(&0));
    assert!(!map.contains_key(&uuids[1]));
    assert!(!map.contains_key(&uuids[2]));

    assert!(set.contains(&uuids[0]));
    assert!(!set.contains(&uuids[1]));
    assert!(!set.contains(&uuids[2]));
}

#[test]
#[should_panic]
fn map_v1() {
    let uuid = Uuid::now_v1(&[0; 6]);

    let mut map = UuidMap::new();
    map.insert(uuid, 0);
}

#[test]
#[should_panic]
fn set_v1() {
    let uuid = Uuid::now_v1(&[0; 6]);

    let mut set = UuidSet::new();
    set.insert(uuid);
}
