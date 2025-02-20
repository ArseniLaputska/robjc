//
//  ContentView.swift
//  Rust_objc
//
//  Created by Arseni Laputska on 20.02.25.
//

import SwiftUI

//@objc(MyClass)
//@objcMembers
//class MyClass: NSObject {
//    
//}

// // ObservableObject, который будет слушать изменения свойства "value" в MyClass через KVO.
class MyClassObserver: NSObject, ObservableObject {
    @Published var value: Int = 0
    var myClass: AnyObject!
    var observation: NSKeyValueObservation?
    
    override init() {
        super.init()
        // Регистрируем класс, если это ещё не сделано.
        registerMyClass()
        // Создаём экземпляр MyClass, реализованного на Rust.
        myClass = createMyClass() as? AnyObject
        // Запускаем обновление значения из Rust (фоновый поток будет периодически менять "value").
        startUpdatingMyClass(myClass)
        
        // Настраиваем KVO‑наблюдение за свойством "value".
        observation = (myClass as? MyClass)?.observe(\.value, options: [.initial, .new]) { [weak self] (object, change) in
            DispatchQueue.main.async {
                self?.value = Int(object.value)
            }
        }
    }
    
    deinit {
        observation?.invalidate()
    }
}

struct ContentView: View {
    @StateObject var observer = MyClassObserver()

    var body: some View {
        VStack(spacing: 20) {
            Text("Value: \(observer.value)")
                .font(.largeTitle)
            Text("Значение обновляется из Rust")
                .foregroundColor(.gray)
        }
        .padding()
    }
}

#Preview {
    ContentView()
}
