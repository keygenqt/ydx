import SwiftUI
import Factory

struct RootView: View {
    @EnvironmentObject private var appNavigator: AppNavigator
    
    var body: some View {
        NavigationStack(path: $appNavigator.path) {
            Group {
                switch appNavigator.root {
                case .welcome:
                    WelcomePage()
                case .home:
                    HomePage()
                case .splash:
                    SplashPage()
                default:
                    EmptyView()
                }
            }
            .navigationDestination(for: AppDestination.self) { destination in
                destination.body
            }
            .navigationBarBackButtonHidden(true)
        }
    }
}
