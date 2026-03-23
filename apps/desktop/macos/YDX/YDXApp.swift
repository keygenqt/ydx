import SwiftUI
import NavigatorUI
import Factory

@main
struct YDXApp: App {
    @StateObject private var appNavigator = Container.shared.appNavigator()
    @State private var isReady = false
    
    var body: some Scene {
        WindowGroup {
            if isReady {
                RootView()
                    .environmentObject(appNavigator)
                    .environment(\.navigator, appNavigator.navigator)
                    .frame(minWidth: 800, minHeight: 600)
            } else {
                Color.clear
                    .frame(width: 0, height: 0)
                    .task {
                        if Container.shared.secureStorageService().hasValidToken() {
                            appNavigator.navigate(to: .splash)
                        } else {
                            appNavigator.navigate(to: .welcome)
                        }
                        isReady = true
                    }
            }
        }
        .windowResizability(.contentSize)
    }
}
