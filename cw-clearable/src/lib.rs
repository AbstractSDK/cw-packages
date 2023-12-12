/// A type representing a value that can either be cleared or set with a value of type `C`.
/// ```
/// use cosmwasm_std::{StdResult, Response, DepsMut};
/// use cw_storage_plus::Item;
/// use cw_clearable::Clearable;
///
/// const FOO: Item<Option<u32>> = Item::new("foo");
///
/// pub fn update_config(deps: DepsMut, foo: Option<Clearable<u32>>) -> StdResult<Response>{
///     if let Some(foo) = foo {
///         FOO.save(deps.storage, &foo.into());
///     }
///     Ok(Response::new())
/// }
/// ```
#[cosmwasm_schema::cw_serde]
pub enum Clearable<C> {
    /// Clear the current state.
    Clear,
    /// Set state with a value of type `C`.
    Set(C),
}

// Get new value for this item
impl<C> Into<Option<C>> for Clearable<C> {
    fn into(self) -> Option<C> {
        match self {
            Clearable::Clear => None,
            Clearable::Set(val) => Some(val),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use cosmwasm_std::testing::MockStorage;
    use cw_storage_plus::Item;

    const FOO: Item<Option<u32>> = Item::new("foo");

    #[test]
    fn clear() {
        let mut storage = MockStorage::new();
        FOO.save(&mut storage, &Some(0u32)).unwrap();

        let clearable: Clearable<u32> = Clearable::Clear;
        FOO.save(&mut storage, &clearable.into()).unwrap();

        let foo = FOO.load(&storage).unwrap();
        assert_eq!(foo, None);
    }

    #[test]
    fn set() {
        let mut storage = MockStorage::new();
        FOO.save(&mut storage, &Some(0u32)).unwrap();

        let clearable: Clearable<u32> = Clearable::Set(42);
        FOO.save(&mut storage, &clearable.into()).unwrap();

        let foo = FOO.load(&storage).unwrap();
        assert_eq!(foo, Some(42));
    }
}
