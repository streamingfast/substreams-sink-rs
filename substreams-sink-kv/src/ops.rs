use crate::pb::sf::substreams::sink::kv::v1::{kv_operation::Type, KVOperation, KVOperations};

impl KVOperations {
    /// Append a [`Type::SET`] operation, writing `value` at `key`.
    ///
    /// `ordinal` orders the operations within the block.
    pub fn push_new<K: AsRef<str>, V: AsRef<[u8]>>(&mut self, key: K, value: V, ordinal: u64) {
        self.operations.push(KVOperation {
            key: key.as_ref().to_string(),
            value: value.as_ref().to_vec(),
            ordinal,
            r#type: Type::Set.into(),
        })
    }
    /// Append a [`Type::DELETE`] operation, removing `key`.
    ///
    /// `ordinal` orders the operations within the block.
    pub fn push_delete<V: AsRef<str>>(&mut self, key: V, ordinal: u64) {
        self.operations.push(KVOperation {
            key: key.as_ref().to_string(),
            value: vec![],
            ordinal,
            r#type: Type::Delete.into(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use buffa::Message;

    #[test]
    fn push_new_sets_type_set() {
        let mut ops = KVOperations::default();
        ops.push_new("key", b"value", 7);

        assert_eq!(ops.operations.len(), 1);
        let op = &ops.operations[0];
        assert_eq!(op.key, "key");
        assert_eq!(op.value, b"value");
        assert_eq!(op.ordinal, 7);
        assert_eq!(op.r#type, Type::SET);
        assert_eq!(op.r#type.to_i32(), 1);
    }

    #[test]
    fn push_delete_sets_type_delete_and_empty_value() {
        let mut ops = KVOperations::default();
        ops.push_delete("key", 9);

        assert_eq!(ops.operations.len(), 1);
        let op = &ops.operations[0];
        assert_eq!(op.key, "key");
        assert!(op.value.is_empty());
        assert_eq!(op.ordinal, 9);
        assert_eq!(op.r#type, Type::DELETE);
        assert_eq!(op.r#type.to_i32(), 2);
    }

    #[test]
    fn never_emits_unset() {
        let mut ops = KVOperations::default();
        ops.push_new("a", b"1", 0);
        ops.push_delete("b", 1);

        for op in &ops.operations {
            assert_ne!(
                op.r#type.to_i32(),
                0,
                "UNSET makes substreams-sink-kv panic in db.Flush"
            );
        }
    }

    #[test]
    fn encodes_to_expected_wire_bytes() {
        let mut ops = KVOperations::default();
        ops.push_new("k", b"v", 3);

        // field 1 (operations), length 10:
        //   field 1 (key) "k", field 2 (value) "v", field 3 (ordinal) 3, field 4 (type) SET
        assert_eq!(
            ops.encode_to_vec(),
            vec![0x0a, 0x0a, 0x0a, 0x01, b'k', 0x12, 0x01, b'v', 0x18, 0x03, 0x20, 0x01]
        );
    }

    #[test]
    fn delete_round_trips_through_the_wire() {
        let mut ops = KVOperations::default();
        ops.push_delete("gone", 11);

        let decoded = KVOperations::decode_from_slice(&ops.encode_to_vec()).unwrap();

        assert_eq!(decoded.operations.len(), 1);
        assert_eq!(decoded.operations[0].key, "gone");
        assert_eq!(decoded.operations[0].r#type, Type::DELETE);
    }
}
