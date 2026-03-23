import SwiftUI
import Factory

struct WelcomePage: View {
    @State private var viewModel = Container.shared.welcomeViewModel()
    
    var body: some View {
        VStack(spacing: 20) {
            Image(systemName: "hand.wave.fill")
                .font(.system(size: 60))
                .foregroundColor(.accentColor)
            
            Text("Welcome to YDX")
                .font(.title)
                .fontWeight(.bold)
            
            Text("Please sign in or create an account")
                .font(.body)
                .foregroundColor(.secondary)
            
            VStack(spacing: 12) {
                Button("Sign In") {
                    viewModel.navigateToSignIn()
                }
                .buttonStyle(.borderedProminent)
                .controlSize(.large)
                
                Button("Create Account") {
                    viewModel.navigateToSignUp()
                }
                .buttonStyle(.bordered)
                .controlSize(.large)
            }
            .padding(.horizontal, 40)
            .padding(.top, 20)
        }
        .padding()
        .frame(width: 500, height: 400)
    }
}
