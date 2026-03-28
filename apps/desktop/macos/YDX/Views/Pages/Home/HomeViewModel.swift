import Foundation
import Factory
import NavigatorUI
import AppKit

@MainActor
@Observable
final class HomeViewModel {
    var dataLabel = "Click the button to fetch data"
    var isLoading = false
    var user: UserResponse?
    
    @ObservationIgnored
    @Injected(\.secureStorageService) private var secureStorageService
    
    @ObservationIgnored
    @Injected(\.appNavigator) private var nav
    
    func fetchData() async {
        isLoading = true
        defer { isLoading = false }
        
        // check if user is authenticated
        guard let userId = secureStorageService.getUserId(),
              let token = secureStorageService.getAccessToken() else {
            dataLabel = "User not found."
            return
        }
        
        // request user data in background
        let jsonString = await Task.detached(priority: .userInitiated) { [userId, token] in
            return NativeMethods.requestGetUser(id: userId, token: token)
        }.value
        
        // parse server response
        guard let jsonData = jsonString.data(using: .utf8),
              let user = try? JSONDecoder().decode(UserResponse.self, from: jsonData) else {
            dataLabel = "Failed to parse user data."
            return
        }
        
        // update ui with user data
        self.user = user
        dataLabel = "Email: \(user.email)"
    }
    
    func showAbout() {
        AboutDialog.show()
    }
    
    func signOut() async {
        secureStorageService.clearAll()
        nav.navigate(to: .welcome)
    }
}
