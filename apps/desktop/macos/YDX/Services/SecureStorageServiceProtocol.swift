import Foundation

protocol SecureStorageServiceProtocol {
    func getUserId() -> String?
    func setUserId(_ value: String?)
    func getAccessToken() -> String?
    func setAccessToken(_ value: String?)
    func getRefreshToken() -> String?
    func setRefreshToken(_ value: String?)
    func clearAll()
    func hasValidToken() -> Bool
    func clearAllKeychainData()
}
