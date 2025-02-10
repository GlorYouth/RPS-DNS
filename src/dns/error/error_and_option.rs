#[cfg(not(feature = "result_error"))]
use std::marker::PhantomData;

#[derive(Debug)]
pub struct ErrorAndOption<T, E: Default = ()> {
    result: Option<T>,
    #[cfg(feature = "result_error")]
    error: E,
    #[cfg(not(feature = "result_error"))]
    _phantom: PhantomData<E>, // 使用 PhantomData 来保留 E 泛型参数
}

impl<T, E: Default> ErrorAndOption<T, E> {
    pub fn from_result(result: Option<T>) -> ErrorAndOption<T, E> {
        Self {
            result,
            #[cfg(feature = "result_error")]
            error: Default::default(),
            #[cfg(not(feature = "result_error"))]
            _phantom: Default::default(),
        }
    }
    #[cfg(feature = "result_error")]
    pub fn from_error(error: E) -> Self {
        Self {
            result: None,
            #[cfg(feature = "result_error")]
            error,
        }
    }

    pub fn get_result(&self) -> &Option<T> {
        &self.result
    }

    #[cfg(feature = "result_error")]
    pub fn get_error(&self) -> &E {
        &self.error
    }
}
