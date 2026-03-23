import SwiftUI
import Factory

struct HomePage: View {
    @State private var viewModel = Container.shared.homeViewModel()
    
    var body: some View {
        VStack(spacing: 20) {
            HStack {
                VStack(alignment: .leading, spacing: 4) {
                    Text("Home")
                        .font(.largeTitle)
                        .fontWeight(.bold)
                }
                
                Spacer()
                
                Button("Sign Out") {
                    Task { await viewModel.signOut() }
                }
                .buttonStyle(.bordered)
            }
            .padding()
            
            Spacer()
            
            VStack(spacing: 20) {
                Image(systemName: "house.fill")
                    .font(.system(size: 60))
                    .foregroundColor(.accentColor)
                
                Text("You are signed in!")
                    .font(.title2)
                    .fontWeight(.semibold)
                
                if let user = viewModel.user {
                    VStack(alignment: .leading, spacing: 8) {
                        InfoRow(title: "Email", value: user.email)
                        InfoRow(title: "ID", value: String(user.id))
                    }
                    .padding()
                    .background(Color.gray.opacity(0.1))
                    .cornerRadius(10)
                    .padding(.horizontal)
                }
            }
            
            Spacer()
        }
        .frame(minWidth: 600, minHeight: 400)
        .task {
            await viewModel.loadUser()
        }
        .overlay {
            if viewModel.isLoading {
                ProgressView()
            }
        }
    }
}
