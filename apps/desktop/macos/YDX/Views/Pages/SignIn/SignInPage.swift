import SwiftUI
import Factory

struct SignInPage: View {
    @State private var viewModel = Container.shared.signInViewModel()
    
    var body: some View {
        VStack(spacing: 0) {
            BackButton()
                .frame(maxWidth: .infinity, alignment: .leading)
                .padding(.horizontal)
                .padding(.top)
            
            Spacer()
            
            VStack(spacing: 30) {
                VStack(spacing: 24) {
                    Image(systemName: "person")
                        .font(.system(size: 85))
                        .foregroundColor(.accentColor)
                        .frame(width: 100, height: 100)
                    
                    Text("👋 Welcome back")
                        .font(.title2)
                        .fontWeight(.semibold)
                }
                
                VStack(spacing: 16) {
                    VStack(alignment: .leading, spacing: 4) {
                        TextField("Email", text: $viewModel.email)
                            .textFieldStyle(.roundedBorder)
                            .textContentType(.emailAddress)
                            .onChange(of: viewModel.email) { _, _ in
                                viewModel.validateFields()
                            }
                        
                        if let error = viewModel.emailError {
                            Text(LocalizedStringKey(error))
                                .font(.caption)
                                .foregroundColor(.red)
                        }
                    }
                    
                    VStack(alignment: .leading, spacing: 4) {
                        SecureField("Password", text: $viewModel.password)
                            .textFieldStyle(.roundedBorder)
                            .textContentType(.password)
                            .onChange(of: viewModel.password) { _, _ in
                                viewModel.validateFields()
                            }
                        
                        if let error = viewModel.passwordError {
                            Text(LocalizedStringKey(error))
                                .font(.caption)
                                .foregroundColor(.red)
                        }
                    }
                    
                    Button {
                        Task { await viewModel.signIn() }
                    } label: {
                        HStack {
                            Spacer(minLength: 0)
                            if viewModel.isLoading {
                                ProgressView().controlSize(.small)
                            } else {
                                Text("Sign In")
                            }
                            Spacer(minLength: 0)
                        }
                    }
                    .buttonStyle(.borderedProminent)
                    .frame(width: 360)
                    .disabled(viewModel.isLoading)
                    .padding(.top, 4)
                }
                .frame(width: 360)
                
                HStack(spacing: 4) {
                    Text("Don't have an account?")
                        .foregroundColor(.secondary)
                    
                    Button("Sign Up") {
                        viewModel.navigateToSignUp()
                    }
                    .buttonStyle(.plain)
                }
                .font(.caption)
            }
            
            Spacer()
        }
    }
}
