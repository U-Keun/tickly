import Foundation

struct WidgetTodoItem: Codable, Identifiable {
    let id: Int64
    let text: String
    var done: Bool
    let categoryId: Int64?
    let categoryName: String?
    let displayOrder: Int64
    let reminderAt: String?
    let updatedAt: String?

    enum CodingKeys: String, CodingKey {
        case id
        case text
        case done
        case categoryId = "category_id"
        case categoryName = "category_name"
        case displayOrder = "display_order"
        case reminderAt = "reminder_at"
        case updatedAt = "updated_at"
    }
}

struct WidgetCategorySummary: Codable, Identifiable {
    let categoryId: Int64?
    let categoryName: String
    let totalCount: Int
    var pendingCount: Int
    var firstPendingItemId: Int64?
    var pendingItemIds: [Int64]
    var pendingItems: [WidgetCategoryPendingItem]

    var id: String {
        if let categoryId {
            return String(categoryId)
        }
        return "uncategorized"
    }

    enum CodingKeys: String, CodingKey {
        case categoryId = "category_id"
        case categoryName = "category_name"
        case totalCount = "total_count"
        case pendingCount = "pending_count"
        case firstPendingItemId = "first_pending_item_id"
        case pendingItemIds = "pending_item_ids"
        case pendingItems = "pending_items"
    }

    init(
        categoryId: Int64?,
        categoryName: String,
        totalCount: Int,
        pendingCount: Int,
        firstPendingItemId: Int64?,
        pendingItemIds: [Int64],
        pendingItems: [WidgetCategoryPendingItem]
    ) {
        self.categoryId = categoryId
        self.categoryName = categoryName
        self.totalCount = totalCount
        self.pendingCount = pendingCount
        self.firstPendingItemId = firstPendingItemId
        self.pendingItemIds = pendingItemIds
        self.pendingItems = pendingItems
    }

    init(from decoder: Decoder) throws {
        let container = try decoder.container(keyedBy: CodingKeys.self)
        categoryId = try container.decodeIfPresent(Int64.self, forKey: .categoryId)
        categoryName = try container.decode(String.self, forKey: .categoryName)
        totalCount = try container.decode(Int.self, forKey: .totalCount)
        pendingCount = try container.decode(Int.self, forKey: .pendingCount)
        firstPendingItemId = try container.decodeIfPresent(Int64.self, forKey: .firstPendingItemId)
        pendingItemIds = try container.decodeIfPresent([Int64].self, forKey: .pendingItemIds) ?? []
        pendingItems = try container.decodeIfPresent([WidgetCategoryPendingItem].self, forKey: .pendingItems) ?? []
    }

    func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: CodingKeys.self)
        try container.encodeIfPresent(categoryId, forKey: .categoryId)
        try container.encode(categoryName, forKey: .categoryName)
        try container.encode(totalCount, forKey: .totalCount)
        try container.encode(pendingCount, forKey: .pendingCount)
        try container.encodeIfPresent(firstPendingItemId, forKey: .firstPendingItemId)
        try container.encode(pendingItemIds, forKey: .pendingItemIds)
        try container.encode(pendingItems, forKey: .pendingItems)
    }
}

struct WidgetCategoryPendingItem: Codable, Identifiable {
    let id: Int64
    let text: String
    let displayOrder: Int64
    let tags: [String]

    enum CodingKeys: String, CodingKey {
        case id
        case text
        case displayOrder = "display_order"
        case tags
    }

    init(id: Int64, text: String, displayOrder: Int64, tags: [String] = []) {
        self.id = id
        self.text = text
        self.displayOrder = displayOrder
        self.tags = tags
    }

    init(from decoder: Decoder) throws {
        let container = try decoder.container(keyedBy: CodingKeys.self)
        id = try container.decode(Int64.self, forKey: .id)
        text = try container.decode(String.self, forKey: .text)
        displayOrder = try container.decode(Int64.self, forKey: .displayOrder)
        tags = try container.decodeIfPresent([String].self, forKey: .tags) ?? []
    }

    func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: CodingKeys.self)
        try container.encode(id, forKey: .id)
        try container.encode(text, forKey: .text)
        try container.encode(displayOrder, forKey: .displayOrder)
        try container.encode(tags, forKey: .tags)
    }
}

struct WidgetTheme: Codable {
    let paper: String
    let canvas: String
    let stroke: String
    let ink: String
    let inkMuted: String
    let accentSky: String
    let accentSkyStrong: String

    enum CodingKeys: String, CodingKey {
        case paper
        case canvas
        case stroke
        case ink
        case inkMuted = "ink_muted"
        case accentSky = "accent_sky"
        case accentSkyStrong = "accent_sky_strong"
    }

    static let defaultTheme = WidgetTheme(
        paper: "#f8f7f3",
        canvas: "#f2efe8",
        stroke: "#e2ded5",
        ink: "#5b5852",
        inkMuted: "#7a776f",
        accentSky: "#a8bddb",
        accentSkyStrong: "#8ea9cf"
    )
}

struct WidgetSnapshot: Codable {
    let generatedAt: String
    let totalCount: Int
    var pendingCount: Int
    var items: [WidgetTodoItem]
    var categories: [WidgetCategorySummary]
    var theme: WidgetTheme

    enum CodingKeys: String, CodingKey {
        case generatedAt = "generated_at"
        case totalCount = "total_count"
        case pendingCount = "pending_count"
        case items
        case categories
        case theme
    }

    init(
        generatedAt: String,
        totalCount: Int,
        pendingCount: Int,
        items: [WidgetTodoItem],
        categories: [WidgetCategorySummary],
        theme: WidgetTheme = .defaultTheme
    ) {
        self.generatedAt = generatedAt
        self.totalCount = totalCount
        self.pendingCount = pendingCount
        self.items = items
        self.categories = categories
        self.theme = theme
    }

    init(from decoder: Decoder) throws {
        let container = try decoder.container(keyedBy: CodingKeys.self)
        generatedAt = try container.decode(String.self, forKey: .generatedAt)
        totalCount = try container.decode(Int.self, forKey: .totalCount)
        pendingCount = try container.decode(Int.self, forKey: .pendingCount)
        items = try container.decodeIfPresent([WidgetTodoItem].self, forKey: .items) ?? []
        categories = try container.decodeIfPresent([WidgetCategorySummary].self, forKey: .categories) ?? []
        theme = try container.decodeIfPresent(WidgetTheme.self, forKey: .theme) ?? .defaultTheme
    }

    func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: CodingKeys.self)
        try container.encode(generatedAt, forKey: .generatedAt)
        try container.encode(totalCount, forKey: .totalCount)
        try container.encode(pendingCount, forKey: .pendingCount)
        try container.encode(items, forKey: .items)
        try container.encode(categories, forKey: .categories)
        try container.encode(theme, forKey: .theme)
    }

    static let empty = WidgetSnapshot(
        generatedAt: "",
        totalCount: 0,
        pendingCount: 0,
        items: [],
        categories: [],
        theme: .defaultTheme
    )
}
