#![allow(non_snake_case)]
#![allow(deprecated)] // Подавляем предупреждения о "runtime" API в objc2 0.6.0

use std::os::raw::c_char;
use std::ptr;
use std::thread;
use std::time::Duration;

use cstr::cstr;
use objc2::declare::ClassBuilder;
use objc2::msg_send;
use objc2::runtime::{AnyClass, AnyObject, Class, Sel};
use objc2::sel;

// -- Внешние C-функции из Objective-C runtime: --
extern "C" {
    fn class_getInstanceVariable(cls: *const core::ffi::c_void, name: *const c_char) -> *mut core::ffi::c_void;
    fn object_getIvar(obj: *mut core::ffi::c_void, ivar: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn ivar_getOffset(ivar: *mut core::ffi::c_void) -> isize;
}

// -- Обёртка, позволяющая передавать *mut AnyObject в поток (thread::spawn):
#[repr(transparent)]
#[derive(Copy, Clone)]
struct SendableAnyObject {
    ptr: *mut AnyObject,
}

unsafe impl Send for SendableAnyObject {}

impl SendableAnyObject {
    /// Установить свойство "value" (метод-сеттер)
    unsafe fn set_value(&self, new_value: i32) {
        let _: () = msg_send![self.ptr, setValue: new_value];
    }
}

// ---------- Геттер и сеттер (динамические методы) ----------
// Геттер: -(int)value
extern "C" fn myclass_value(this: *mut AnyObject, _cmd: Sel) -> i32 {
    unsafe {
        // Получаем класс объекта:
        let cls: &AnyClass = (*this).class();
        let raw_cls = cls as *const AnyClass as *const core::ffi::c_void;

        // Ищем ivar "value"
        let ivar = class_getInstanceVariable(raw_cls, cstr!("value").as_ptr());
        // Считываем текущее значение как i32
        let value_ptr = object_getIvar(this as *mut core::ffi::c_void, ivar) as *mut i32;
        *value_ptr
    }
}

// Сеттер: -(void)setValue:(int)newValue
// (Принимаем i32 в newValue, вызываем KVO‑уведомления.)
extern "C" fn myclass_set_value(this: *mut AnyObject, _cmd: Sel, new_value: i32) {
    unsafe {
        let key = cstr!("value").as_ptr();
        // Вызываем KVO: willChangeValueForKey:
        let _: () = msg_send![this, willChangeValueForKey: key];

        let cls: &AnyClass = (*this).class();
        let raw_cls = cls as *const AnyClass as *const core::ffi::c_void;
        let ivar = class_getInstanceVariable(raw_cls, cstr!("value").as_ptr());
        let offset = ivar_getOffset(ivar);

        // Смещаемся до поля ivar:
        let value_ptr = (this as *mut u8).offset(offset) as *mut i32;
        *value_ptr = new_value;

        // Вызываем KVO: didChangeValueForKey:
        let _: () = msg_send![this, didChangeValueForKey: key];
    }
}

// -------------------------------------------------------------

/// Регистрирует класс MyClass, если он ещё не зарегистрирован.
#[no_mangle]
pub extern "C" fn registerMyClass() {
    // Если уже зарегистрирован, выходим:
    if Class::get(cstr!("MyClass")).is_some() {
        return;
    }
    // Ищем класс NSObject (наш super)
    let super_cls = Class::get(cstr!("NSObject"))
        .expect("NSObject не найден — среда Objective-C не инициализирована?");

    // Создаём билдер для нового класса "MyClass"
    let mut builder = ClassBuilder::new(cstr!("MyClass"), super_cls)
        .expect("Не удалось создать билдер класса MyClass");

    unsafe {
        // Добавляем ivar "value" (i32):
        builder.add_ivar::<i32>(cstr!("value"));

        // Геттер: add_method::<Calleе, F>()
        // 1) Callee == AnyObject
        // 2) F == extern "C" fn(*mut AnyObject, Sel) -> i32
        builder.add_method::<AnyObject, extern "C" fn(*mut AnyObject, Sel) -> i32>(
            sel!(value),
            myclass_value,
        );

        // Сеттер:
        builder.add_method::<AnyObject, extern "C" fn(*mut AnyObject, Sel, i32)>(
            sel!(setValue:),
            myclass_set_value,
        );

        // Регистрация класса
        builder.register();
    }
}

/// Создаёт новый экземпляр MyClass и возвращает *mut AnyObject
#[no_mangle]
pub extern "C" fn createMyClass() -> *mut AnyObject {
    let cls = Class::get(cstr!("MyClass"))
        .expect("Класс MyClass не зарегистрирован!");
    // Создаём объект: [[MyClass alloc] init], упрощённо msg_send![cls, new]
    unsafe { msg_send![cls, new] }
}

/// Запускает фоновый поток, который каждую секунду увеличивает значение `value`.
#[no_mangle]
pub extern "C" fn startUpdatingMyClass(obj: *mut AnyObject) {
    // Оборачиваем объект в структуру, объявленную как `Send`.
    let wrapped = SendableAnyObject { ptr: obj };

    // (Если нужно, можно сделать drop(obj), чтобы
    //  исходный `obj` не остался в области видимости.)

    thread::spawn(move || {
        let mut counter = 0i32;
        loop {
            counter += 1;
            unsafe {
                wrapped.set_value(counter);
            }
            thread::sleep(Duration::from_secs(1));
        }
    });
}
