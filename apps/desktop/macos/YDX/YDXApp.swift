import SwiftUI
import NavigatorUI
import Factory

@main
struct YDXApp: App {
    @StateObject private var appNavigator = Container.shared.appNavigator()
    @State private var isReady = false
    
    init() {
//        Container.shared.secureStorageService().clearAllKeychainData()
//        UserDefaults.standard.set(["en"], forKey: "AppleLanguages")
//        UserDefaults.standard.synchronize()
        
        NativeMethods.httpClientInit(
            baseURL: AppConstants.API.baseURL,
            uuid: AppConstants.API.uuid
        )
    }
    
    var body: some Scene {
        WindowGroup {
            if isReady {
                RootView()
                    .environmentObject(appNavigator)
                    .environment(\.navigator, appNavigator.navigator)
                    .frame(minWidth: CGFloat(AppConstants.DefaultWindow.width), minHeight: CGFloat(AppConstants.DefaultWindow.height))
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
        .windowResizability(.contentMinSize)
    }
}
