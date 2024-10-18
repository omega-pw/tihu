pub struct TupleManyStruct<const N: usize> {}

pub type TupleMany<const N: usize, T> = <TupleManyStruct<N> as TupleManyTrait<T>>::Type;

pub trait TupleManyTrait<T> {
    type Type;
    fn try_from_iter<I>(iter: I) -> Result<Self::Type, ()>
    where
        I: IntoIterator<Item = T>;
}

impl<T> TupleManyTrait<T> for TupleManyStruct<1> {
    type Type = (T,);
    fn try_from_iter<I>(iter: I) -> Result<Self::Type, ()>
    where
        I: IntoIterator<Item = T>,
    {
        let mut iter = iter.into_iter();
        let first = iter.next().ok_or(())?;
        if iter.next().is_none() {
            return Ok((first,));
        } else {
            return Err(());
        }
    }
}

impl<T> TupleManyTrait<T> for TupleManyStruct<2> {
    type Type = (T, T);
    fn try_from_iter<I>(iter: I) -> Result<Self::Type, ()>
    where
        I: IntoIterator<Item = T>,
    {
        let mut iter = iter.into_iter();
        let first = iter.next().ok_or(())?;
        let second = iter.next().ok_or(())?;
        if iter.next().is_none() {
            return Ok((first, second));
        } else {
            return Err(());
        }
    }
}

impl<T> TupleManyTrait<T> for TupleManyStruct<3> {
    type Type = (T, T, T);
    fn try_from_iter<I>(iter: I) -> Result<Self::Type, ()>
    where
        I: IntoIterator<Item = T>,
    {
        let mut iter = iter.into_iter();
        let first = iter.next().ok_or(())?;
        let second = iter.next().ok_or(())?;
        let third = iter.next().ok_or(())?;
        if iter.next().is_none() {
            return Ok((first, second, third));
        } else {
            return Err(());
        }
    }
}

impl<T> TupleManyTrait<T> for TupleManyStruct<4> {
    type Type = (T, T, T, T);
    fn try_from_iter<I>(iter: I) -> Result<Self::Type, ()>
    where
        I: IntoIterator<Item = T>,
    {
        let mut iter = iter.into_iter();
        let first = iter.next().ok_or(())?;
        let second = iter.next().ok_or(())?;
        let third = iter.next().ok_or(())?;
        let fourth = iter.next().ok_or(())?;
        if iter.next().is_none() {
            return Ok((first, second, third, fourth));
        } else {
            return Err(());
        }
    }
}

impl<T> TupleManyTrait<T> for TupleManyStruct<5> {
    type Type = (T, T, T, T, T);
    fn try_from_iter<I>(iter: I) -> Result<Self::Type, ()>
    where
        I: IntoIterator<Item = T>,
    {
        let mut iter = iter.into_iter();
        let first = iter.next().ok_or(())?;
        let second = iter.next().ok_or(())?;
        let third = iter.next().ok_or(())?;
        let fourth = iter.next().ok_or(())?;
        let fifth = iter.next().ok_or(())?;
        if iter.next().is_none() {
            return Ok((first, second, third, fourth, fifth));
        } else {
            return Err(());
        }
    }
}
