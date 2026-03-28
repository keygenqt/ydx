import Foundation

struct EmptyData: Codable {}

struct ApiResponse<T: Codable>: Codable {
    let code: Int
    let message: String
    let data: T?
}
