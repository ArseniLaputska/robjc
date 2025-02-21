//
//  MyClassWrapperView.swift
//  Rust_objc
//
//  Created by Arseni Laputska on 21.02.25.
//


import UIKit

// Обёртка, которая содержит ссылку на MyClass и отображает его значение.
class MyClassWrapperView: UIView {
    var myClass: MyClass! {
        didSet {
            // При смене объекта обновляем наблюдение.
            observation?.invalidate()
            observation = myClass.observe(\MyClass.value, options: [.initial, .new]) { [weak self] (object, change) in
                DispatchQueue.main.async {
                    self?.updateUI()
                }
            }
        }
    }
    
    private var observation: NSKeyValueObservation?
    private let valueLabel: UILabel = {
        let label = UILabel()
        label.font = UIFont.systemFont(ofSize: 24)
        label.textAlignment = .center
        label.textColor = .black
        label.translatesAutoresizingMaskIntoConstraints = false
        return label
    }()
    
    override init(frame: CGRect) {
        super.init(frame: frame)
        setupView()
    }
    
    required init?(coder: NSCoder) {
        super.init(coder: coder)
        setupView()
    }
    
    deinit {
        observation?.invalidate()
    }
    
    private func setupView() {
        backgroundColor = .white
        addSubview(valueLabel)
        NSLayoutConstraint.activate([
            valueLabel.centerXAnchor.constraint(equalTo: self.centerXAnchor),
            valueLabel.centerYAnchor.constraint(equalTo: self.centerYAnchor)
        ])
    }
    
    private func updateUI() {
        if let myClass = myClass {
            valueLabel.text = "Value: \(myClass.value)"
        } else {
            valueLabel.text = "Value: –"
        }
    }
}
