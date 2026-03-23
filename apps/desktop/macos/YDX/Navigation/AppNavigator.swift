import SwiftUI
import Combine
import NavigatorUI

@MainActor
final class AppNavigator: ObservableObject {
    @Published var path = NavigationPath()
    @Published var root: AppDestination = .splash
    
    let navigator: Navigator
    
    init() {
        self.navigator = Navigator(configuration: .init())
    }
    
    func navigate(to destination: AppDestination) {
        switch destination {
        case .splash, .welcome, .home:
            root = destination
            path = NavigationPath()
        default:
            path.append(destination)
        }
    }
    
    func replace(to destination: AppDestination) {
        if !path.isEmpty {
            path.removeLast()
        }
        path.append(destination)
    }
    
    func pop() {
        if !path.isEmpty {
            path.removeLast()
        }
    }
}
