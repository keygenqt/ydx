//
//  ContentView.swift
//  Demo Rust
//
//  Created by Zarubin Vitaliy on 14.02.2026.
//

import SwiftUI
import Shared

struct ContentView: View {
    
    @State private var state = "Run performance test"
    
    var body: some View {
        VStack(spacing: 24) {
            Text("iOS + Rust")
                .font(.largeTitle)
                .fontWeight(.bold)
            HStack(spacing: 20) {
                Image(.ios)
                    .renderingMode(.template)
                    .resizable()
                    .scaledToFit()
                    .frame(width: 80, height: 80)
                    .foregroundColor(Color("AppleColor"))
                Image(.rust)
                    .renderingMode(.template)
                    .resizable()
                    .scaledToFit()
                    .frame(width: 80, height: 80)
                    .foregroundColor(Color("RustColor"))
            }
            Button(state) {
                Task {
                    let value = await shared_performance_test_async()
                    state = "The test was performed: \(value)ms"
                }
                state = "Loading..."
            }
            .disabled(state == "Loading...")
            .padding(.top, 20)
            Text("Click to test the performance of the Rust library.")
                .multilineTextAlignment(.center)
                .padding()
        }
        .padding()
    }
}

func shared_performance_test_async() async -> UInt64 {
    return await withUnsafeContinuation { continuation in
        DispatchQueue.global().async {
            let result = shared_performance_test()
            continuation.resume(returning: result)
        }
    }
}

#Preview {
    ContentView()
}
