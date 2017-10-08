use core::intrinsics::{volatile_load, volatile_store};

#[derive(Copy, Clone)]
#[repr(C)]
pub struct RWReg<T> {
  pub value: T,
}
/* nakonec to staci takto jednoduse */
impl<T> RWReg<T> {
  /// Create a cell with initial value.
  pub fn new(value: T) -> RWReg<T> {
    RWReg {
      value: value,
    }
  }

  /// Get register value.
  #[inline]
  pub fn get(&self) -> T {
    unsafe {
      volatile_load(&self.value)
    }
  }

  /// Set register value.
  #[inline]
  pub fn set(&self, value: T) {
    unsafe {
      volatile_store(&self.value as *const T as *mut T, value)
    }
  }
  // takhle divne se to v rustu pise
  #[inline]
  pub fn modify<F>(&self, f:F) where F: FnOnce(T)->T {
    self.set(f(self.get()));
  }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ROReg<T> {
  pub value: T,
}

impl<T> ROReg<T> {
  /// Create a cell with initial value.
  pub fn new(value: T) -> RWReg<T> {
    RWReg {
      value: value,
    }
  }

  /// Get register value.
  #[inline]
  pub fn get(&self) -> T {
    unsafe {
      volatile_load(&self.value)
    }
  }
}

