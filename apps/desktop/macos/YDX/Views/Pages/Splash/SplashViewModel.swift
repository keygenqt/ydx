import Foundation
import Factory
import NavigatorUI
import YdxMacos

@MainActor
@Observable
final class SplashViewModel {
    @ObservationIgnored
    @Injected(\.secureStorageService) private var secureStorageService
    
    @ObservationIgnored
    @Injected(\.appNavigator) private var nav
    
    func checkAuthState() async {
        // for smooth animation
        try? await Task.sleep(nanoseconds: 800_000_000)
        
        // check if token exists locally
        guard let token = secureStorageService.getAccessToken() else {
            nav.navigate(to: .welcome)
            return
        }
        
        // validate token with server in background thread
        let jsonString = await Task.detached(priority: .userInitiated) { [token] in
            return NativeMethods.requestSignCheck(token: token)
        }.value
        
        // parse response and navigate based on result
        guard let jsonData = jsonString.data(using: .utf8),
              let response = try? JSONDecoder().decode(ApiResponse<EmptyData>.self, from: jsonData),
              response.code == 200 else {
            nav.navigate(to: .welcome)
            return
        }
        
        // open user page
        nav.navigate(to: .home)
    }
}
