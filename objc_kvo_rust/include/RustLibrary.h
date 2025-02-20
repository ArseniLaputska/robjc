#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

extern void *class_getInstanceVariable(const void *cls, const char *name);

extern void *object_getIvar(void *obj, void *ivar);

extern intptr_t ivar_getOffset(void *ivar);

/**
 * Регистрирует класс MyClass, если он ещё не зарегистрирован.
 */
void registerMyClass(void);

/**
 * Создаёт новый экземпляр MyClass и возвращает *mut AnyObject
 */
AnyObject *createMyClass(void);

/**
 * Запускает фоновый поток, который каждую секунду увеличивает значение `value`.
 */
void startUpdatingMyClass(AnyObject *obj);
