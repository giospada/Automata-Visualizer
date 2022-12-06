#[macro_export]
macro_rules! set {
    () => {
        std::collections::BTreeSet::new()
    };
    ($($x:expr),*) => {
        {
            let mut set = std::collections::BTreeSet::new();
            $(
                set.insert($x);
            )*
            set
        }
    };
}

#[macro_export]
macro_rules! map {
    () => {
        std::collections::BTreeMap::new()
    };
    ($($k:expr => $v:expr),*) => {
        {
            let mut map = std::collections::BTreeMap::new();
            $(
                map.insert($k, $v);
            )*
            map
        }
    };
}

#[cfg(test)]
mod test {
    #[test]
    fn test_set(){
        let mut set = std::collections::BTreeSet::new();
        set.insert(1);
        set.insert(2);
        set.insert(3);
        set.insert(4);
        assert_eq!(set![1,2,3,4, 4,4],set);
    }

    #[test]
    fn test_empty_set(){
        let set: std::collections::BTreeSet<i32> = set![];
        assert_eq!(set![],set);
    }

    #[test]
    fn test_map(){
        let mut map = std::collections::BTreeMap::new();
        map.insert(1, "one");
        map.insert(2, "two");
        map.insert(3, "three");
        map.insert(4, "four");
        assert_eq!(map![1 => "one", 2 => "two", 3 => "three", 4 => "four"],map);
    }
}