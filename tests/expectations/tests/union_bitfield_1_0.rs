/* automatically generated by rust-bindgen */


#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]


#[repr(C)]
pub struct __BindgenUnionField<T>(::std::marker::PhantomData<T>);
impl<T> __BindgenUnionField<T> {
    #[inline]
    pub fn new() -> Self {
        __BindgenUnionField(::std::marker::PhantomData)
    }
    #[inline]
    pub unsafe fn as_ref(&self) -> &T {
        ::std::mem::transmute(self)
    }
    #[inline]
    pub unsafe fn as_mut(&mut self) -> &mut T {
        ::std::mem::transmute(self)
    }
}
impl<T> ::std::default::Default for __BindgenUnionField<T> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}
impl<T> ::std::clone::Clone for __BindgenUnionField<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self::new()
    }
}
impl<T> ::std::marker::Copy for __BindgenUnionField<T> {}
impl<T> ::std::fmt::Debug for __BindgenUnionField<T> {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        fmt.write_str("__BindgenUnionField")
    }
}
impl<T> ::std::hash::Hash for __BindgenUnionField<T> {
    fn hash<H: ::std::hash::Hasher>(&self, _state: &mut H) {}
}
impl<T> ::std::cmp::PartialEq for __BindgenUnionField<T> {
    fn eq(&self, _other: &__BindgenUnionField<T>) -> bool {
        true
    }
}
impl<T> ::std::cmp::Eq for __BindgenUnionField<T> {}
#[repr(C)]
#[derive(Debug, Default, Copy, Hash, PartialEq, Eq)]
pub struct U4 {
    pub _bitfield_1: __BindgenUnionField<u8>,
    pub bindgen_union_field: u32,
}
#[test]
fn bindgen_test_layout_U4() {
    assert_eq!(
        ::std::mem::size_of::<U4>(),
        4usize,
        concat!("Size of: ", stringify!(U4))
    );
    assert_eq!(
        ::std::mem::align_of::<U4>(),
        4usize,
        concat!("Alignment of ", stringify!(U4))
    );
}
impl Clone for U4 {
    fn clone(&self) -> Self {
        *self
    }
}
impl U4 {
    #[inline]
    pub fn derp(&self) -> ::std::os::raw::c_uint {
        let mut unit_field_val: u8 = unsafe { ::std::mem::uninitialized() };
        unsafe {
            ::std::ptr::copy_nonoverlapping(
                &self._bitfield_1 as *const _ as *const u8,
                &mut unit_field_val as *mut u8 as *mut u8,
                ::std::mem::size_of::<u8>(),
            )
        };
        let mask = 0x1 as u8;
        let val = (unit_field_val & mask) >> 0usize;
        unsafe { ::std::mem::transmute(val as u32) }
    }
    #[inline]
    pub fn set_derp(&mut self, val: ::std::os::raw::c_uint) {
        let mask = 0x1 as u8;
        let val = val as u32 as u8;
        let mut unit_field_val: u8 = unsafe { ::std::mem::uninitialized() };
        unsafe {
            ::std::ptr::copy_nonoverlapping(
                &self._bitfield_1 as *const _ as *const u8,
                &mut unit_field_val as *mut u8 as *mut u8,
                ::std::mem::size_of::<u8>(),
            )
        };
        unit_field_val &= !mask;
        unit_field_val |= (val << 0usize) & mask;
        unsafe {
            ::std::ptr::copy_nonoverlapping(
                &unit_field_val as *const _ as *const u8,
                &mut self._bitfield_1 as *mut _ as *mut u8,
                ::std::mem::size_of::<u8>(),
            );
        }
    }
    #[inline]
    pub fn new_bitfield_1(derp: ::std::os::raw::c_uint) -> u8 {
        (0 | ((derp as u32 as u8) << 0usize) & (0x1 as u8))
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Hash, PartialEq, Eq)]
pub struct B {
    pub _bitfield_1: __BindgenUnionField<u32>,
    pub bindgen_union_field: u32,
}
#[test]
fn bindgen_test_layout_B() {
    assert_eq!(
        ::std::mem::size_of::<B>(),
        4usize,
        concat!("Size of: ", stringify!(B))
    );
    assert_eq!(
        ::std::mem::align_of::<B>(),
        4usize,
        concat!("Alignment of ", stringify!(B))
    );
}
impl Clone for B {
    fn clone(&self) -> Self {
        *self
    }
}
impl B {
    #[inline]
    pub fn foo(&self) -> ::std::os::raw::c_uint {
        let mut unit_field_val: u32 = unsafe { ::std::mem::uninitialized() };
        unsafe {
            ::std::ptr::copy_nonoverlapping(
                &self._bitfield_1 as *const _ as *const u8,
                &mut unit_field_val as *mut u32 as *mut u8,
                ::std::mem::size_of::<u32>(),
            )
        };
        let mask = 0x7fffffff as u32;
        let val = (unit_field_val & mask) >> 0usize;
        unsafe { ::std::mem::transmute(val as u32) }
    }
    #[inline]
    pub fn set_foo(&mut self, val: ::std::os::raw::c_uint) {
        let mask = 0x7fffffff as u32;
        let val = val as u32 as u32;
        let mut unit_field_val: u32 = unsafe { ::std::mem::uninitialized() };
        unsafe {
            ::std::ptr::copy_nonoverlapping(
                &self._bitfield_1 as *const _ as *const u8,
                &mut unit_field_val as *mut u32 as *mut u8,
                ::std::mem::size_of::<u32>(),
            )
        };
        unit_field_val &= !mask;
        unit_field_val |= (val << 0usize) & mask;
        unsafe {
            ::std::ptr::copy_nonoverlapping(
                &unit_field_val as *const _ as *const u8,
                &mut self._bitfield_1 as *mut _ as *mut u8,
                ::std::mem::size_of::<u32>(),
            );
        }
    }
    #[inline]
    pub fn bar(&self) -> ::std::os::raw::c_uchar {
        let mut unit_field_val: u32 = unsafe { ::std::mem::uninitialized() };
        unsafe {
            ::std::ptr::copy_nonoverlapping(
                &self._bitfield_1 as *const _ as *const u8,
                &mut unit_field_val as *mut u32 as *mut u8,
                ::std::mem::size_of::<u32>(),
            )
        };
        let mask = 0x80000000 as u32;
        let val = (unit_field_val & mask) >> 31usize;
        unsafe { ::std::mem::transmute(val as u8) }
    }
    #[inline]
    pub fn set_bar(&mut self, val: ::std::os::raw::c_uchar) {
        let mask = 0x80000000 as u32;
        let val = val as u8 as u32;
        let mut unit_field_val: u32 = unsafe { ::std::mem::uninitialized() };
        unsafe {
            ::std::ptr::copy_nonoverlapping(
                &self._bitfield_1 as *const _ as *const u8,
                &mut unit_field_val as *mut u32 as *mut u8,
                ::std::mem::size_of::<u32>(),
            )
        };
        unit_field_val &= !mask;
        unit_field_val |= (val << 31usize) & mask;
        unsafe {
            ::std::ptr::copy_nonoverlapping(
                &unit_field_val as *const _ as *const u8,
                &mut self._bitfield_1 as *mut _ as *mut u8,
                ::std::mem::size_of::<u32>(),
            );
        }
    }
    #[inline]
    pub fn new_bitfield_1(foo: ::std::os::raw::c_uint, bar: ::std::os::raw::c_uchar) -> u32 {
        ((0 | ((foo as u32 as u32) << 0usize) & (0x7fffffff as u32))
            | ((bar as u8 as u32) << 31usize) & (0x80000000 as u32))
    }
}
#[repr(C)]
#[derive(Copy)]
pub struct HasBigBitfield {
    pub _bitfield_1: __BindgenUnionField<[u8; 16usize]>,
    pub bindgen_union_field: [u8; 16usize],
}
#[test]
fn bindgen_test_layout_HasBigBitfield() {
    assert_eq!(
        ::std::mem::size_of::<HasBigBitfield>(),
        16usize,
        concat!("Size of: ", stringify!(HasBigBitfield))
    );
}
impl Clone for HasBigBitfield {
    fn clone(&self) -> Self {
        *self
    }
}
impl Default for HasBigBitfield {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
