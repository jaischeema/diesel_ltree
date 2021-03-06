#[macro_use]
extern crate diesel;
#[cfg(test)]
#[macro_use]
extern crate diesel_codegen;

#[cfg(test)]
mod tests;

mod types {
    use diesel::pg::{Pg, PgMetadataLookup, PgTypeMetadata};
    use diesel::types::HasSqlType;

    #[derive(Clone, Copy)]
    pub struct Ltree;

    impl HasSqlType<Ltree> for Pg {
        fn metadata(lookup: &PgMetadataLookup) -> PgTypeMetadata {
            lookup.lookup_type("ltree")
        }
    }

    impl_query_id!(Ltree);

    #[derive(Clone, Copy)]
    pub struct Lquery;

    impl HasSqlType<Lquery> for Pg {
        fn metadata(lookup: &PgMetadataLookup) -> PgTypeMetadata {
            lookup.lookup_type("lquery")
        }
    }

    impl_query_id!(Lquery);


    #[derive(Clone, Copy)]
    pub struct Ltxtquery;

    impl HasSqlType<Ltxtquery> for Pg {
        fn metadata(lookup: &PgMetadataLookup) -> PgTypeMetadata {
            lookup.lookup_type("ltxtquery")
        }
    }

    impl_query_id!(Ltxtquery);
}

mod functions {
    use types::*;
    use diesel::types::*;

    sql_function!(subltree, subltree_t, (ltree: Ltree, start: Int4, end: Int4) -> Ltree);
    sql_function!(subpath, subpath_t, (ltree: Ltree, offset: Int4, len: Int4) -> Ltree);
    // sql_function!(subpath, subpath_t, (ltree: Ltree, offset: Int4) -> Ltree);
    sql_function!(nlevel, nlevel_t, (ltree: Ltree) -> Int4);
    sql_function!(index, index_t, (a: Ltree, b: Ltree) -> Int4);
    // sql_function!(index, index_t, (a: Ltree, b: Ltree, offset: Int4) -> Int4);
    sql_function!(text2ltree, text2ltree_t, (text: Text) -> Ltree);
    sql_function!(ltree2text, ltree2text_t, (ltree: Ltree) -> Text);

    sql_function!(lquery, lquery_t, (x: Text) -> Lquery);
    sql_function!(ltxtquery, ltxtquery_t, (x: Text) -> Ltxtquery);
}

mod dsl {
    use types::*;
    use diesel::expression::{AsExpression, Expression};
    use diesel::types::SingleValue;

    mod predicates {
        use types::*;
        use diesel::pg::Pg;

        diesel_infix_operator!(Contains, " @> ", backend: Pg);
        diesel_infix_operator!(ContainedBy, " <@ ", backend: Pg);
        diesel_infix_operator!(Matches, " ~ ", backend: Pg);
        diesel_infix_operator!(TMatches, " @ ", backend: Pg);
        diesel_infix_operator!(Concat, " || ", Ltree, backend: Pg);
    }

    use self::predicates::*;

    impl SingleValue for Ltree {}

    pub trait LtreeExtensions: Expression<SqlType = Ltree> + Sized {
        fn contains<T: AsExpression<Ltree>>(self, other: T) -> Contains<Self, T::Expression> {
            Contains::new(self, other.as_expression())
        }

        fn contained_by<T: AsExpression<Ltree>>(
            self,
            other: T,
        ) -> ContainedBy<Self, T::Expression> {
            ContainedBy::new(self, other.as_expression())
        }

        fn matches<T: AsExpression<Lquery>>(self, other: T) -> Matches<Self, T::Expression> {
            Matches::new(self, other.as_expression())
        }

        fn tmatches<T: AsExpression<Ltxtquery>>(self, other: T) -> TMatches<Self, T::Expression> {
            TMatches::new(self, other.as_expression())
        }

        fn concat<T: AsExpression<Ltree>>(self, other: T) -> Concat<Self, T::Expression> {
            Concat::new(self, other.as_expression())
        }
    }

    pub trait LqueryExtensions: Expression<SqlType = Lquery> + Sized {
        fn matches<T: AsExpression<Ltree>>(self, other: T) -> Matches<Self, T::Expression> {
            Matches::new(self, other.as_expression())
        }
    }

    pub trait LtxtqueryExtensions: Expression<SqlType = Ltxtquery> + Sized {
        fn tmatches<T: AsExpression<Ltree>>(self, other: T) -> TMatches<Self, T::Expression> {
            TMatches::new(self, other.as_expression())
        }
    }

    impl<T: Expression<SqlType = Ltree>> LtreeExtensions for T {}
    impl<T: Expression<SqlType = Lquery>> LqueryExtensions for T {}
    impl<T: Expression<SqlType = Ltxtquery>> LtxtqueryExtensions for T {}
}

pub use self::types::*;
pub use self::functions::*;
pub use self::dsl::*;
