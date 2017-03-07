/* automatically generated by rust-bindgen */


#![allow(non_snake_case)]


#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Base {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Derived {
    pub b: bool,
}
#[test]
fn __bindgen_test_layout__bindgen_ty_id_20_instantiation_14() {
    assert_eq!(::std::mem::size_of::<[u32; 2usize]>() , 8usize , concat ! (
               "Size of template specialization: " , stringify ! (
               [u32; 2usize] ) ));
    assert_eq!(::std::mem::align_of::<[u32; 2usize]>() , 4usize , concat ! (
               "Alignment of template specialization: " , stringify ! (
               [u32; 2usize] ) ));
}
#[repr(C)]
#[derive(Debug, Default, Copy)]
pub struct Usage {
    pub _address: u8,
}
extern "C" {
    #[link_name = "_ZN5Usage13static_memberE"]
    pub static mut Usage_static_member: [u32; 2usize];
}
#[test]
fn bindgen_test_layout_Usage() {
    assert_eq!(::std::mem::size_of::<Usage>() , 1usize , concat ! (
               "Size of: " , stringify ! ( Usage ) ));
    assert_eq! (::std::mem::align_of::<Usage>() , 1usize , concat ! (
                "Alignment of " , stringify ! ( Usage ) ));
}
impl Clone for Usage {
    fn clone(&self) -> Self { *self }
}
