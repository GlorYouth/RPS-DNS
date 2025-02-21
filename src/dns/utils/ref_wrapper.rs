use std::mem::MaybeUninit;
use std::pin::Pin;
use std::ptr::NonNull;

pub struct RefWrapper<T> {
    _val: Pin<T>,
    _ref: NonNull<T>,
}

impl<T> RefWrapper<T> {
    pub fn from_ref(_ref: &T) -> Self {
        unsafe {
            Self {
                _val: Pin::new(MaybeUninit::uninit().assume_init()),
                _ref: NonNull::from(_ref),
            }
        }
    }
    
    pub fn from_val(_val: T) -> Self {
        let mut result = Self {
            _val: Pin::new(_val),
            _ref: NonNull::dangling(),
        };
        result._ref = NonNull::from(&mut result._val);
        result
    }
    
    pub fn as_ref(&self) -> &T {
        unsafe { self._ref.as_ref() }
    }
}

#[cfg(test)]
mod tests {
    use crate::dns::utils::RefWrapper;

    #[test]
    fn test_wrapper() {
        {
            let wrapper = RefWrapper::from_ref(&1);
            assert_eq!(wrapper.as_ref(), &1);
        }
        {
            let wrapper = RefWrapper::from_val(1);
            assert_eq!(wrapper.as_ref(), &1);
        }
    }
}