import SwiftUI
import WidgetKit

struct TicklyTimelineEntry: TimelineEntry {
    let date: Date
    let snapshot: WidgetSnapshot
}

struct TicklyTimelineProvider: TimelineProvider {
    func placeholder(in context: Context) -> TicklyTimelineEntry {
        TicklyTimelineEntry(
            date: Date(),
            snapshot: WidgetSnapshot(
                generatedAt: "",
                totalCount: 3,
                pendingCount: 2,
                items: [
                    WidgetTodoItem(
                        id: 1,
                        text: "Wallet",
                        done: false,
                        categoryId: nil,
                        categoryName: nil,
                        displayOrder: 1000,
                        reminderAt: nil,
                        updatedAt: nil
                    ),
                    WidgetTodoItem(
                        id: 2,
                        text: "Keys",
                        done: false,
                        categoryId: nil,
                        categoryName: nil,
                        displayOrder: 2000,
                        reminderAt: nil,
                        updatedAt: nil
                    ),
                    WidgetTodoItem(
                        id: 3,
                        text: "Water Bottle",
                        done: true,
                        categoryId: nil,
                        categoryName: nil,
                        displayOrder: 3000,
                        reminderAt: nil,
                        updatedAt: nil
                    )
                ]
            )
        )
    }

    func getSnapshot(in context: Context, completion: @escaping (TicklyTimelineEntry) -> Void) {
        let entry = TicklyTimelineEntry(
            date: Date(),
            snapshot: WidgetSnapshotLoader.load()
        )
        completion(entry)
    }

    func getTimeline(in context: Context, completion: @escaping (Timeline<TicklyTimelineEntry>) -> Void) {
        let entry = TicklyTimelineEntry(
            date: Date(),
            snapshot: WidgetSnapshotLoader.load()
        )

        let refreshDate = Calendar.current.date(byAdding: .minute, value: 15, to: Date()) ?? Date().addingTimeInterval(900)
        completion(Timeline(entries: [entry], policy: .after(refreshDate)))
    }
}

struct TicklyWidgetEntryView: View {
    var entry: TicklyTimelineProvider.Entry

    var body: some View {
        VStack(alignment: .leading, spacing: 8) {
            HStack {
                Text("Tickly")
                    .font(.headline)
                Spacer()
                Text("\(entry.snapshot.pendingCount)")
                    .font(.headline)
                    .foregroundColor(.secondary)
            }

            if entry.snapshot.items.isEmpty {
                Spacer()
                Text("No items")
                    .font(.subheadline)
                    .foregroundColor(.secondary)
                Spacer()
            } else {
                ForEach(Array(entry.snapshot.items.prefix(4).enumerated()), id: \.element.id) { _, item in
                    Link(destination: toggleURL(itemId: item.id)) {
                        HStack(spacing: 8) {
                            Image(systemName: item.done ? "checkmark.circle.fill" : "circle")
                                .foregroundColor(item.done ? .green : .secondary)
                            Text(item.text)
                                .lineLimit(1)
                                .foregroundColor(.primary)
                        }
                    }
                }
                Spacer(minLength: 0)
            }
        }
        .padding()
    }

    private func toggleURL(itemId: Int64) -> URL {
        var components = URLComponents()
        components.scheme = "tickly"
        components.host = "widget"
        components.path = "/toggle"
        components.queryItems = [URLQueryItem(name: "itemId", value: String(itemId))]
        return components.url ?? URL(string: "tickly://widget")!
    }
}

struct TicklyWidget: Widget {
    let kind: String = "TicklyWidget"

    var body: some WidgetConfiguration {
        StaticConfiguration(kind: kind, provider: TicklyTimelineProvider()) { entry in
            TicklyWidgetEntryView(entry: entry)
        }
        .configurationDisplayName("Tickly")
        .description("Today checklist with quick check actions.")
        .supportedFamilies([.systemMedium])
    }
}
