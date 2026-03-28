import SwiftUI
import Factory

struct SplashPage: View {
    @State private var viewModel = Container.shared.splashViewModel()
    @State private var currentMessageIndex = 0
    
    private let messages = [
        "☕ Brewing coffee",
        "🍵 Steeping tea",
        "🗄️ Tickling the database",
        "🚀 Igniting the reactor",
        "🔐 Inspecting tokens",
        "🧙 Casting authorization spells",
        "🎩 Pulling rabbit out of hat",
        "🦄 Feeding unicorns",
        "⚡ Accelerating neutrons",
        "🧪 Mixing reagents",
        "🌱 Watering the server"
    ]
    
    var body: some View {
        VStack(spacing: 24) {
            Image("Ydx")
                .resizable()
                .frame(width: 120, height: 120)
            
            Text(LocalizedStringKey(messages[currentMessageIndex]))
                .font(.title2)
                .fontWeight(.semibold)
                .multilineTextAlignment(.center)
            
            ProgressView()
                .progressViewStyle(.linear)
                .frame(width: 400)
            
            Text("Please wait...")
                .font(.subheadline)
                .foregroundColor(.secondary)
        }
        .frame(width: 400, height: 300)
        .task {
            Task {
                await rotateMessages()
            }
            await viewModel.checkAuthState()
        }
    }
    
    private func rotateMessages() async {
        while true {
            try? await Task.sleep(nanoseconds: 2_000_000_000)
            withAnimation {
                currentMessageIndex = (currentMessageIndex + 1) % messages.count
            }
        }
    }
}
