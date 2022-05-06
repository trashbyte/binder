use std::cell::UnsafeCell;
use std::ops::{Deref, DerefMut};
use std::ptr::NonNull;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};


#[derive(Debug)]
/// A binding to a [Property]. Allows mutable and immutable access to the value via dereferencing.
/// Exclusive access is enforced by the borrow checker like any other mutable object.
/// `PropertyBinding` automatically unbinds itself when [Drop](std::ops::Drop)ped, and allowing
/// the binding to leave scope and be [Drop](std::ops::Drop)ped is the preferred usage.
///
/// # Safety
///
/// Cannot be cloned, as it is assumed to have an exclusive lock on the property.
/// Thread-safe but not shareable. [Send](core::marker::Send) but not [Sync](core::marker::Sync).
pub struct PropertyBinding<T> {
    value: NonNull<T>,
    lock: Arc<AtomicBool>,
}

impl<T> Deref for PropertyBinding<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { self.value.as_ref() }
    }
}

impl<T> DerefMut for PropertyBinding<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.value.as_mut() }
    }
}

impl<T> Drop for PropertyBinding<T> {
    fn drop(&mut self) {
        if self.lock.swap(false, Ordering::SeqCst) == false {
            panic!("PropertyBinding<{}>: Tried to drop a lock that was already unlocked!", std::any::type_name::<T>())
        }
    }
}

unsafe impl<T: Send> Send for PropertyBinding<T> {}

/// Used to define a bindable property. Designed to use
/// [imgui-rs](https://github.com/imgui-rs/imgui-rs) without drowning in mutable references to
/// everything and constantly fighting with the borrow checker.
///
/// # Usage
///
/// [bind](Property::bind)() can be called on an immutable `&Property` to get a mutable binding.
/// The [PropertyBinding] returned by `bind()` [Deref](std::ops::Deref)s to `&T` and
/// [DerefMut](std::ops::DerefMut)s to `&mut T`. The binding needs to be referenced mutably for
/// [DerefMut](std::ops::DerefMut), so rust's borrow checker enforces exclusive mutable access
/// XOR muliple immutable access to the binding itself. The binding unbinds itself when
/// [Drop](std::ops::Drop)ped, so it is automatically freed when it exits scope.
///
/// ### Example
///
/// ```rust
/// # mod imgui { pub struct Ui; impl Ui { pub fn slider(&self, _: &str, _: &mut f32) {} } }
/// pub struct PropHaver {
///     pub prop: binder::Property<f32>
/// }
/// fn use_prop(p: &PropHaver, ui: &imgui::Ui) {
///     ui.slider("wow what a cool slider", &mut p.prop.bind());
/// }
/// ```
///
/// # Safety
///
/// `Property` owns its value and maintains its own invariants over that value. Properties cannot
/// be bound more than once at the same time. The thread-safe [AtomicBool](std::sync::AtomicBool)
/// is used to synchronize access to the binding, so it should be fully thread-safe as well.
///
/// Properties CANNOT be cloned to get more references to the same value. You can use
/// [Rc<Property>](std::rc::Rc) or [Arc<Property>](std::sync::Arc) for that.
///
/// # Panic
///
/// [bind](Property::bind)() will panic if called on a `Property` that's already been bound
/// elsewhere. Use [try_bind](Property::try_bind)`() -> Result` for a non-panicking version.
#[derive(Debug)]
pub struct Property<T> {
    property: UnsafeCell<T>,
    mut_lock: Arc<AtomicBool>
}

impl<T> Property<T> {
    /// Creates a new `Property` that owns the given value.
    pub fn new(value: T) -> Self {
        Property {
            property: UnsafeCell::new(value),
            mut_lock: Arc::new(AtomicBool::new(false))
        }
    }
}

impl<T> Property<T> {
    /// Attempts to bind the property. This will panic if the property is already bound!
    pub fn bind(&self) -> PropertyBinding<T> {
        let was_locked = self.mut_lock.swap(true, Ordering::SeqCst);
        if was_locked {
            panic!("PropertyBinding<{}>: Tried to bind a property that was already bound!", std::any::type_name::<T>());
        }
        PropertyBinding {
            value: NonNull::new(self.property.get()).unwrap(),
            lock: self.mut_lock.clone()
        }
    }

    /// Safer alternative to [bind](Property::bind). Returns
    /// [Ok](core::result::Result::Ok)`(`[PropertyBinding]`<T>)` upon successful binding and
    /// [Err(())](core::result::Result::Err) if the `Property` was already bound.
    pub fn try_bind(&self) -> Result<PropertyBinding<T>, ()> {
        let was_locked = self.mut_lock.swap(true, Ordering::SeqCst);
        if was_locked {
            Err(())
        }
        else {
            Ok(PropertyBinding {
                value: NonNull::new(self.property.get()).unwrap(),
                lock: self.mut_lock.clone()
            })
        }
    }
}

unsafe impl<T: Send> Send for Property<T> {}
unsafe impl<T: Send + Sync> Sync for Property<T> {}


// hack to run compile_fail doctests
#[cfg(doc)]
#[doc(hidden)]
#[path = "../tests/tests.rs"]
mod tests;
