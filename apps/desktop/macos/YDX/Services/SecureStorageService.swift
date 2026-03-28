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
    private func saveValue(key: String, value: String) {
        #if DEBUG
        // Use UserDefaults during development to avoid keychain prompts
        UserDefaults.standard.set(value, forKey: key)
        #else
        // Use Keychain for production
        guard let data = value.data(using: .utf8) else { return }
        
        let query: [String: Any] = [
            kSecClass as String: kSecClassGenericPassword,
            kSecAttrService as String: service,
            kSecAttrAccount as String: key
        ]
        
        let attributes: [String: Any] = [
            kSecValueData as String: data,
            kSecAttrAccessible as String: kSecAttrAccessibleAfterFirstUnlock
        ]
        
        let status = SecItemUpdate(query as CFDictionary, attributes as CFDictionary)
        
        if status == errSecItemNotFound {
            var addQuery = query
            addQuery[kSecValueData as String] = data
            addQuery[kSecAttrAccessible as String] = kSecAttrAccessibleAfterFirstUnlock
            SecItemAdd(addQuery as CFDictionary, nil)
        }
        #endif
    }
    
    private func getValue(key: String) -> String? {
        #if DEBUG
        // Use UserDefaults during development
        return UserDefaults.standard.string(forKey: key)
        #else
        // Use Keychain for production
        let query: [String: Any] = [
            kSecClass as String: kSecClassGenericPassword,
            kSecAttrService as String: service,
            kSecAttrAccount as String: key,
            kSecReturnData as String: true,
            kSecMatchLimit as String: kSecMatchLimitOne,
            kSecAttrAccessible as String: kSecAttrAccessibleAfterFirstUnlock
        ]
        
        var result: AnyObject?
        let status = SecItemCopyMatching(query as CFDictionary, &result)
        
        guard status == errSecSuccess,
              let data = result as? Data,
              let value = String(data: data, encoding: .utf8) else {
            return nil
        }
        return value
        #endif
    }
    
    private func deleteValue(key: String) {
        #if DEBUG
        // Use UserDefaults during development
        UserDefaults.standard.removeObject(forKey: key)
        #else
        // Use Keychain for production
        let query: [String: Any] = [
            kSecClass as String: kSecClassGenericPassword,
            kSecAttrService as String: service,
            kSecAttrAccount as String: key
        ]
        SecItemDelete(query as CFDictionary)
        #endif
    }
    
    // MARK: - Development Helper
    func clearAllKeychainData() {
        #if DEBUG
        let keys = ["user_id", "access_token", "refresh_token"]
        keys.forEach { UserDefaults.standard.removeObject(forKey: $0) }
        #else
        let query: [String: Any] = [
            kSecClass as String: kSecClassGenericPassword,
            kSecAttrService as String: service
        ]
        SecItemDelete(query as CFDictionary)
        #endif
    }
}
