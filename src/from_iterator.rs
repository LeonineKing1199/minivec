use crate::MiniVec;

impl<A> core::iter::FromIterator<A> for MiniVec<A> {
  fn from_iter<T>(iter: T) -> Self
  where
    T: IntoIterator<Item = A>,
  {
    let mut v = MiniVec::<A>::new();
    let it = iter.into_iter();
    for x in it {
      v.push(x);
    }
    v
  }
}
