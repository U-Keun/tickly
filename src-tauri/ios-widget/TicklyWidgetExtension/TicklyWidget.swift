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
                totalCount: 9,
                pendingCount: 5,
                items: [],
                categories: [
                    WidgetCategorySummary(
                        categoryId: 1,
                        categoryName: "Essentials",
                        totalCount: 3,
                        pendingCount: 2,
                        firstPendingItemId: 101,
                        pendingItemIds: [101, 102]
                    ),
                    WidgetCategorySummary(
                        categoryId: 2,
                        categoryName: "Work",
                        totalCount: 3,
                        pendingCount: 2,
                        firstPendingItemId: 202,
                        pendingItemIds: [202, 203]
                    ),
                    WidgetCategorySummary(
                        categoryId: 3,
                        categoryName: "Gym",
                        totalCount: 2,
                        pendingCount: 1,
                        firstPendingItemId: 303,
                        pendingItemIds: [303]
                    ),
                    WidgetCategorySummary(
                        categoryId: nil,
                        categoryName: "Uncategorized",
                        totalCount: 2,
                        pendingCount: 0,
                        firstPendingItemId: nil,
                        pendingItemIds: []
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
    @Environment(\.widgetFamily) private var family
    var entry: TicklyTimelineProvider.Entry

    private var categoryColumns: [GridItem] {
        if family == .systemLarge {
            return [GridItem(.flexible(), spacing: 8), GridItem(.flexible(), spacing: 8)]
        }
        return [GridItem(.flexible(), spacing: 8)]
    }

    private var completionRate: Int {
        guard entry.snapshot.totalCount > 0 else { return 0 }
        let doneCount = entry.snapshot.totalCount - entry.snapshot.pendingCount
        return Int((Double(doneCount) / Double(entry.snapshot.totalCount) * 100).rounded())
    }

    var body: some View {
        VStack(alignment: .leading, spacing: 10) {
            HStack(alignment: .firstTextBaseline) {
                VStack(alignment: .leading, spacing: 2) {
                    Text("Tickly")
                        .font(.headline)
                    Text("By category")
                        .font(.caption)
                        .foregroundColor(.secondary)
                }
                Spacer()
                VStack(alignment: .trailing, spacing: 2) {
                    Text("\(entry.snapshot.pendingCount) left")
                        .font(.headline)
                    Text("\(completionRate)% done")
                        .font(.caption)
                        .foregroundColor(.secondary)
                }
            }

            if entry.snapshot.categories.isEmpty {
                Spacer()
                Text("No categories")
                    .font(.subheadline)
                    .foregroundColor(.secondary)
                Spacer()
            } else {
                LazyVGrid(columns: categoryColumns, alignment: .leading, spacing: 6) {
                    ForEach(entry.snapshot.categories) { category in
                        HStack(spacing: 6) {
                            if let pendingItemId = category.firstPendingItemId {
                                if #available(iOSApplicationExtension 17.0, *) {
                                    Button(
                                        intent: ToggleTodoIntent(
                                            itemId: Int(pendingItemId),
                                            categoryId: category.categoryId.map { Int($0) }
                                        )
                                    ) {
                                        Image(systemName: "circle")
                                            .font(.caption)
                                            .foregroundColor(.secondary)
                                    }
                                    .buttonStyle(.plain)
                                } else {
                                    Link(destination: toggleURL(itemId: pendingItemId)) {
                                        Image(systemName: "circle")
                                            .font(.caption)
                                            .foregroundColor(.secondary)
                                    }
                                }
                            } else {
                                Image(systemName: "checkmark.circle.fill")
                                    .font(.caption)
                                    .foregroundColor(.green)
                            }

                            Text(category.categoryName)
                                .font(.caption)
                                .lineLimit(1)
                                .minimumScaleFactor(0.7)

                            Spacer(minLength: 4)

                            Text("\(category.pendingCount)/\(category.totalCount)")
                                .font(.caption2.weight(.semibold))
                                .foregroundColor(category.pendingCount > 0 ? .primary : .secondary)
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
        .configurationDisplayName("Tickly Categories")
        .description("Check pending tasks by category.")
        .supportedFamilies([.systemMedium, .systemLarge])
    }
}
