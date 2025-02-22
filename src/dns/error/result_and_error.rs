use crate::dns::utils::RefWrapper;
#[cfg(not(feature = "result_error"))]
use std::marker::PhantomData;

pub trait Wrapper {
    type Index: Default;
    fn get_index(&self) -> &Self::Index;

    fn into_index(self) -> Self::Index;

    fn from_index(index: Self::Index) -> Self;
}

impl<T> Wrapper for Option<T> {
    type Index = Option<T>;
    #[inline(always)]
    fn get_index(&self) -> &Self::Index {
        &self
    }

    #[inline(always)]
    fn into_index(self) -> Self::Index {
        self
    }

    #[inline(always)]
    fn from_index(index: Self::Index) -> Self {
        index
    }
}

impl<T> Wrapper for Vec<T> {
    type Index = Vec<T>;

    #[inline(always)]
    fn get_index(&self) -> &Self::Index {
        &self
    }

    #[inline(always)]
    fn into_index(self) -> Self::Index {
        self
    }

    #[inline(always)]
    fn from_index(index: Self::Index) -> Self {
        index
    }
}

#[derive(Debug)]
pub struct ResultAndError<W: Wrapper, E = ()> {
    #[cfg(feature = "result_error")]
    result: Result<W, E>,
    #[cfg(not(feature = "result_error"))]
    result: W,
    #[cfg(not(feature = "result_error"))]
    _phantom: PhantomData<E>, // 使用 PhantomData 来保留 E 泛型参数
}

impl<W: Wrapper, E> ResultAndError<W, E> {
    #[inline]
    pub fn from_result(result: W) -> ResultAndError<W, E> {
        Self {
            #[cfg(feature = "result_error")]
            result: Ok(result),
            #[cfg(not(feature = "result_error"))]
            result,
            #[cfg(not(feature = "result_error"))]
            _phantom: Default::default(),
        }
    }
    #[cfg(feature = "result_error")]
    #[inline]
    pub fn from_error(error: E) -> Self {
        Self { result: Err(error) }
    }

    pub fn result(&self) -> RefWrapper<W::Index> {
        #[cfg(feature = "result_error")]
        if let Ok(result) = &self.result {
            RefWrapper::from_ref(result.get_index())
        } else {
            Default::default()
        }
        #[cfg(not(feature = "result_error"))]
        RefWrapper::from_ref(self.result.get_index())
    }

    pub fn into_result(self) -> W::Index {
        #[cfg(feature = "result_error")]
        if let Ok(result) = self.result {
            result.into_index()
        } else {
            Default::default()
        }
        #[cfg(not(feature = "result_error"))]
        self.result.into_index()
    }

    #[cfg(feature = "result_error")]
    #[inline]
    pub fn error(&self) -> Option<&E> {
        self.result.as_ref().err()
    }

    #[cfg(feature = "result_error")]
    #[inline]
    pub fn into_error(self) -> Option<E> {
        self.result.err()
    }

    #[cfg(feature = "result_error")]
    #[inline]
    pub fn index(&self) -> &Result<W, E> {
        &self.result
    }

    #[cfg(not(feature = "result_error"))]
    #[inline]
    pub fn get_index(&self) -> &W {
        &self.result
    }
    #[cfg(feature = "result_error")]
    #[inline]
    pub fn into_index(self) -> Result<W, E> {
        self.result
    }

    #[cfg(not(feature = "result_error"))]
    #[inline]
    pub fn into_index(self) -> W::Index {
        self.result.into_index()
    }
}

#[cfg(feature = "result_error")]
impl<W: Wrapper, E> From<Result<W, E>> for ResultAndError<W, E> {
    fn from(result: Result<W, E>) -> Self {
        Self {
            result,
        }
    }
}