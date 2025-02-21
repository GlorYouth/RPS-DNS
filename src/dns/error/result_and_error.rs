#[cfg(not(feature = "result_error"))]
use std::marker::PhantomData;
use crate::dns::utils::RefWrapper;

pub mod error_trait {
    pub trait A {}
    pub trait B {}
}

#[derive(Debug)]
pub struct ResultAndError<T, E = ()> {
    #[cfg(feature = "result_error")]
    result: Result<Option<T>, E>,
    #[cfg(not(feature = "result_error"))]
    result: Option<T>,
    #[cfg(not(feature = "result_error"))]
    _phantom: PhantomData<E>, // 使用 PhantomData 来保留 E 泛型参数
}

impl<T, E> ResultAndError<T, E> {
    pub fn from_result(result: Option<T>) -> ResultAndError<T, E> {
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
    pub fn from_error(error: E) -> Self {
        Self { result: Err(error) }
    }

    pub fn result(&self) -> Option<&T> {
        #[cfg(feature = "result_error")]
        if let Ok(result) = &self.result {
            Option::from(result)
        } else {
            None
        }
        #[cfg(not(feature = "result_error"))]
        self.result.as_ref()
    }

    pub fn into_result(self) -> Option<T> {
        #[cfg(feature = "result_error")]
        if let Ok(result) = self.result {
            result
        } else {
            None
        }
        #[cfg(not(feature = "result_error"))]
        self.result
    }

    #[cfg(feature = "result_error")]
    pub fn error(&self) -> Option<&E> {
        if let Err(error) = &self.result {
            Some(error)
        } else {
            None
        }
    }

    #[cfg(feature = "result_error")]
    pub fn into_error(self) -> Option<E> {
        if let Err(error) = self.result {
            Some(error)
        } else {
            None
        }
    }

    #[cfg(feature = "result_error")]
    pub fn get_index(&self) -> &Result<Option<T>, E> {
        &self.result
    }

    #[cfg(not(feature = "result_error"))]
    pub fn get_index(&self) -> Option<&T> {
        self.result.as_ref()
    }
    #[cfg(feature = "result_error")]
    pub fn into_index(self) -> Result<Option<T>, E> {
        self.result
    }

    #[cfg(not(feature = "result_error"))]
    pub fn into_index(self) -> Option<T> {
        self.result
    }
}

impl<T: error_trait::A, E> From<Option<T>> for ResultAndError<T, E> {
    fn from(result: Option<T>) -> Self {
        ResultAndError::from_result(result)
    }
}

#[cfg(feature = "result_error")]
impl<T, E: error_trait::B> From<E> for ResultAndError<T, E> {
    fn from(error: E) -> Self {
        ResultAndError::from_error(error)
    }
}



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
        self
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
pub struct ResultAndError2<W: Wrapper, E = ()> {
    #[cfg(feature = "result_error")]
    result: Result<W, E>,
    #[cfg(not(feature = "result_error"))]
    result: W,
    #[cfg(not(feature = "result_error"))]
    _phantom: PhantomData<E>, // 使用 PhantomData 来保留 E 泛型参数
}

impl<W: Wrapper, E> ResultAndError2<W, E> {
    pub fn from_result(result: W) -> ResultAndError2<W, E> {
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
    pub fn from_error(error: E) -> Self {
        Self { result: Err(error) }
    }

    pub fn result(&self) -> RefWrapper<W::Index> {
        #[cfg(feature = "result_error")]
        if let Ok(result) = &self.result {
            RefWrapper::from_ref(result.get_index())
        } else {
            RefWrapper::from_val(Default::default())
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
    pub fn err(self) -> Option<E> {
        self.result.err()
    }

    #[cfg(feature = "result_error")]
    pub fn into_error(self) -> Option<E> {
        if let Err(error) = self.result {
            Some(error)
        } else {
            None
        }
    }

    #[cfg(feature = "result_error")]
    pub fn index(&self) -> &Result<W, E> {
        &self.result
    }

    #[cfg(not(feature = "result_error"))]
    pub fn get_index(&self) -> &W {
        &self.result
    }
    #[cfg(feature = "result_error")]
    pub fn into_index(self) -> Result<W, E> {
        self.result
    }

    #[cfg(not(feature = "result_error"))]
    pub fn into_index(self) -> W::Index {
        self.result.into_index()
    }
}

impl<W: Wrapper, E> AsRef<ResultAndError2<W,E>> for ResultAndError2<W, E> {
    fn as_ref(&self) -> &ResultAndError2<W,E> {
        self
    }
}