
pub enum RefWrapper<'a,T> {
    _Val(T),
    _Ref(&'a T),
}

impl<'a,T> RefWrapper<'a, T> {
    #[inline]
    pub fn from_ref(_ref: &'a T) -> Self {
        Self::_Ref(_ref)
    }
    
    #[inline]
    pub fn from_val(_val: T) -> Self {
         Self::_Val(_val)
    }
    
    #[inline]
    pub fn as_ref(&self) -> &T {
        match self {
            RefWrapper::_Val(v) => v,
            RefWrapper::_Ref(r) => r,
        }
    }
    
    #[inline]
    pub fn into_ref(self) -> Option<&'a T> {
        match self {
            RefWrapper::_Val(_) => None,
            RefWrapper::_Ref(r) => Some(r),
        }
    }
}

impl<'a,T:Default> Default for RefWrapper<'a,T> {
    #[inline(always)]
    fn default() -> Self {
        Self::_Val(Default::default())
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