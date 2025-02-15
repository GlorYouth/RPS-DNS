#[cfg(not(feature = "result_error"))]
use std::marker::PhantomData;

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
