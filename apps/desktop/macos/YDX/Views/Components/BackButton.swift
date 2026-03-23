import SwiftUI

struct BackButton: View {
    @EnvironmentObject private var appNavigator: AppNavigator
    
    var body: some View {
        Button(action: {
            appNavigator.pop()
        }) {
            HStack {
                Image(systemName: "chevron.left")
                Text("Back")
            }
        }
        .buttonStyle(.plain)
    }
}
