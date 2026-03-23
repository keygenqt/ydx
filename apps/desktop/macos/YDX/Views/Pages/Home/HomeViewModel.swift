import Foundation
import Factory
import NavigatorUI

@MainActor
@Observable
final class HomeViewModel {
    var user: UserResponse?
    var isLoading = false
    
    @ObservationIgnored
    @Injected(\.secureStorageService) private var secureStorageService
    
    @ObservationIgnored
    @Injected(\.appNavigator) private var nav
    
    func loadUser() async {
        isLoading = true
        defer { isLoading = false }
        
        // TODO: Load user from Keychain or API
        try? await Task.sleep(nanoseconds: 1_000_000_000)
        
        // Temporary mock data
        user = UserResponse(
            id: 1,
            email: "user@example.com",
            icon: nil,
            fname: "John",
            lname: "Doe"
        )
    }
    
    func signOut() async {
        secureStorageService.clearAll()
        nav.navigate(to: .welcome)
    }
}
