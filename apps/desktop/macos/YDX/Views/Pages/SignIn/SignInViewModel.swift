import Foundation
import Factory
import NavigatorUI

@MainActor
@Observable
final class SignInViewModel {
    var email = ""
    var password = ""
    var isLoading = false
    var errorMessage: String?
    
    @ObservationIgnored
    @Injected(\.secureStorageService) private var secureStorageService
    
    @ObservationIgnored
    @Injected(\.appNavigator) private var nav
    
    var canSignIn: Bool {
        email.isValidEmail() && password.isValidPassword()
    }
    
    func signIn() async {
        guard canSignIn else { return }
        
        isLoading = true
        defer { isLoading = false }
        
        // Simulate network request
        try? await Task.sleep(nanoseconds: 500_000_000)
        
        // TODO: Real authorization
        // secureStorageService.setUserId(response.userId)
        // secureStorageService.setAccessToken(response.accessToken)
        // secureStorageService.setRefreshToken(response.refreshToken)
        
        nav.navigate(to: .home)
    }
    
    func navigateToSignUp() {
        nav.replace(to: .signUp)
    }
}
