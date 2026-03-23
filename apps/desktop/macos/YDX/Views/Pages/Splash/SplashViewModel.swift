import Foundation
import Factory
import NavigatorUI

@MainActor
@Observable
final class SplashViewModel {
    var currentMessage: String = "Checking authentication..."
    
    @ObservationIgnored
    @Injected(\.secureStorageService) private var secureStorageService
    
    @ObservationIgnored
    @Injected(\.appNavigator) private var nav
    
    func checkAuthState() async {
        currentMessage = "Validating token..."
        try? await Task.sleep(nanoseconds: 1_000_000_000)
        
        let token = secureStorageService.getAccessToken()
        
        currentMessage = "Connecting to server..."
        try? await Task.sleep(nanoseconds: 500_000_000)
        
        if token != nil {
            nav.navigate(to: .home)
        } else {
            nav.navigate(to: .welcome)
        }
    }
}
