/// The `QueryBuilder` trait.
///
/// Primarily used for subqueries within conditions.
pub trait QueryBuilder<'a>: QueryBuilderClone<'a> {
    fn to_sql(&self) -> String;
}

// A helper trait to enable cloning of `Box<dyn QueryBuilder>`
pub trait QueryBuilderClone<'a> {
    fn clone_box(&self) -> Box<dyn QueryBuilder<'a> + 'a>;
}

impl<'a, T> QueryBuilderClone<'a> for T
where
    T: 'a + QueryBuilder<'a> + Clone,
{
    fn clone_box(&self) -> Box<dyn QueryBuilder<'a> + 'a> {
        Box::new(self.clone())
    }
}

impl<'a> Clone for Box<dyn QueryBuilder<'a> + 'a> {
    fn clone(&self) -> Box<dyn QueryBuilder<'a> + 'a> {
        self.clone_box()
    }
}
