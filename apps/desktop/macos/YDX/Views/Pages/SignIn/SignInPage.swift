import SwiftUI
import Factory

struct SignInPage: View {
    @State private var viewModel = Container.shared.signInViewModel()
    
    var body: some View {
        VStack(spacing: 20) {
            BackButton()
                .frame(maxWidth: .infinity, alignment: .leading)
                .padding(.horizontal)
            
            VStack(spacing: 8) {
                Image(systemName: "person.circle.fill")
                    .font(.system(size: 50))
                    .foregroundColor(.accentColor)
                
                Text("Sign In")
                    .font(.title)
                    .fontWeight(.bold)
            }
            
            VStack(spacing: 16) {
                TextField("Email", text: $viewModel.email)
                    .textFieldStyle(.roundedBorder)
                
                SecureField("Password", text: $viewModel.password)
                    .textFieldStyle(.roundedBorder)
            }
            .padding(.horizontal, 40)
            
            if let error = viewModel.errorMessage {
                Text(error)
                    .font(.caption)
                    .foregroundColor(.red)
            }
            
            Button(action: {
                Task { await viewModel.signIn() }
            }) {
                if viewModel.isLoading {
                    ProgressView()
                } else {
                    Text("Sign In")
                        .frame(maxWidth: .infinity)
                }
            }
            .buttonStyle(.borderedProminent)
            .controlSize(.large)
            .disabled(!viewModel.canSignIn || viewModel.isLoading)
            .padding(.horizontal, 40)
            
            HStack {
                Text("Don't have an account?")
                Button("Sign Up") {
                    viewModel.navigateToSignUp()
                }
                .buttonStyle(.plain)
            }
            .font(.caption)
            
            Spacer()
        }
        .padding(.vertical)
        .frame(width: 500, height: 550)
    }
}
