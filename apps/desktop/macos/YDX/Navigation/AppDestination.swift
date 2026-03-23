import SwiftUI
import NavigatorUI

enum AppDestination: Hashable {
    case splash
    case welcome
    case signIn
    case signUp
    case home
}

extension AppDestination: NavigationDestination {
    @ViewBuilder
    var body: some View {
        switch self {
        case .splash:
            SplashPage()
        case .welcome:
            WelcomePage()
        case .signIn:
            SignInPage()
        case .signUp:
            SignUpPage()
        case .home:
            HomePage()
        }
    }
}
