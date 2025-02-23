#![feature(
    no_core,
    lang_items,
    intrinsics,
    unboxed_closures,
    extern_types,
    decl_macro,
    crablangc_attrs,
    transparent_unions,
    auto_traits,
    thread_local
)]
#![no_core]
#![allow(dead_code)]

#[lang = "sized"]
pub trait Sized {}

#[lang = "destruct"]
pub trait Destruct {}

#[lang = "tuple_trait"]
pub trait Tuple {}

#[lang = "unsize"]
pub trait Unsize<T: ?Sized> {}

#[lang = "coerce_unsized"]
pub trait CoerceUnsized<T> {}

impl<'a, 'b: 'a, T: ?Sized + Unsize<U>, U: ?Sized> CoerceUnsized<&'a U> for &'b T {}
impl<'a, T: ?Sized + Unsize<U>, U: ?Sized> CoerceUnsized<&'a mut U> for &'a mut T {}
impl<T: ?Sized + Unsize<U>, U: ?Sized> CoerceUnsized<*const U> for *const T {}
impl<T: ?Sized + Unsize<U>, U: ?Sized> CoerceUnsized<*mut U> for *mut T {}

#[lang = "dispatch_from_dyn"]
pub trait DispatchFromDyn<T> {}

// &T -> &U
impl<'a, T: ?Sized+Unsize<U>, U: ?Sized> DispatchFromDyn<&'a U> for &'a T {}
// &mut T -> &mut U
impl<'a, T: ?Sized+Unsize<U>, U: ?Sized> DispatchFromDyn<&'a mut U> for &'a mut T {}
// *const T -> *const U
impl<T: ?Sized+Unsize<U>, U: ?Sized> DispatchFromDyn<*const U> for *const T {}
// *mut T -> *mut U
impl<T: ?Sized+Unsize<U>, U: ?Sized> DispatchFromDyn<*mut U> for *mut T {}
impl<T: ?Sized + Unsize<U>, U: ?Sized> DispatchFromDyn<Box<U>> for Box<T> {}

#[lang = "receiver"]
pub trait Receiver {}

impl<T: ?Sized> Receiver for &T {}
impl<T: ?Sized> Receiver for &mut T {}
impl<T: ?Sized> Receiver for Box<T> {}

#[lang = "copy"]
pub unsafe trait Copy {}

unsafe impl Copy for bool {}
unsafe impl Copy for u8 {}
unsafe impl Copy for u16 {}
unsafe impl Copy for u32 {}
unsafe impl Copy for u64 {}
unsafe impl Copy for u128 {}
unsafe impl Copy for usize {}
unsafe impl Copy for i8 {}
unsafe impl Copy for i16 {}
unsafe impl Copy for i32 {}
unsafe impl Copy for isize {}
unsafe impl Copy for f32 {}
unsafe impl Copy for f64 {}
unsafe impl Copy for char {}
unsafe impl<'a, T: ?Sized> Copy for &'a T {}
unsafe impl<T: ?Sized> Copy for *const T {}
unsafe impl<T: ?Sized> Copy for *mut T {}
unsafe impl<T: Copy> Copy for Option<T> {}

#[lang = "sync"]
pub unsafe trait Sync {}

unsafe impl Sync for bool {}
unsafe impl Sync for u8 {}
unsafe impl Sync for u16 {}
unsafe impl Sync for u32 {}
unsafe impl Sync for u64 {}
unsafe impl Sync for usize {}
unsafe impl Sync for i8 {}
unsafe impl Sync for i16 {}
unsafe impl Sync for i32 {}
unsafe impl Sync for isize {}
unsafe impl Sync for char {}
unsafe impl<'a, T: ?Sized> Sync for &'a T {}
unsafe impl Sync for [u8; 16] {}

#[lang = "freeze"]
unsafe auto trait Freeze {}

unsafe impl<T: ?Sized> Freeze for PhantomData<T> {}
unsafe impl<T: ?Sized> Freeze for *const T {}
unsafe impl<T: ?Sized> Freeze for *mut T {}
unsafe impl<T: ?Sized> Freeze for &T {}
unsafe impl<T: ?Sized> Freeze for &mut T {}

#[lang = "structural_peq"]
pub trait StructuralPartialEq {}

#[lang = "structural_teq"]
pub trait StructuralEq {}

#[lang = "not"]
pub trait Not {
    type Output;

    fn not(self) -> Self::Output;
}

impl Not for bool {
    type Output = bool;

    fn not(self) -> bool {
        !self
    }
}

#[lang = "mul"]
pub trait Mul<RHS = Self> {
    type Output;

    #[must_use]
    fn mul(self, rhs: RHS) -> Self::Output;
}

impl Mul for u8 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        self * rhs
    }
}

impl Mul for usize {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        self * rhs
    }
}

#[lang = "add"]
pub trait Add<RHS = Self> {
    type Output;

    fn add(self, rhs: RHS) -> Self::Output;
}

impl Add for u8 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        self + rhs
    }
}

impl Add for i8 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        self + rhs
    }
}

impl Add for usize {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        self + rhs
    }
}

#[lang = "sub"]
pub trait Sub<RHS = Self> {
    type Output;

    fn sub(self, rhs: RHS) -> Self::Output;
}

impl Sub for usize {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        self - rhs
    }
}

impl Sub for u8 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        self - rhs
    }
}

impl Sub for i8 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        self - rhs
    }
}

impl Sub for i16 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        self - rhs
    }
}

#[lang = "rem"]
pub trait Rem<RHS = Self> {
    type Output;

    fn rem(self, rhs: RHS) -> Self::Output;
}

impl Rem for usize {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self {
        self % rhs
    }
}

#[lang = "bitor"]
pub trait BitOr<RHS = Self> {
    type Output;

    #[must_use]
    fn bitor(self, rhs: RHS) -> Self::Output;
}

impl BitOr for bool {
    type Output = bool;

    fn bitor(self, rhs: bool) -> bool {
        self | rhs
    }
}

impl<'a> BitOr<bool> for &'a bool {
    type Output = bool;

    fn bitor(self, rhs: bool) -> bool {
        *self | rhs
    }
}

#[lang = "eq"]
pub trait PartialEq<Rhs: ?Sized = Self> {
    fn eq(&self, other: &Rhs) -> bool;
    fn ne(&self, other: &Rhs) -> bool;
}

impl PartialEq for u8 {
    fn eq(&self, other: &u8) -> bool {
        (*self) == (*other)
    }
    fn ne(&self, other: &u8) -> bool {
        (*self) != (*other)
    }
}

impl PartialEq for u16 {
    fn eq(&self, other: &u16) -> bool {
        (*self) == (*other)
    }
    fn ne(&self, other: &u16) -> bool {
        (*self) != (*other)
    }
}

impl PartialEq for u32 {
    fn eq(&self, other: &u32) -> bool {
        (*self) == (*other)
    }
    fn ne(&self, other: &u32) -> bool {
        (*self) != (*other)
    }
}


impl PartialEq for u64 {
    fn eq(&self, other: &u64) -> bool {
        (*self) == (*other)
    }
    fn ne(&self, other: &u64) -> bool {
        (*self) != (*other)
    }
}

impl PartialEq for u128 {
    fn eq(&self, other: &u128) -> bool {
        (*self) == (*other)
    }
    fn ne(&self, other: &u128) -> bool {
        (*self) != (*other)
    }
}

impl PartialEq for usize {
    fn eq(&self, other: &usize) -> bool {
        (*self) == (*other)
    }
    fn ne(&self, other: &usize) -> bool {
        (*self) != (*other)
    }
}

impl PartialEq for i8 {
    fn eq(&self, other: &i8) -> bool {
        (*self) == (*other)
    }
    fn ne(&self, other: &i8) -> bool {
        (*self) != (*other)
    }
}

impl PartialEq for i32 {
    fn eq(&self, other: &i32) -> bool {
        (*self) == (*other)
    }
    fn ne(&self, other: &i32) -> bool {
        (*self) != (*other)
    }
}

impl PartialEq for isize {
    fn eq(&self, other: &isize) -> bool {
        (*self) == (*other)
    }
    fn ne(&self, other: &isize) -> bool {
        (*self) != (*other)
    }
}

impl PartialEq for char {
    fn eq(&self, other: &char) -> bool {
        (*self) == (*other)
    }
    fn ne(&self, other: &char) -> bool {
        (*self) != (*other)
    }
}

impl<T: ?Sized> PartialEq for *const T {
    fn eq(&self, other: &*const T) -> bool {
        *self == *other
    }
    fn ne(&self, other: &*const T) -> bool {
        *self != *other
    }
}

impl <T: PartialEq> PartialEq for Option<T> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Some(lhs), Some(rhs)) => *lhs == *rhs,
            (None, None) => true,
            _ => false,
        }
    }

    fn ne(&self, other: &Self) -> bool {
        match (self, other) {
            (Some(lhs), Some(rhs)) => *lhs != *rhs,
            (None, None) => false,
            _ => true,
        }
    }
}

#[lang = "shl"]
pub trait Shl<RHS = Self> {
    type Output;

    #[must_use]
    fn shl(self, rhs: RHS) -> Self::Output;
}

impl Shl for u128 {
    type Output = u128;

    fn shl(self, rhs: u128) -> u128 {
        self << rhs
    }
}

#[lang = "neg"]
pub trait Neg {
    type Output;

    fn neg(self) -> Self::Output;
}

impl Neg for i8 {
    type Output = i8;

    fn neg(self) -> i8 {
        -self
    }
}

impl Neg for i16 {
    type Output = i16;

    fn neg(self) -> i16 {
        self
    }
}

impl Neg for isize {
    type Output = isize;

    fn neg(self) -> isize {
        -self
    }
}

impl Neg for f32 {
    type Output = f32;

    fn neg(self) -> f32 {
        -self
    }
}

pub enum Option<T> {
    Some(T),
    None,
}

pub use Option::*;

#[lang = "phantom_data"]
pub struct PhantomData<T: ?Sized>;

#[lang = "fn_once"]
#[crablangc_paren_sugar]
pub trait FnOnce<Args: Tuple> {
    #[lang = "fn_once_output"]
    type Output;

    extern "crablang-call" fn call_once(self, args: Args) -> Self::Output;
}

#[lang = "fn_mut"]
#[crablangc_paren_sugar]
pub trait FnMut<Args: Tuple>: FnOnce<Args> {
    extern "crablang-call" fn call_mut(&mut self, args: Args) -> Self::Output;
}

#[lang = "panic"]
#[track_caller]
pub fn panic(_msg: &'static str) -> ! {
    unsafe {
        libc::puts("Panicking\n\0" as *const str as *const i8);
        intrinsics::abort();
    }
}

#[lang = "panic_bounds_check"]
#[track_caller]
fn panic_bounds_check(index: usize, len: usize) -> ! {
    unsafe {
        libc::printf("index out of bounds: the len is %d but the index is %d\n\0" as *const str as *const i8, len, index);
        intrinsics::abort();
    }
}

#[lang = "eh_personality"]
fn eh_personality() -> ! {
    loop {}
}

#[lang = "drop_in_place"]
#[allow(unconditional_recursion)]
pub unsafe fn drop_in_place<T: ?Sized>(to_drop: *mut T) {
    // Code here does not matter - this is replaced by the
    // real drop glue by the compiler.
    drop_in_place(to_drop);
}

#[lang = "deref"]
pub trait Deref {
    type Target: ?Sized;

    fn deref(&self) -> &Self::Target;
}

#[repr(transparent)]
#[crablangc_layout_scalar_valid_range_start(1)]
#[crablangc_nonnull_optimization_guaranteed]
pub struct NonNull<T: ?Sized>(pub *const T);

impl<T: ?Sized, U: ?Sized> CoerceUnsized<NonNull<U>> for NonNull<T> where T: Unsize<U> {}
impl<T: ?Sized, U: ?Sized> DispatchFromDyn<NonNull<U>> for NonNull<T> where T: Unsize<U> {}

pub struct Unique<T: ?Sized> {
    pub pointer: NonNull<T>,
    pub _marker: PhantomData<T>,
}

impl<T: ?Sized, U: ?Sized> CoerceUnsized<Unique<U>> for Unique<T> where T: Unsize<U> {}
impl<T: ?Sized, U: ?Sized> DispatchFromDyn<Unique<U>> for Unique<T> where T: Unsize<U> {}

#[lang = "owned_box"]
pub struct Box<T: ?Sized>(Unique<T>, ());

impl<T: ?Sized + Unsize<U>, U: ?Sized> CoerceUnsized<Box<U>> for Box<T> {}

impl<T> Box<T> {
    pub fn new(val: T) -> Box<T> {
        unsafe {
            let size = intrinsics::size_of::<T>();
            let ptr = libc::malloc(size);
            intrinsics::copy(&val as *const T as *const u8, ptr, size);
            Box(Unique { pointer: NonNull(ptr as *const T), _marker: PhantomData }, ())
        }
    }
}

impl<T: ?Sized> Drop for Box<T> {
    fn drop(&mut self) {
        // drop is currently performed by compiler.
    }
}

impl<T: ?Sized> Deref for Box<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &**self
    }
}

#[lang = "exchange_malloc"]
unsafe fn allocate(size: usize, _align: usize) -> *mut u8 {
    libc::malloc(size)
}

#[lang = "box_free"]
unsafe fn box_free<T: ?Sized>(ptr: Unique<T>, _alloc: ()) {
    libc::free(ptr.pointer.0 as *mut u8);
}

#[lang = "drop"]
pub trait Drop {
    fn drop(&mut self);
}

#[lang = "manually_drop"]
#[repr(transparent)]
pub struct ManuallyDrop<T: ?Sized> {
    pub value: T,
}

#[lang = "maybe_uninit"]
#[repr(transparent)]
pub union MaybeUninit<T> {
    pub uninit: (),
    pub value: ManuallyDrop<T>,
}

pub mod intrinsics {
    extern "crablang-intrinsic" {
        #[crablangc_safe_intrinsic]
        pub fn abort() -> !;
        #[crablangc_safe_intrinsic]
        pub fn size_of<T>() -> usize;
        pub fn size_of_val<T: ?::Sized>(val: *const T) -> usize;
        #[crablangc_safe_intrinsic]
        pub fn min_align_of<T>() -> usize;
        pub fn min_align_of_val<T: ?::Sized>(val: *const T) -> usize;
        pub fn copy<T>(src: *const T, dst: *mut T, count: usize);
        pub fn transmute<T, U>(e: T) -> U;
        pub fn ctlz_nonzero<T>(x: T) -> T;
        #[crablangc_safe_intrinsic]
        pub fn needs_drop<T: ?::Sized>() -> bool;
        #[crablangc_safe_intrinsic]
        pub fn bitreverse<T>(x: T) -> T;
        #[crablangc_safe_intrinsic]
        pub fn bswap<T>(x: T) -> T;
        pub fn write_bytes<T>(dst: *mut T, val: u8, count: usize);
    }
}

pub mod libc {
    // With the new Universal CRT, msvc has switched to all the printf functions being inline wrapper
    // functions. legacy_stdio_definitions.lib which provides the printf wrapper functions as normal
    // symbols to link against.
    #[cfg_attr(unix, link(name = "c"))]
    #[cfg_attr(target_env="msvc", link(name="legacy_stdio_definitions"))]
    extern "C" {
        pub fn printf(format: *const i8, ...) -> i32;
    }

    #[cfg_attr(unix, link(name = "c"))]
    #[cfg_attr(target_env = "msvc", link(name = "msvcrt"))]
    extern "C" {
        pub fn puts(s: *const i8) -> i32;
        pub fn malloc(size: usize) -> *mut u8;
        pub fn free(ptr: *mut u8);
        pub fn memcpy(dst: *mut u8, src: *const u8, size: usize);
        pub fn memmove(dst: *mut u8, src: *const u8, size: usize);
        pub fn strncpy(dst: *mut u8, src: *const u8, size: usize);
    }
}

#[lang = "index"]
pub trait Index<Idx: ?Sized> {
    type Output: ?Sized;
    fn index(&self, index: Idx) -> &Self::Output;
}

impl<T> Index<usize> for [T; 3] {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self[index]
    }
}

impl<T> Index<usize> for [T] {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self[index]
    }
}

extern {
    type VaListImpl;
}

#[lang = "va_list"]
#[repr(transparent)]
pub struct VaList<'a>(&'a mut VaListImpl);

#[crablangc_builtin_macro]
#[crablangc_macro_transparency = "semitransparent"]
pub macro stringify($($t:tt)*) { /* compiler built-in */ }

#[crablangc_builtin_macro]
#[crablangc_macro_transparency = "semitransparent"]
pub macro file() { /* compiler built-in */ }

#[crablangc_builtin_macro]
#[crablangc_macro_transparency = "semitransparent"]
pub macro line() { /* compiler built-in */ }

#[crablangc_builtin_macro]
#[crablangc_macro_transparency = "semitransparent"]
pub macro cfg() { /* compiler built-in */ }

#[crablangc_builtin_macro]
#[crablangc_macro_transparency = "semitransparent"]
pub macro global_asm() { /* compiler built-in */ }

pub static A_STATIC: u8 = 42;

#[lang = "panic_location"]
struct PanicLocation {
    file: &'static str,
    line: u32,
    column: u32,
}

#[no_mangle]
#[cfg(not(windows))]
pub fn get_tls() -> u8 {
    #[thread_local]
    static A: u8 = 42;

    A
}
