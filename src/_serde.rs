use super::VecOrObject;
use serde::de::{Deserialize, Deserializer, Error, MapAccess, Visitor};
use serde::ser::{Serialize, Serializer};
use std::fmt::{Debug, Formatter};
use std::hash::Hash;
impl<T: Serialize + Debug + Clone + PartialEq + Hash> Serialize for VecOrObject<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        match self {
            VecOrObject::Array(v) => {
                if v.len() == 1 {
                    v[0].serialize(serializer)
                } else {
                    v.serialize(serializer)
                }
            }
            VecOrObject::Object(v) => v.serialize(serializer),
        }
    }
}
impl<'de, T: Deserialize<'de>> Deserialize<'de>
for VecOrObject<T>
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        struct ArrayOrObjectVisitor<T>(
            std::marker::PhantomData<T>,
        );
        impl<'de, T: Deserialize<'de>> Visitor<'de>
        for ArrayOrObjectVisitor<T>
        {
            type Value = VecOrObject<T>;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("an array or an object")
            }
            fn visit_none<E>(self) -> Result<Self::Value, E>
                where
                    E: Error,
            {
                Ok(VecOrObject::Array(Vec::new()))
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
                where
                    A: serde::de::SeqAccess<'de>,
            {
                let mut v = Vec::with_capacity(seq.size_hint().unwrap_or(1));
                while let Some(elem) = seq.next_element()? {
                    v.push(elem);
                }
                if v.len() == 1 {
                    Ok(VecOrObject::Object(v.remove(0)))
                } else {
                    Ok(VecOrObject::Array(v))
                }
            }

            fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
                where
                    A: MapAccess<'de>,
            {
                Ok(VecOrObject::Object(T::deserialize(
                    serde::de::value::MapAccessDeserializer::new(map),
                )?))
            }
        }
        deserializer.deserialize_any(ArrayOrObjectVisitor(std::marker::PhantomData))
    }
    fn deserialize_in_place<D>(deserializer: D, place: &mut Self) -> Result<(), D::Error> where D: Deserializer<'de> {

        struct ArrayOrObjectVisitor<'a, T>(
            &'a mut VecOrObject<T>,
        );
        impl<'de:'a, 'a, T: Deserialize<'de>> Visitor<'de> for ArrayOrObjectVisitor<'a, T>{
            type Value = ();

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("an array or an object")
            }
            fn visit_none<E>(self) -> Result<Self::Value, E>
                where
                    E: Error,
            {
                if let VecOrObject::Array(v) = self.0 {
                    v.clear();
                } else {
                    *self.0 = VecOrObject::Array(Vec::new())
                }
                Ok(())
            }
            fn visit_seq<A>( self, mut seq: A) -> Result<Self::Value, A::Error>
                where
                    A: serde::de::SeqAccess<'de>,
            {
                let vec_ref =  if let VecOrObject::Array(v) = self.0 {
                    v.clear();
                    v
                } else {
                    *self.0 = VecOrObject::Array(Vec::with_capacity(seq.size_hint().unwrap_or(1)));
                    if let VecOrObject::Array(v) = self.0 {
                        v
                    } else {
                        unreachable!()
                    }
                };
                while let Some(elem) = seq.next_element()? {
                    vec_ref.push(elem);
                }
                if vec_ref.len() == 1 {
                    *self.0 = VecOrObject::Object(vec_ref.remove(0));
                }
                Ok(())
            }
            fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error> where A: MapAccess<'de> {
                *self.0 = VecOrObject::Object(T::deserialize(
                    serde::de::value::MapAccessDeserializer::new(map),
                )?);

                Ok(())
            }
        }
        deserializer.deserialize_any(ArrayOrObjectVisitor(place))
    }
}