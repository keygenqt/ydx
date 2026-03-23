import Foundation
import Security

final class SecureStorageService: SecureStorageServiceProtocol {
    private let service = "com.ydx.secure"
    
    // MARK: - User ID
    func getUserId() -> String? {
        return getValue(key: "user_id")
    }
    
    func setUserId(_ value: String?) {
        if let value = value {
            saveValue(key: "user_id", value: value)
        } else {
            deleteValue(key: "user_id")
        }
    }
    
    // MARK: - Access Token
    func getAccessToken() -> String? {
        return getValue(key: "access_token")
    }
    
    func setAccessToken(_ value: String?) {
        if let value = value {
            saveValue(key: "access_token", value: value)
        } else {
            deleteValue(key: "access_token")
        }
    }
    
    // MARK: - Refresh Token
    func getRefreshToken() -> String? {
        return getValue(key: "refresh_token")
    }
    
    func setRefreshToken(_ value: String?) {
        if let value = value {
            saveValue(key: "refresh_token", value: value)
        } else {
            deleteValue(key: "refresh_token")
        }
    }
    
    // MARK: - Token Validation
    func hasValidToken() -> Bool {
        return getAccessToken() != nil
    }
    
    // MARK: - Clear All
    func clearAll() {
        deleteValue(key: "user_id")
        deleteValue(key: "access_token")
        deleteValue(key: "refresh_token")
    }
    
    // MARK: - Private Helpers
    private func getValue(key: String) -> String? {
        let query: [String: Any] = [
            kSecClass as String: kSecClassGenericPassword,
            kSecAttrService as String: service,
            kSecAttrAccount as String: key,
            kSecReturnData as String: true,
            kSecMatchLimit as String: kSecMatchLimitOne
        ]
        
        var result: AnyObject?
        let status = SecItemCopyMatching(query as CFDictionary, &result)
        
        guard status == errSecSuccess,
              let data = result as? Data,
              let value = String(data: data, encoding: .utf8) else {
            return nil
        }
        return value
    }
    
    private func saveValue(key: String, value: String) {
        guard let data = value.data(using: .utf8) else { return }
        
        let query: [String: Any] = [
            kSecClass as String: kSecClassGenericPassword,
            kSecAttrService as String: service,
            kSecAttrAccount as String: key,
            kSecValueData as String: data
        ]
        
        SecItemDelete(query as CFDictionary)
        SecItemAdd(query as CFDictionary, nil)
    }
    
    private func deleteValue(key: String) {
        let query: [String: Any] = [
            kSecClass as String: kSecClassGenericPassword,
            kSecAttrService as String: service,
            kSecAttrAccount as String: key
        ]
        
        SecItemDelete(query as CFDictionary)
    }
}
