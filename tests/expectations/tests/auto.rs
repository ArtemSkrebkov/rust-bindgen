/* automatically generated by rust-bindgen */


#![allow(non_snake_case)]


#[repr(C)]
#[derive(Debug, Default, Copy)]
pub struct Foo {
    pub _address: u8,
}
pub const Foo_kFoo: bool = true;
#[test]
fn bindgen_test_layout_Foo() {
    assert_eq!(::std::mem::size_of::<Foo>() , 1usize , concat ! (
               "Size of: " , stringify ! ( Foo ) ));
    assert_eq! (::std::mem::align_of::<Foo>() , 1usize , concat ! (
                "Alignment of " , stringify ! ( Foo ) ));
}
impl Clone for Foo {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Bar {
    pub _address: u8,
}
extern "C" {
    #[link_name = "_Z5Test2v"]
    pub fn Test2() -> ::std::os::raw::c_uint;
}
