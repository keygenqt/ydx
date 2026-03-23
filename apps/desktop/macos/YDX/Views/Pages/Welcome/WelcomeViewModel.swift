import Foundation
import Factory
import NavigatorUI

@MainActor
@Observable
final class WelcomeViewModel {
    @ObservationIgnored
    @Injected(\.appNavigator) private var nav
    
    func navigateToSignIn() {
        nav.navigate(to: .signIn)
    }
    
    func navigateToSignUp() {
        nav.navigate(to: .signUp)
    }
}
