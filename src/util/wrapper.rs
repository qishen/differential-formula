pub trait HasUniqueForm<U> {
    /// Generate an unique form.
    fn derive_unique_form(&self) -> U;
}

pub trait UniqueForm<U> {
    /// This method provides easy access to the unique form as reference.
    fn unique_form(&self) -> &U;

    /// The item may not be immutable and the unique form needs to be updated 
    /// when it is internally mutated.
    fn update_unique_form(&mut self);
}

pub struct UniqueFormWrapper<U, T> {
    unique_form: U,
    item: T,
}

impl<U, T> UniqueForm<U, T> for UniqueFormWrapper<U> 
where T: HasUniqueForm<U>
{
    fn unique_form(&self) -> &U {
        self.unique_form
    }

    fn update_unique_form(&mut self) {
        let new_form = self.item.derive_unique_form();
        self.unique_form = new_form;
    }
}