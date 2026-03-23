import Foundation

struct UserResponse: Codable, Identifiable {
    let id: Int
    let email: String
    let icon: String?
    let fname: String?
    let lname: String?
}
