//
//  MyClass.m
//  Rust_objc
//
//  Created by Arseni Laputska on 20.02.25.
//

#import <Foundation/Foundation.h>
#import "MyClass.h"

// Объявляем функцию из Rust‑библиотеки.
extern void registerMyClass(void);

@implementation MyClass

+ (void)load {
    // При загрузке класса вызываем функцию регистрации.
    registerMyClass();
}

@end
