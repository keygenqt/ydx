import Foundation

extension String {
    func isValidEmail() -> Bool {
        let pattern = "^[^\\s@]+@[^\\s@]+\\.[^\\s@]+$"
        let predicate = NSPredicate(format: "SELF MATCHES %@", pattern)
        return predicate.evaluate(with: self)
    }
    
    func isValidPassword() -> Bool {
        guard count >= 8 && count <= 16 else { return false }
        let pattern = "^(?=.*[a-z])(?=.*[A-Z])(?=.*\\d).{8,16}$"
        let predicate = NSPredicate(format: "SELF MATCHES %@", pattern)
        return predicate.evaluate(with: self)
    }
}
