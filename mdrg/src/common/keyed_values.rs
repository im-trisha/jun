use serde::ser::{Serialize, SerializeSeq, SerializeStruct, Serializer};
use serde::{Deserialize, Deserializer};
use std::collections::HashMap;
use std::hash::Hash;

/// An "hollow" type containing a key-value pair of [K]->[T]
///
/// In the save files, some json structs are like such:
/// ```json
/// {
///     "keys": ["KeyA"]
///     "values": {valueRepresentingKeyA}
/// }
/// ```
///
/// An example of this, is the dialogue chains, the join us blog, the rng compensation data...
/// So it has been mapped through this class, having a more rusty interface
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[derive(Clone)]
pub struct KeyedValues<K, V>
where
    K: Eq + Hash,
{
    pub map: HashMap<K, V>,
}

impl<K, V> Default for KeyedValues<K, V>
where
    K: Eq + Hash,
{
    fn default() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
}

/// This type is, as the name suggests, a proxy to rustify the `SerializableDictionary<TKey, TValue>` C# type
///
/// (TypeDefIndex: 2027)
#[derive(Deserialize)]
struct ParallelProxy<K, V> {
    keys: Vec<K>,
    values: Vec<V>,
}

impl<K, V> Serialize for KeyedValues<K, V>
where
    K: Serialize + Eq + Hash,
    V: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("ParallelProxy", 2)?;

        state.serialize_field("keys", &MapKeysSerializeWrapper(&self.map))?;
        state.serialize_field("values", &MapValuesSerializeWrapper(&self.map))?;

        state.end()
    }
}

impl<'de, K, V> Deserialize<'de> for KeyedValues<K, V>
where
    K: Deserialize<'de> + Eq + Hash,
    V: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let proxy = ParallelProxy::<K, V>::deserialize(deserializer)?;
        let map = proxy.keys.into_iter().zip(proxy.values).collect();
        Ok(KeyedValues { map })
    }
}

/// Helper to serialize only the keys of the HashMap as a sequence
struct MapKeysSerializeWrapper<'a, K, V>(&'a HashMap<K, V>);
impl<'a, K, V> Serialize for MapKeysSerializeWrapper<'a, K, V>
where
    K: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.0.len()))?;
        for k in self.0.keys() {
            seq.serialize_element(k)?;
        }
        seq.end()
    }
}

/// Helper to serialize only the values of the HashMap as a sequence
struct MapValuesSerializeWrapper<'a, K, V>(&'a HashMap<K, V>);
impl<'a, K, V> Serialize for MapValuesSerializeWrapper<'a, K, V>
where
    V: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.0.len()))?;
        for v in self.0.values() {
            seq.serialize_element(v)?;
        }
        seq.end()
    }
}
