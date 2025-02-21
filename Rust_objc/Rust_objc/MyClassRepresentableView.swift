//
//  MyClassRepresentableView.swift
//  Rust_objc
//
//  Created by Arseni Laputska on 21.02.25.
//


import SwiftUI

struct MyClassRepresentableView: UIViewRepresentable {
    let myClass: MyClass

    func makeUIView(context: Context) -> MyClassWrapperView {
        let view = MyClassWrapperView()
        view.myClass = myClass
        return view
    }
    
    func updateUIView(_ uiView: MyClassWrapperView, context: Context) {
        // Обновляем ссылку, если она изменилась (обычно объект остается неизменным).
        if uiView.myClass != myClass {
            uiView.myClass = myClass
        }
    }
}
