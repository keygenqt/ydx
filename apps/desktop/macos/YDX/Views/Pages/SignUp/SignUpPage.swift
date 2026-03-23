import SwiftUI
import Factory

struct SignUpPage: View {
    @State private var viewModel = Container.shared.signUpViewModel()
    
    var body: some View {
        VStack(spacing: 20) {
            BackButton()
                .frame(maxWidth: .infinity, alignment: .leading)
                .padding(.horizontal)
            
            VStack(spacing: 8) {
                Image(systemName: "person.badge.plus")
                    .font(.system(size: 50))
                    .foregroundColor(.accentColor)
                
                Text("Create Account")
                    .font(.title)
                    .fontWeight(.bold)
            }
            
            VStack(spacing: 16) {
                TextField("Email", text: $viewModel.email)
                    .textFieldStyle(.roundedBorder)
                
                TextField("Username", text: $viewModel.username)
                    .textFieldStyle(.roundedBorder)
                
                SecureField("Password", text: $viewModel.password)
                    .textFieldStyle(.roundedBorder)
                
                SecureField("Confirm Password", text: $viewModel.confirmPassword)
                    .textFieldStyle(.roundedBorder)
            }
            .padding(.horizontal, 40)
            
            if let error = viewModel.errorMessage {
                Text(error)
                    .font(.caption)
                    .foregroundColor(.red)
            }
            
            Button(action: {
                Task { await viewModel.signUp() }
            }) {
                if viewModel.isLoading {
                    ProgressView()
                } else {
                    Text("Create Account")
                        .frame(maxWidth: .infinity)
                }
            }
            .buttonStyle(.borderedProminent)
            .controlSize(.large)
            .disabled(!viewModel.canSignUp || viewModel.isLoading)
            .padding(.horizontal, 40)
            
            HStack {
                Text("Already have an account?")
                Button("Sign In") {
                    viewModel.navigateToSignIn()
                }
                .buttonStyle(.plain)
            }
            .font(.caption)
            
            Spacer()
        }
        .padding(.vertical)
        .frame(width: 500, height: 600)
    }
}
