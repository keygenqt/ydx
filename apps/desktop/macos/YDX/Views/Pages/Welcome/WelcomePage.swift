import SwiftUI
import Factory

struct WelcomePage: View {
    @State private var viewModel = Container.shared.welcomeViewModel()
    
    var body: some View {
        VStack(spacing: 36) {
            Text("Welcome to YDX!")
                .font(.system(size: 36, weight: .bold))
            
            Image("Ydx")
                .resizable()
                .frame(width: 120, height: 120)
            
            VStack(spacing: 6) {
                Text("✓ Native experience on all 8 platforms")
                    .font(.system(size: 14))
                Text("✓ Lightning fast on any device")
                    .font(.system(size: 14))
                Text("✓ Sync across all devices")
                    .font(.system(size: 14))
            }
            
            HStack(spacing: 20) {
                Button {
                    viewModel.navigateToSignIn()
                } label: {
                    HStack {
                        Spacer(minLength: 0)
                        Text("Sign In")
                        Spacer(minLength: 0)
                    }
                }
                .buttonStyle(.borderedProminent)
                .frame(width: 120)
                
                Button {
                    viewModel.navigateToSignUp()
                } label: {
                    HStack {
                        Spacer(minLength: 0)
                        Text("Sign Up")
                        Spacer(minLength: 0)
                    }
                }
                .buttonStyle(.bordered)
                .frame(width: 120)
            }
        }
        .padding()
    }
}
