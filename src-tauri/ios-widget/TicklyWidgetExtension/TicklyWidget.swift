import SwiftUI
import WidgetKit

@available(iOSApplicationExtension 17.0, *)
struct TicklyTimelineEntry: TimelineEntry {
    let date: Date
    let snapshot: WidgetSnapshot
    let selectedCategoryIdentifier: String?
}

@available(iOSApplicationExtension 17.0, *)
struct TicklyTimelineProvider: AppIntentTimelineProvider {
    typealias Intent = CategoryWidgetConfigurationIntent

    func placeholder(in context: Context) -> TicklyTimelineEntry {
        TicklyTimelineEntry(
            date: Date(),
            snapshot: WidgetSnapshot(
                generatedAt: "",
                totalCount: 7,
                pendingCount: 4,
                items: [],
                categories: [
                    WidgetCategorySummary(
                        categoryId: 1,
                        categoryName: "Essentials",
                        totalCount: 4,
                        pendingCount: 3,
                        firstPendingItemId: 101,
                        pendingItemIds: [101, 102, 103],
                        pendingItems: [
                            WidgetCategoryPendingItem(id: 101, text: "Wallet", displayOrder: 1000),
                            WidgetCategoryPendingItem(id: 102, text: "Keys", displayOrder: 2000),
                            WidgetCategoryPendingItem(id: 103, text: "Phone charger", displayOrder: 3000),
                        ]
                    ),
                    WidgetCategorySummary(
                        categoryId: 2,
                        categoryName: "Work",
                        totalCount: 3,
                        pendingCount: 1,
                        firstPendingItemId: 201,
                        pendingItemIds: [201],
                        pendingItems: [
                            WidgetCategoryPendingItem(id: 201, text: "Prepare report", displayOrder: 1000),
                        ]
                    ),
                ]
            ),
            selectedCategoryIdentifier: WidgetCategoryIdentifier.make(from: 1)
        )
    }

    func snapshot(
        for configuration: CategoryWidgetConfigurationIntent,
        in context: Context
    ) async -> TicklyTimelineEntry {
        makeEntry(configuration: configuration)
    }

    func timeline(
        for configuration: CategoryWidgetConfigurationIntent,
        in context: Context
    ) async -> Timeline<TicklyTimelineEntry> {
        let entry = makeEntry(configuration: configuration)
        let refreshDate = Calendar.current.date(byAdding: .minute, value: 15, to: Date())
            ?? Date().addingTimeInterval(900)
        return Timeline(entries: [entry], policy: .after(refreshDate))
    }

    private func makeEntry(configuration: CategoryWidgetConfigurationIntent) -> TicklyTimelineEntry {
        TicklyTimelineEntry(
            date: Date(),
            snapshot: WidgetSnapshotLoader.load(),
            selectedCategoryIdentifier: configuration.category?.id
        )
    }
}

@available(iOSApplicationExtension 17.0, *)
struct TicklyWidgetEntryView: View {
    @Environment(\.widgetFamily) private var family
    var entry: TicklyTimelineProvider.Entry

    private var selectedCategory: WidgetCategorySummary? {
        if let selectedCategoryIdentifier = entry.selectedCategoryIdentifier {
            if let category = entry.snapshot.categories.first(
                where: { WidgetCategoryIdentifier.make(from: $0.categoryId) == selectedCategoryIdentifier }
            ) {
                return category
            }
        }

        return entry.snapshot.categories.first
    }

    private var maxVisibleItems: Int {
        if family == .systemLarge {
            return 8
        }
        return 4
    }

    private var visiblePendingItems: [WidgetCategoryPendingItem] {
        guard let selectedCategory else {
            return []
        }

        return Array(selectedCategory.pendingItems.prefix(maxVisibleItems))
    }

    private var completionRate: Int {
        guard let selectedCategory, selectedCategory.totalCount > 0 else {
            return 0
        }

        let doneCount = selectedCategory.totalCount - selectedCategory.pendingCount
        let completion = Double(doneCount) / Double(selectedCategory.totalCount) * 100
        return Int(completion.rounded())
    }

    var body: some View {
        VStack(alignment: .leading, spacing: 10) {
            if let selectedCategory {
                header(category: selectedCategory)

                if visiblePendingItems.isEmpty {
                    Spacer()
                    Text("All done")
                        .font(.subheadline)
                        .foregroundColor(.secondary)
                    Spacer()
                } else {
                    ForEach(visiblePendingItems) { item in
                        pendingItemRow(item: item, categoryId: selectedCategory.categoryId)
                    }

                    if selectedCategory.pendingItems.count > maxVisibleItems {
                        Text("+\(selectedCategory.pendingItems.count - maxVisibleItems) more")
                            .font(.caption2)
                            .foregroundColor(.secondary)
                    }
                }
            } else {
                Spacer()
                Text("No categories")
                    .font(.subheadline)
                    .foregroundColor(.secondary)
                Spacer()
            }
        }
        .padding()
    }

    @ViewBuilder
    private func header(category: WidgetCategorySummary) -> some View {
        HStack(alignment: .firstTextBaseline) {
            VStack(alignment: .leading, spacing: 2) {
                Text(category.categoryName)
                    .font(.headline)
                    .lineLimit(1)
                Text("Category widget")
                    .font(.caption)
                    .foregroundColor(.secondary)
            }
            Spacer()
            VStack(alignment: .trailing, spacing: 2) {
                Text("\(category.pendingCount)/\(category.totalCount)")
                    .font(.headline)
                Text("\(completionRate)% done")
                    .font(.caption)
                    .foregroundColor(.secondary)
            }
        }
    }

    @ViewBuilder
    private func pendingItemRow(item: WidgetCategoryPendingItem, categoryId: Int64?) -> some View {
        HStack(spacing: 8) {
            Button(
                intent: ToggleTodoIntent(
                    itemId: Int(item.id),
                    categoryId: categoryId.map { Int($0) }
                )
            ) {
                Image(systemName: "circle")
                    .font(.subheadline)
                    .foregroundColor(.secondary)
            }
            .buttonStyle(.plain)

            Text(item.text)
                .font(.caption)
                .lineLimit(1)
                .minimumScaleFactor(0.75)

            Spacer(minLength: 0)
        }
    }
}

@available(iOSApplicationExtension 17.0, *)
struct TicklyWidget: Widget {
    let kind: String = "TicklyWidget"

    var body: some WidgetConfiguration {
        AppIntentConfiguration(
            kind: kind,
            intent: CategoryWidgetConfigurationIntent.self,
            provider: TicklyTimelineProvider()
        ) { entry in
            TicklyWidgetEntryView(entry: entry)
        }
        .configurationDisplayName("Tickly Category")
        .description("Show and check tasks in one category.")
        .supportedFamilies([.systemMedium, .systemLarge])
    }
}
