import Factory
import NavigatorUI

extension Container {
    var appNavigator: Factory<AppNavigator> {
        self { @MainActor in AppNavigator() }
            .singleton
    }
    
    var secureStorageService: Factory<SecureStorageServiceProtocol> {
        self { @MainActor in SecureStorageService() }
            .singleton
    }
    
    var splashViewModel: Factory<SplashViewModel> {
        self { @MainActor in SplashViewModel() }
            .singleton
    }
    
    var welcomeViewModel: Factory<WelcomeViewModel> {
        self { @MainActor in WelcomeViewModel() }
            .singleton
    }
    
    var signInViewModel: Factory<SignInViewModel> {
        self { @MainActor in SignInViewModel() }
            .singleton
    }
    
    var signUpViewModel: Factory<SignUpViewModel> {
        self { @MainActor in SignUpViewModel() }
            .singleton
    }
    
    var homeViewModel: Factory<HomeViewModel> {
        self { @MainActor in HomeViewModel() }
            .singleton
    }
}
