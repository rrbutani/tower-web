use super::{BufStream, SizeHint};
use crate::util::tuple::Either2 as Either;

use futures::Poll;

/// A buf stream that sequences two buf streams together.
///
/// `Chain` values are produced by the `chain` function on `BufStream`.
#[derive(Debug)]
pub struct Chain<T, U> {
    left: Option<T>,
    right: U,
}

impl<T, U> Chain<T, U> {
    pub(crate) fn new(left: T, right: U) -> Chain<T, U> {
        Chain {
            left: Some(left),
            right,
        }
    }
}

impl<T, U> BufStream for Chain<T, U>
where
    T: BufStream,
    U: BufStream<Error = T::Error>,
{
    type Item = Either<T::Item, U::Item>;
    type Error = T::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        if let Some(ref mut stream) = self.left {
            let res = try_ready!(stream.poll());

            if res.is_some() {
                return Ok(res.map(Either::A).into());
            }
        }

        self.left = None;

        let res = try_ready!(self.right.poll());
        Ok(res.map(Either::B).into())
    }

    fn size_hint(&self) -> SizeHint {
        // TODO: Implement
        SizeHint::default()
    }
}
