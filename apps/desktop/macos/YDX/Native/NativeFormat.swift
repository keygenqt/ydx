import Foundation
import YdxMacos

struct NativeFormat {
    nonisolated static func stringToVec(_ str: String, into data: inout Data) -> Vec_uint8_t {
        data = str.data(using: .utf8) ?? Data()
        let count = data.count
        
        return data.withUnsafeMutableBytes { bytes in
            let ptr = bytes.baseAddress?.assumingMemoryBound(to: UInt8.self)
            return Vec_uint8_t(
                ptr: ptr,
                len: count,
                cap: count
            )
        }
    }
    
    nonisolated static func vecToString(_ vec: Vec_uint8_t) -> String {
        let data = Data(bytes: vec.ptr, count: vec.len)
        return String(data: data, encoding: .utf8) ?? ""
    }
}
