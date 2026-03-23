import Foundation

struct TokenResponse: Codable {
    let userId: Int
    let accessToken: String
    let refreshToken: String
    let expiresAt: Int64
    let createdAt: Int64
    
    enum CodingKeys: String, CodingKey {
        case userId = "user_id"
        case accessToken = "access_token"
        case refreshToken = "refresh_token"
        case expiresAt = "expires_at"
        case createdAt = "created_at"
    }
    
    var expiresAtDate: Date {
        Date(timeIntervalSince1970: TimeInterval(expiresAt) / 1000)
    }
    
    var createdAtDate: Date {
        Date(timeIntervalSince1970: TimeInterval(createdAt) / 1000)
    }
}
