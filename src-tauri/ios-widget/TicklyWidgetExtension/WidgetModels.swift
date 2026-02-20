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

    enum CodingKeys: String, CodingKey {
        case id
        case text
        case displayOrder = "display_order"
    }
}

struct WidgetSnapshot: Codable {
    let generatedAt: String
    let totalCount: Int
    var pendingCount: Int
    var items: [WidgetTodoItem]
    var categories: [WidgetCategorySummary]

    enum CodingKeys: String, CodingKey {
        case generatedAt = "generated_at"
        case totalCount = "total_count"
        case pendingCount = "pending_count"
        case items
        case categories
    }

    init(
        generatedAt: String,
        totalCount: Int,
        pendingCount: Int,
        items: [WidgetTodoItem],
        categories: [WidgetCategorySummary]
    ) {
        self.generatedAt = generatedAt
        self.totalCount = totalCount
        self.pendingCount = pendingCount
        self.items = items
        self.categories = categories
    }

    init(from decoder: Decoder) throws {
        let container = try decoder.container(keyedBy: CodingKeys.self)
        generatedAt = try container.decode(String.self, forKey: .generatedAt)
        totalCount = try container.decode(Int.self, forKey: .totalCount)
        pendingCount = try container.decode(Int.self, forKey: .pendingCount)
        items = try container.decodeIfPresent([WidgetTodoItem].self, forKey: .items) ?? []
        categories = try container.decodeIfPresent([WidgetCategorySummary].self, forKey: .categories) ?? []
    }

    func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: CodingKeys.self)
        try container.encode(generatedAt, forKey: .generatedAt)
        try container.encode(totalCount, forKey: .totalCount)
        try container.encode(pendingCount, forKey: .pendingCount)
        try container.encode(items, forKey: .items)
        try container.encode(categories, forKey: .categories)
    }

    static let empty = WidgetSnapshot(
        generatedAt: "",
        totalCount: 0,
        pendingCount: 0,
        items: [],
        categories: []
    )
}
