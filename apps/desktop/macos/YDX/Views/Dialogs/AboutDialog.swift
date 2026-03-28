import SwiftUI
import AppKit

struct AboutDialog {
    static func show() {
        let alert = NSAlert()
        alert.messageText = ""
        alert.informativeText = ""
        alert.alertStyle = .informational
        alert.addButton(withTitle: NSLocalizedString("Close", comment: ""))
        
        // Create SwiftUI view
        let aboutView = VStack(spacing: 16) {
            Image("Ydx")
                .resizable()
                .frame(width: 120, height: 120)
            
            Text("YDX")
                .font(.title)
                .fontWeight(.bold)
            
            Text("Version 0.0.1")
                .font(.caption)
                .foregroundColor(.secondary)
            
            Text("© 2026 KeyGenQt")
                .font(.caption2)
                .foregroundColor(.secondary)
        }
        .padding(.horizontal, 20)
        .padding(.vertical, 16)
        
        let hostingController = NSHostingController(rootView: aboutView)
        hostingController.view.frame.size = hostingController.view.fittingSize
        
        alert.accessoryView = hostingController.view
        
        if let window = NSApp.keyWindow {
            alert.beginSheetModal(for: window) { _ in }
        } else {
            alert.runModal()
        }
    }
}
