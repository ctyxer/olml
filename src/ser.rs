use serde::{Serialize, ser};

use crate::{Error, error::Result};

struct Serializer {
    output: String,
}

pub fn to_string<T>(value: &T) -> Result<String>
where
    T: Serialize,
{
    let mut serializer = Serializer {
        output: String::new(),
    };

    value.serialize(&mut serializer)?;

    Ok(serializer.output)
}

impl<'a> ser::Serializer for &'a mut Serializer {
    type Ok = ();

    type Error = Error;

    type SerializeSeq = Self;

    type SerializeTuple = Self;

    type SerializeTupleStruct = Self;

    type SerializeTupleVariant = Self;

    type SerializeMap = Self;

    type SerializeStruct = Self;

    type SerializeStructVariant = Self;

    #[inline]
    fn serialize_bool(self, v: bool) -> std::result::Result<Self::Ok, Self::Error> {
        self.output += if v { "true" } else { "false" };
        Ok(())
    }

    #[inline]
    fn serialize_i8(self, v: i8) -> std::result::Result<Self::Ok, Self::Error> {
        self.output += &v.to_string();
        Ok(())
    }

    #[inline]
    fn serialize_i16(self, v: i16) -> std::result::Result<Self::Ok, Self::Error> {
        self.output += &v.to_string();
        Ok(())
    }

    #[inline]
    fn serialize_i32(self, v: i32) -> std::result::Result<Self::Ok, Self::Error> {
        self.output += &v.to_string();
        Ok(())
    }

    #[inline]
    fn serialize_i64(self, v: i64) -> std::result::Result<Self::Ok, Self::Error> {
        self.output += &v.to_string();
        Ok(())
    }

    #[inline]
    fn serialize_u8(self, v: u8) -> std::result::Result<Self::Ok, Self::Error> {
        self.output += &v.to_string();
        Ok(())
    }

    #[inline]
    fn serialize_u16(self, v: u16) -> std::result::Result<Self::Ok, Self::Error> {
        self.output += &v.to_string();
        Ok(())
    }

    #[inline]
    fn serialize_u32(self, v: u32) -> std::result::Result<Self::Ok, Self::Error> {
        self.output += &v.to_string();
        Ok(())
    }

    #[inline]
    fn serialize_u64(self, v: u64) -> std::result::Result<Self::Ok, Self::Error> {
        self.output += &v.to_string();
        Ok(())
    }

    #[inline]
    fn serialize_f32(self, v: f32) -> std::result::Result<Self::Ok, Self::Error> {
        self.output += &v.to_string();
        Ok(())
    }

    #[inline]
    fn serialize_f64(self, v: f64) -> std::result::Result<Self::Ok, Self::Error> {
        self.output += &v.to_string();
        Ok(())
    }

    #[inline]
    fn serialize_char(self, v: char) -> std::result::Result<Self::Ok, Self::Error> {
        self.serialize_str(&v.to_string())
    }

    #[inline]
    fn serialize_str(self, v: &str) -> std::result::Result<Self::Ok, Self::Error> {
        self.output += &format!("\"{v}\"");
        Ok(())
    }

    #[inline]
    fn serialize_bytes(self, v: &[u8]) -> std::result::Result<Self::Ok, Self::Error> {
        use serde::ser::SerializeSeq;
        let mut seq = self.serialize_seq(Some(v.len()))?;

        for byte in v {
            seq.serialize_element(byte)?;
        }

        seq.end()
    }

    #[inline]
    fn serialize_none(self) -> std::result::Result<Self::Ok, Self::Error> {
        self.output += "null";
        Ok(())
    }

    #[inline]
    fn serialize_some<T>(self, value: &T) -> std::result::Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    #[inline]
    fn serialize_unit(self) -> std::result::Result<Self::Ok, Self::Error> {
        self.serialize_none()
    }

    #[inline]
    fn serialize_unit_struct(
        self,
        _name: &'static str,
    ) -> std::result::Result<Self::Ok, Self::Error> {
        self.serialize_none()
    }

    #[inline]
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> std::result::Result<Self::Ok, Self::Error> {
        self.output += variant;
        Ok(())
    }

    #[inline]
    fn serialize_newtype_struct<T>(
        self,
        _name: &'static str,
        value: &T,
    ) -> std::result::Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    #[inline]
    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> std::result::Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.output += &format!("{{{variant}:");
        value.serialize(&mut *self)?;
        self.output += "}";
        Ok(())
    }

    #[inline]
    fn serialize_seq(
        self,
        _len: Option<usize>,
    ) -> std::result::Result<Self::SerializeSeq, Self::Error> {
        self.output += "[";
        Ok(self)
    }

    #[inline]
    fn serialize_tuple(self, len: usize) -> std::result::Result<Self::SerializeTuple, Self::Error> {
        self.serialize_seq(Some(len))
    }

    #[inline]
    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> std::result::Result<Self::SerializeTupleStruct, Self::Error> {
        self.output += "{[";
        Ok(self)
    }

    #[inline]
    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> std::result::Result<Self::SerializeTupleVariant, Self::Error> {
        self.output += "{";
        variant.serialize(&mut *self)?;
        self.output += ":[";
        Ok(self)
    }

    #[inline]
    fn serialize_map(
        self,
        _len: Option<usize>,
    ) -> std::result::Result<Self::SerializeMap, Self::Error> {
        self.output += "{";
        Ok(self)
    }

    #[inline]
    fn serialize_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> std::result::Result<Self::SerializeStruct, Self::Error> {
        self.serialize_map(Some(len))
    }

    #[inline]
    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> std::result::Result<Self::SerializeStructVariant, Self::Error> {
        self.output += &format!("{{{variant}:{{");
        Ok(self)
    }
}

impl<'a> ser::SerializeSeq for &'a mut Serializer {
    type Error = Error;

    type Ok = ();

    #[inline]
    fn serialize_element<T>(&mut self, value: &T) -> std::result::Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        if !self.output.ends_with("[") {
            self.output += " ";
        }

        value.serialize(&mut **self)
    }

    #[inline]
    fn end(self) -> std::result::Result<Self::Ok, Self::Error> {
        self.output += "]";
        Ok(())
    }
}

impl<'a> ser::SerializeMap for &'a mut Serializer {
    type Error = Error;

    type Ok = ();

    #[inline]
    fn end(self) -> std::result::Result<Self::Ok, Self::Error> {
        self.output += "}";
        Ok(())
    }

    #[inline]
    fn serialize_key<T>(&mut self, key: &T) -> std::result::Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        if !self.output.ends_with('{') {
            self.output += " ";
        }

        key.serialize(&mut **self)
    }

    #[inline]
    fn serialize_value<T>(&mut self, value: &T) -> std::result::Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.output += ":";

        value.serialize(&mut **self)
    }
}

impl<'a> ser::SerializeStruct for &'a mut Serializer {
    type Ok = ();

    type Error = Error;

    #[inline]
    fn serialize_field<T>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> std::result::Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        if !self.output.ends_with("{") {
            self.output += " ";
        }

        self.output += &format!("{key}:");
        value.serialize(&mut **self)
    }

    #[inline]
    fn end(self) -> std::result::Result<Self::Ok, Self::Error> {
        self.output += "}";
        Ok(())
    }
}
impl<'a> ser::SerializeStructVariant for &'a mut Serializer {
    type Ok = ();

    type Error = Error;

    #[inline]
    fn serialize_field<T>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> std::result::Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        if !self.output.ends_with('{') {
            self.output += " ";
        }

        self.output += &format!("{key}:");
        value.serialize(&mut **self)
    }

    #[inline]
    fn end(self) -> std::result::Result<Self::Ok, Self::Error> {
        self.output += "}}";
        Ok(())
    }
}

impl<'a> ser::SerializeTuple for &'a mut Serializer {
    type Ok = ();

    type Error = Error;

    #[inline]
    fn serialize_element<T>(&mut self, value: &T) -> std::result::Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        if !self.output.ends_with('[') {
            self.output += " ";
        }

        value.serialize(&mut **self)
    }

    #[inline]
    fn end(self) -> std::result::Result<Self::Ok, Self::Error> {
        self.output += "]";
        Ok(())
    }
}

impl<'a> ser::SerializeTupleVariant for &'a mut Serializer {
    type Ok = ();

    type Error = Error;

    #[inline]
    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        if !self.output.ends_with('[') {
            self.output += " ";
        }

        value.serialize(&mut **self)
    }

    #[inline]
    fn end(self) -> Result<()> {
        self.output += "]}";
        Ok(())
    }
}

impl<'a> ser::SerializeTupleStruct for &mut Serializer {
    type Ok = ();

    type Error = Error;

    #[inline]
    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        if !self.output.ends_with('[') {
            self.output += ",";
        }
        value.serialize(&mut **self)
    }

    #[inline]
    fn end(self) -> Result<()> {
        self.output += "]}";
        Ok(())
    }
}

////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod test {
    use serde::Serialize;

    use crate::to_string;

    #[test]
    fn test_unit_struct() {
        #[derive(Serialize)]
        struct S;

        let v = S;

        assert_eq!("null", to_string(&v).unwrap());
    }

    #[test]
    fn test_unit_variant() {
        #[derive(Serialize)]
        enum E {
            A,
        }

        let v = E::A;

        assert_eq!("A", to_string(&v).unwrap());
    }

    #[test]
    fn test_newtype_struct() {
        #[derive(Serialize)]
        struct S(u8);

        let v = S(47);

        assert_eq!("47", to_string(&v).unwrap());
    }

    #[test]
    fn test_newtype_variant() {
        #[derive(Serialize)]
        enum E {
            Dp(u8),
        }

        let v = E::Dp(47);

        assert_eq!("{Dp:47}", to_string(&v).unwrap());
    }

    #[test]
    fn test_seq_with_numbers() {
        let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        assert_eq!("[1 2 3 4 5 6 7 8 9 10]", to_string(&v).unwrap());
    }

    #[test]
    fn test_seq_with_strings() {
        let v = vec!["1", "2", "3", "4", "5", "6", "7", "8", "9", "10"];

        assert_eq!(
            "[\"1\" \"2\" \"3\" \"4\" \"5\" \"6\" \"7\" \"8\" \"9\" \"10\"]",
            to_string(&v).unwrap()
        );
    }

    #[test]
    fn test_tuple() {
        let v = (String::from("hlp"), vec!["D", "p"], 47);

        assert_eq!("[\"hlp\" [\"D\" \"p\"] 47]", to_string(&v).unwrap());
    }

    #[test]
    fn test_tuple_struct() {
        #[derive(Serialize)]
        struct S(u8, String);

        let v = S(47, "Dp".to_owned());

        assert_eq!("{[47,\"Dp\"]}", to_string(&v).unwrap());
    }

    #[test]
    fn test_tuple_variant() {
        #[derive(Serialize)]
        enum E {
            S(String, u8),
        }

        let v = E::S("Dp".to_owned(), 47);

        assert_eq!("{\"S\":[\"Dp\" 47]}", to_string(&v).unwrap());
    }

    #[test]
    fn test_map() {
        let v = std::collections::BTreeMap::from([(1, "6,5"), (47, "Dp")]);

        assert_eq!("{1:\"6,5\" 47:\"Dp\"}", to_string(&v).unwrap());
    }

    #[test]
    fn test_struct() {
        #[derive(Serialize)]
        struct S {
            r: u8,
            g: u8,
            b: u8,
        }

        let v = S { r: 1, g: 2, b: 4 };

        assert_eq!("{r:1 g:2 b:4}", to_string(&v).unwrap());
    }

    #[test]
    fn test_struct_variant() {
        #[derive(Serialize)]
        enum E {
            S { r: u8, g: u8, b: u8 },
        }

        let v = E::S { r: 1, g: 2, b: 4 };

        assert_eq!("{S:{r:1 g:2 b:4}}", to_string(&v).unwrap());
    }
}
