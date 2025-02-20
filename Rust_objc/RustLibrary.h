#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
#include <Foundation/Foundation.h>
#include <objc/runtime.h>

/**
 * Регистрирует класс MyClass, если он ещё не зарегистрирован.
 */
void registerMyClass(void);

/**
 * Создаёт новый экземпляр MyClass и возвращает *mut AnyObject
 */
id createMyClass(void);

/**
 * Запускает фоновый поток, который каждую секунду увеличивает значение `value`.
 */
void startUpdatingMyClass(id obj);
