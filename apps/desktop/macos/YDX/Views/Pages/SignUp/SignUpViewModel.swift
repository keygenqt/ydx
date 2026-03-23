import Foundation
import Factory
import NavigatorUI

@MainActor
@Observable
final class SignUpViewModel {
    var email = ""
    var username = ""
    var password = ""
    var confirmPassword = ""
    var isLoading = false
    var errorMessage: String?
    
    @ObservationIgnored
    @Injected(\.secureStorageService) private var secureStorageService
    
    @ObservationIgnored
    @Injected(\.appNavigator) private var nav
    
    var canSignUp: Bool {
        email.isValidEmail() &&
        !username.isEmpty &&
        password.isValidPassword() &&
        password == confirmPassword
    }
    
    func signUp() async {
        guard canSignUp else { return }
        
        isLoading = true
        defer { isLoading = false }
        
        // Simulate network request
        try? await Task.sleep(nanoseconds: 500_000_000)
        
        // TODO: Real registration
        // secureStorageService.setUserId(response.userId)
        // secureStorageService.setAccessToken(response.accessToken)
        // secureStorageService.setRefreshToken(response.refreshToken)
        
        nav.navigate(to: .home)
    }
    
    func navigateToSignIn() {
        nav.replace(to: .signIn)
    }
}
