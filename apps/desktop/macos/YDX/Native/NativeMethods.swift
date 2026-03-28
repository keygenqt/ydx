import Foundation
import YdxMacos

internal class NativeMethods {
    nonisolated static func httpClientInit(baseURL: String, uuid: String) {
        var baseURLData = baseURL.data(using: .utf8) ?? Data()
        var uuidData = uuid.data(using: .utf8) ?? Data()
        let baseURLCount = baseURLData.count
        let uuidCount = uuidData.count
        
        baseURLData.withUnsafeMutableBytes { baseURLBytes in
            uuidData.withUnsafeMutableBytes { uuidBytes in
                var baseURLVec = Vec_uint8_t(
                    ptr: baseURLBytes.baseAddress?.assumingMemoryBound(to: UInt8.self),
                    len: baseURLCount,
                    cap: baseURLCount
                )
                var uuidVec = Vec_uint8_t(
                    ptr: uuidBytes.baseAddress?.assumingMemoryBound(to: UInt8.self),
                    len: uuidCount,
                    cap: uuidCount
                )
                ydx_macos_request_init(&baseURLVec, &uuidVec)
            }
        }
    }
    
    nonisolated static func requestSignIn(email: String, password: String) -> String {
        var emailData = email.data(using: .utf8) ?? Data()
        var passwordData = password.data(using: .utf8) ?? Data()
        let emailCount = emailData.count
        let passwordCount = passwordData.count
        
        let result = emailData.withUnsafeMutableBytes { emailBytes in
            let emailPtr = emailBytes.baseAddress?.assumingMemoryBound(to: UInt8.self)
            var emailVec = Vec_uint8_t(
                ptr: emailPtr,
                len: emailCount,
                cap: emailCount
            )
            
            return passwordData.withUnsafeMutableBytes { passwordBytes in
                let passwordPtr = passwordBytes.baseAddress?.assumingMemoryBound(to: UInt8.self)
                var passwordVec = Vec_uint8_t(
                    ptr: passwordPtr,
                    len: passwordCount,
                    cap: passwordCount
                )
                return ydx_macos_request_signin(&emailVec, &passwordVec)
            }
        }
        
        return NativeFormat.vecToString(result)
    }
    
    nonisolated static func requestSignUp(email: String, password: String) -> String {
        var emailData = email.data(using: .utf8) ?? Data()
        var passwordData = password.data(using: .utf8) ?? Data()
        let emailCount = emailData.count
        let passwordCount = passwordData.count
        
        let result = emailData.withUnsafeMutableBytes { emailBytes in
            let emailPtr = emailBytes.baseAddress?.assumingMemoryBound(to: UInt8.self)
            var emailVec = Vec_uint8_t(
                ptr: emailPtr,
                len: emailCount,
                cap: emailCount
            )
            
            return passwordData.withUnsafeMutableBytes { passwordBytes in
                let passwordPtr = passwordBytes.baseAddress?.assumingMemoryBound(to: UInt8.self)
                var passwordVec = Vec_uint8_t(
                    ptr: passwordPtr,
                    len: passwordCount,
                    cap: passwordCount
                )
                return ydx_macos_request_signup(&emailVec, &passwordVec)
            }
        }
        
        return NativeFormat.vecToString(result)
    }
    
    nonisolated static func requestSignCheck(token: String) -> String {
        var tokenData = token.data(using: .utf8) ?? Data()
        let tokenCount = tokenData.count
        
        let result = tokenData.withUnsafeMutableBytes { tokenBytes in
            let tokenPtr = tokenBytes.baseAddress?.assumingMemoryBound(to: UInt8.self)
            var tokenVec = Vec_uint8_t(
                ptr: tokenPtr,
                len: tokenCount,
                cap: tokenCount
            )
            return ydx_macos_request_signcheck(&tokenVec)
        }
        
        return NativeFormat.vecToString(result)
    }
    
    nonisolated static func requestGetUser(id: String, token: String) -> String {
        var idData = id.data(using: .utf8) ?? Data()
        var tokenData = token.data(using: .utf8) ?? Data()
        let idCount = idData.count
        let tokenCount = tokenData.count
        
        let result = idData.withUnsafeMutableBytes { idBytes in
            let idPtr = idBytes.baseAddress?.assumingMemoryBound(to: UInt8.self)
            var idVec = Vec_uint8_t(
                ptr: idPtr,
                len: idCount,
                cap: idCount
            )
            
            return tokenData.withUnsafeMutableBytes { tokenBytes in
                let tokenPtr = tokenBytes.baseAddress?.assumingMemoryBound(to: UInt8.self)
                var tokenVec = Vec_uint8_t(
                    ptr: tokenPtr,
                    len: tokenCount,
                    cap: tokenCount
                )
                return ydx_macos_request_get_user(&idVec, &tokenVec)
            }
        }
        
        return NativeFormat.vecToString(result)
    }
}
