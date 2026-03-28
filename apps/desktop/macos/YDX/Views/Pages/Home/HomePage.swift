import SwiftUI
import Factory

struct HomePage: View {
    @State private var viewModel = Container.shared.homeViewModel()
    @State private var showLogoutAlert = false
    
    var body: some View {
        VStack(spacing: 0) {
            // header
            HStack {
                Button {
                    viewModel.showAbout()
                } label: {
                    Image(systemName: "info.circle")
                        .font(.system(size: 18))
                }
                .buttonStyle(.plain)
                .help("About")
                
                Spacer()
                
                Button {
                    showLogoutAlert = true
                } label: {
                    Image(systemName: "rectangle.portrait.and.arrow.right")
                        .font(.system(size: 18))
                }
                .buttonStyle(.plain)
                .help("Logout")
            }
            .padding(.horizontal)
            .padding(.vertical, 12)
            .background(Color(NSColor.controlBackgroundColor))
            
            Spacer()
            
            // content
            VStack(spacing: 24) {
                Image(systemName: "house.fill")
                    .font(.system(size: 60))
                    .foregroundColor(.accentColor)
                
                Text("Welcome to YDX!")
                    .font(.title)
                    .fontWeight(.bold)
                
                Text(LocalizedStringKey(viewModel.dataLabel))
                    .font(.body)
                    .foregroundColor(.secondary)
                    .multilineTextAlignment(.center)
                    .frame(maxWidth: 400)
                
                Button("Fetch Data") {
                    Task { await viewModel.fetchData() }
                }
                .buttonStyle(.borderedProminent)
                .controlSize(.large)
            }
            
            Spacer()
        }
        .frame(minHeight: 400)
        .alert("Logout", isPresented: $showLogoutAlert) {
            Button("Cancel", role: .cancel) { }
            Button("Logout", role: .destructive) {
                Task { await viewModel.signOut() }
            }
        } message: {
            Text("Are you sure you want to logout?")
        }
        .overlay {
            if viewModel.isLoading {
                ProgressView()
            }
        }
    }
}
