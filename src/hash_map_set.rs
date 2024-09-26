use std::collections::HashMap;
use std::collections::HashSet;

#[test] fn basic_usage() {
    //
    let s = HashSet::from([3,2,1]);
    assert_eq!(s, HashSet::from([3,2,1]));
    assert_eq!(s, HashSet::from([1,2,3]));

    let mut map = HashMap::from([ (1, "1"), (3, "3"), (2, "2")]);

    // unordered compare
    assert_eq!(map, HashMap::from([ (1, "1"), (2, "2"), (3, "3")]));
    assert_eq!(map.len(), 3);

    // entry API
    assert_eq!(map.get(&4), None);
    // insert a key ONLY IF it doesn't already exist
    map.entry(4).or_insert("4");
    assert_eq!(map.get(&4), Some(&"4"));
    //
    map.entry(4).and_modify( |e| *e = "4_updated" );
    assert_eq!(map.get(&4), Some(&"4_updated"));

    map.entry(5).and_modify( |e| *e = "4_updated" );
    assert_eq!(map.get(&5), None);
    //
    map.entry(5).and_modify( |e| *e = "5_updated" ).or_insert("5_new");
    assert_eq!(map.get(&5), Some(&"5_new"));

    map.entry(5).and_modify( |e| *e = "5_updated" ).or_insert("5_new");
    assert_eq!(map.get(&5), Some(&"5_updated"));
}