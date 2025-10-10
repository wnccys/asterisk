#[cfg(test)]
mod hash_table {
    use std::{cell::RefCell, rc::Rc};
    use asterisk::{objects::hash_table::{Entry, HashTable}, primitives::{primitive::Primitive, value::Value}};

    #[test]
    fn insert_get_single() {
        let var_name = String::from("v");

        let mut table = HashTable::<String, Value>::default();
        table.insert(
            &var_name,
            Value {
                value: Primitive::Int(2),
                ..Default::default()
            }
        );

        assert!(table.get(&var_name).is_some());
    }

    #[test]
    fn insert_get_multi() {
        let mut table = HashTable::<String, Value>::default();
        table.insert(
            &String::from("a"),
            Value {
                value: Primitive::Int(1),
                ..Default::default()
            }
        );

        table.insert(
            &String::from("b"),
            Value {
                value: Primitive::Int(2),
                ..Default::default()
            }
        );

        table.insert(
            &String::from("c"),
            Value {
                value: Primitive::Int(3),
                ..Default::default()
            }
        );

        table.insert(
            &String::from("d"),
            Value {
                value: Primitive::Int(4),
                ..Default::default()
            }
        );

        assert!(table.get(&String::from("a")).is_some());
        assert_eq!(table.get(&String::from("a")).unwrap().borrow().value, Primitive::Int(1));
        assert!(table.get(&String::from("b")).is_some());
        assert_eq!(table.get(&String::from("b")).unwrap().borrow().value, Primitive::Int(2));
        assert!(table.get(&String::from("c")).is_some());
        assert_eq!(table.get(&String::from("c")).unwrap().borrow().value, Primitive::Int(3));
        assert!(table.get(&String::from("d")).is_some());
        assert_eq!(table.get(&String::from("d")).unwrap().borrow().value, Primitive::Int(4));
    }

    #[test]
    fn reassign() {
        let mut table = HashTable::<String, Value>::default();
        table.insert(
            &String::from("a"),
            Value {
                value: Primitive::Int(1),
                ..Default::default()
            }
        );

        table.insert(
            &String::from("b"),
            Value {
                value: Primitive::Int(2),
                ..Default::default()
            }
        );

        table.insert(
            &String::from("c"),
            Value {
                value: Primitive::Int(3),
                ..Default::default()
            }
        );

        table.insert(
            &String::from("d"),
            Value {
                value: Primitive::Int(4),
                ..Default::default()
            }
        );
        
        table.insert(
            &String::from("a"),
            Value {
                value: Primitive::String(String::from("newa")),
                ..Default::default()
            }
        );

        assert!(table.get(&String::from("a")).is_some());
        assert_eq!(table.get(&String::from("a")).unwrap().borrow().value, Primitive::String(String::from("newa")));
        assert!(table.get(&String::from("b")).is_some());
        assert_eq!(table.get(&String::from("b")).unwrap().borrow().value, Primitive::Int(2));
        assert!(table.get(&String::from("c")).is_some());
        assert_eq!(table.get(&String::from("c")).unwrap().borrow().value, Primitive::Int(3));
        assert!(table.get(&String::from("d")).is_some());
        assert_eq!(table.get(&String::from("d")).unwrap().borrow().value, Primitive::Int(4));
    }

    #[test]
    fn reassign_multi() {
        let mut table = HashTable::<String, Value>::default();
        table.insert(
            &String::from("a"),
            Value {
                value: Primitive::Int(1),
                ..Default::default()
            }
        );

        table.insert(
            &String::from("b"),
            Value {
                value: Primitive::Int(2),
                ..Default::default()
            }
        );

        table.insert(
            &String::from("c"),
            Value {
                value: Primitive::Int(3),
                ..Default::default()
            }
        );

        table.insert(
            &String::from("d"),
            Value {
                value: Primitive::Int(4),
                ..Default::default()
            }
        );
        
        table.insert(
            &String::from("a"),
            Value {
                value: Primitive::String(String::from("newa")),
                ..Default::default()
            }
        );

        table.insert(
            &String::from("c"),
            Value {
                value: Primitive::String(String::from("newc")),
                ..Default::default()
            }
        );

        assert!(table.get(&String::from("a")).is_some());
        assert_eq!(table.get(&String::from("a")).unwrap().borrow().value, Primitive::String(String::from("newa")));
        assert!(table.get(&String::from("b")).is_some());
        assert_eq!(table.get(&String::from("b")).unwrap().borrow().value, Primitive::Int(2));
        assert!(table.get(&String::from("c")).is_some());
        assert_eq!(table.get(&String::from("c")).unwrap().borrow().value, Primitive::String(String::from("newc")));
        assert!(table.get(&String::from("d")).is_some());
        assert_eq!(table.get(&String::from("d")).unwrap().borrow().value, Primitive::Int(4));
    }


    #[test]
    fn size_factor() {
        let table = HashTable::<String, Value>::default();
    }

    #[test]
    fn _insert_get_multi() {}

    #[test]
    fn __insert_get_multi() {}

    #[test]
    fn probe_idx() {
        let mut entries: Vec<Option<Entry<String, i32>>> = vec![None; 4];
        assert_eq!(HashTable::<String, i32>::probe_idx(&entries, 0), 1);

        entries[0] = Some((String::from("n"), Rc::new(RefCell::new(2))));
        assert_eq!(HashTable::<String, i32>::probe_idx(&entries, 0), 1);

        entries[2] = Some((String::from("m"), Rc::new(RefCell::new(3))));
        assert_eq!(HashTable::<String, i32>::probe_idx(&entries, 0), 1);
    }
}