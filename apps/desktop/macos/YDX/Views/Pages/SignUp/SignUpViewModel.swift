import Foundation
import Factory
import NavigatorUI
import YdxMacos

@MainActor
@Observable
final class SignUpViewModel {
    var email = ""
    var password = ""
    var isLoading = false
    var emailError: String?
    var passwordError: String?
    var didAttemptSubmit = false
    
    @ObservationIgnored
    @Injected(\.secureStorageService) private var secureStorageService
    
    @ObservationIgnored
    @Injected(\.appNavigator) private var nav
    
    func signUp() async {
        // validate fields after first submit attempt
        didAttemptSubmit = true
        validateFields()
        guard emailError == nil && passwordError == nil else { return }
        
        // show loading indicator
        isLoading = true
        defer { isLoading = false }
        
        // send registration request in background
        let jsonString = await Task.detached(priority: .userInitiated) { [email, password] in
            return NativeMethods.requestSignUp(email: email, password: password)
        }.value
        
        // parse server response
        guard let jsonData = jsonString.data(using: .utf8),
              let response = try? JSONDecoder().decode(ApiResponse<TokenResponse>.self, from: jsonData) else {
            emailError = "Failed to parse user data."
            return
        }
        
        if response.code == 200, let tokenData = response.data {
            // save credentials on success
            secureStorageService.setAccessToken(tokenData.accessToken)
            secureStorageService.setRefreshToken(tokenData.refreshToken)
            secureStorageService.setUserId(String(tokenData.userId))
            // navigate to home
            nav.navigate(to: .home)
        } else if response.code == 422 {
            // handle validation errors from server
            if let errorData = response.data {
                let errorJson = try? JSONSerialization.jsonObject(with: JSONEncoder().encode(errorData)) as? [String: Any]
                if let emailErr = errorJson?["email"] as? String {
                    emailError = emailErr
                }
                if let passwordErr = errorJson?["password"] as? String {
                    passwordError = passwordErr
                }
            }
        } else if response.code == 409 {
            // email already exists
            emailError = "User with this email already exists."
        } else {
            // generic error
            emailError = "Please verify your email."
            passwordError = "Please verify your password."
        }
    }
    
    func validateFields() {
        guard didAttemptSubmit else { return }
        
        if email.isEmpty {
            emailError = "Must not be not blank."
        } else if !email.isValidEmail() {
            emailError = "Please enter a valid email address."
        } else {
            emailError = nil
        }
        
        if password.isEmpty {
            passwordError = "Must not be not blank."
        } else if password.count < 8 || password.count > 16 {
            passwordError = "Password must be between 8 and 16 characters."
        } else if !password.isValidPassword() {
            passwordError = "Password needs uppercase, lowercase and number."
        } else {
            passwordError = nil
        }
    }
    
    func navigateToSignIn() {
        nav.replace(to: .signIn)
    }
}
