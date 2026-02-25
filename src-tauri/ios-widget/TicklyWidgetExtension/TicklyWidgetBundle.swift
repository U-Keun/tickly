import WidgetKit
import SwiftUI

@main
@available(iOSApplicationExtension 17.0, *)
struct TicklyWidgetBundle: WidgetBundle {
    var body: some Widget {
        TicklyWidget()
        TicklyLockScreenWidget()
    }
}
