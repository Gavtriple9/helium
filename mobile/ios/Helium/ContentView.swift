import SwiftUI

struct ContentView: View {
    var body: some View {
        VStack(spacing: 16) {
            Text("Helium")
                .font(.title)
            Text("Rust library linked successfully (ping=\(helium_ffi_ping()))")
                .multilineTextAlignment(.center)
        }
        .padding()
    }
}

#Preview {
    ContentView()
}

