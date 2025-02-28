new_type::newtype!(Uint32: u32);

#[cfg(feature = "postgres")]
mod postgres {
    use super::Uint32;
    use bytes::BytesMut;
    use postgres_types::Format;
    use postgres_types::FromSql;
    use postgres_types::IsNull;
    use postgres_types::ToSql;
    use postgres_types::Type;

    impl ToSql for Uint32 {
        fn to_sql(
            &self,
            ty: &Type,
            out: &mut BytesMut,
        ) -> Result<IsNull, Box<dyn std::error::Error + Sync + Send>>
        where
            Self: Sized,
        {
            let value = self.0 as i64;
            <i64 as ToSql>::to_sql(&value, ty, out)
        }
        fn accepts(ty: &Type) -> bool
        where
            Self: Sized,
        {
            <i64 as ToSql>::accepts(ty)
        }
        fn to_sql_checked(
            &self,
            ty: &Type,
            out: &mut BytesMut,
        ) -> Result<IsNull, Box<dyn std::error::Error + Sync + Send>> {
            let value = self.0 as i64;
            <i64 as ToSql>::to_sql_checked(&value, ty, out)
        }
        fn encode_format(&self, ty: &Type) -> Format {
            let value = self.0 as i64;
            <i64 as ToSql>::encode_format(&value, ty)
        }
    }
    impl Uint32 {
        fn try_from_i64(value: i64) -> Result<Self, Box<dyn std::error::Error + Sync + Send>> {
            if 0 > value {
                return Err(format!(
                    "Negative integer {} cannot be converted to positive integer.",
                    value
                )
                .into());
            } else if (u32::MAX as i64) < value {
                return Err(format!("The integer {} is too large.", value).into());
            } else {
                return Ok(Self(value as u32));
            }
        }
    }
    impl<'a> FromSql<'a> for Uint32 {
        fn from_sql(
            ty: &Type,
            raw: &'a [u8],
        ) -> Result<Self, Box<dyn std::error::Error + Sync + Send>> {
            let value = <i64 as FromSql>::from_sql(ty, raw)?;
            return Self::try_from_i64(value);
        }

        fn from_sql_null(ty: &Type) -> Result<Self, Box<dyn std::error::Error + Sync + Send>> {
            let value = <i64 as FromSql>::from_sql_null(ty)?;
            return Self::try_from_i64(value);
        }

        fn from_sql_nullable(
            ty: &Type,
            raw: Option<&'a [u8]>,
        ) -> Result<Self, Box<dyn std::error::Error + Sync + Send>> {
            let value = <i64 as FromSql>::from_sql_nullable(ty, raw)?;
            return Self::try_from_i64(value);
        }

        fn accepts(ty: &Type) -> bool {
            <i64 as FromSql>::accepts(ty)
        }
    }
}
