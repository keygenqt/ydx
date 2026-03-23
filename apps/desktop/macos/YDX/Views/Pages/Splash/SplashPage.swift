import SwiftUI
import Factory

struct SplashPage: View {
    @State private var viewModel = Container.shared.splashViewModel()
    
    var body: some View {
        VStack(spacing: 20) {
            Image("Ydx")
                .resizable()
                .frame(width: 100, height: 100)
            
            Text(viewModel.currentMessage)
                .font(.title2)
                .fontWeight(.semibold)
                .multilineTextAlignment(.center)
            
            ProgressView()
                .progressViewStyle(.linear)
                .frame(width: 300)
            
            Text("Please wait...")
                .font(.caption)
                .foregroundColor(.secondary)
        }
        .frame(width: 400, height: 300)
        .task {
            await viewModel.checkAuthState()
        }
    }
}
