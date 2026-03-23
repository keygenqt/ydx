import SwiftUI

struct InfoRow: View {
    let title: String
    let value: String
    
    var body: some View {
        HStack {
            Text("\(title):")
                .fontWeight(.medium)
                .foregroundColor(.secondary)
            Text(value)
                .font(.body.monospaced())
        }
        .font(.caption)
    }
}
