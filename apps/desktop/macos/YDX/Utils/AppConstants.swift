import Foundation

enum AppConstants {
    static let dataVersion = 1
    
    enum DefaultWindow {
        static let width: Int = 950
        static let height: Int = 550
    }
    
    enum API {
        static let baseURL = "http://localhost:3002"
        static let uuid = "uuid-macos"
    }
    
    enum SecretKeys {
        static let userId = "user_id"
        static let accessToken = "access_token"
        static let refreshToken = "refresh_token"
    }
    
    enum SettingsKeys {
        static let windowWidth = "window-width"
        static let windowHeight = "window-height"
        static let version = "data-version"
    }
}
