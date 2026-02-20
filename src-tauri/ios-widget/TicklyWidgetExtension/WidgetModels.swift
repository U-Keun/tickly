import Foundation

struct WidgetTodoItem: Decodable, Identifiable {
    let id: Int64
    let text: String
    let done: Bool
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

struct WidgetSnapshot: Decodable {
    let generatedAt: String
    let totalCount: Int
    let pendingCount: Int
    let items: [WidgetTodoItem]

    enum CodingKeys: String, CodingKey {
        case generatedAt = "generated_at"
        case totalCount = "total_count"
        case pendingCount = "pending_count"
        case items
    }

    static let empty = WidgetSnapshot(
        generatedAt: "",
        totalCount: 0,
        pendingCount: 0,
        items: []
    )
}
