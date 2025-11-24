// Sample data generation module
// This module contains functions to generate sample data for testing

use chrono::Utc;
use serde_json::json;

/// Generate sample e-commerce product documents
pub fn generate_products(count: usize) -> Vec<serde_json::Value> {
    let categories = vec!["Electronics", "Clothing", "Books", "Home", "Sports"];
    let product_names = vec![
        "Laptop", "Smartphone", "T-Shirt", "Jeans", "Novel",
        "Coffee Maker", "Basketball", "Headphones", "Watch", "Backpack"
    ];
    let tags_options = vec![
        vec!["new", "featured"],
        vec!["sale", "popular"],
        vec!["bestseller"],
        vec!["limited"],
        vec!["premium", "featured"],
    ];

    (0..count)
        .map(|i| {
            let category = categories[i % categories.len()];
            let product_name = product_names[i % product_names.len()];
            let tags = &tags_options[i % tags_options.len()];
            let now = Utc::now();

            json!({
                "product_id": format!("PROD-{:05}", i + 1),
                "name": format!("{} {}", product_name, i + 1),
                "category": category,
                "price": 10.0 + (i as f64 * 5.5) % 500.0,
                "stock": (i * 7) % 100,
                "description": format!("High quality {} for your needs", product_name.to_lowercase()),
                "tags": tags,
                "created_at": now.to_rfc3339(),
                "updated_at": now.to_rfc3339(),
            })
        })
        .collect()
}

/// Generate sample application log documents
pub fn generate_logs(count: usize) -> Vec<serde_json::Value> {
    let levels = vec!["INFO", "WARN", "ERROR", "DEBUG"];
    let services = vec!["api-gateway", "auth-service", "user-service", "payment-service"];
    let hosts = vec!["host-01", "host-02", "host-03"];
    let messages = vec![
        "Request processed successfully",
        "Database connection timeout",
        "User authentication failed",
        "Payment processed",
        "Cache miss for key",
        "Rate limit exceeded",
        "Service health check passed",
    ];

    (0..count)
        .map(|i| {
            let now = Utc::now() - chrono::Duration::minutes(i as i64);

            json!({
                "timestamp": now.to_rfc3339(),
                "level": levels[i % levels.len()],
                "message": format!("{} - Request #{}", messages[i % messages.len()], i + 1),
                "service": services[i % services.len()],
                "host": hosts[i % hosts.len()],
                "user_id": format!("user_{}", (i % 50) + 1),
                "request_id": format!("req-{:08x}", i),
                "duration_ms": (i * 13) % 5000,
            })
        })
        .collect()
}

/// Generate sample user analytics documents
pub fn generate_analytics(count: usize) -> Vec<serde_json::Value> {
    let event_types = vec!["page_view", "click", "purchase", "signup", "logout"];
    let pages = vec!["/home", "/products", "/cart", "/checkout", "/profile"];
    let referrers = vec!["google.com", "facebook.com", "direct", "twitter.com"];
    let devices = vec!["desktop", "mobile", "tablet"];
    let browsers = vec!["Chrome", "Firefox", "Safari", "Edge"];
    let countries = vec!["US", "UK", "JP", "DE", "FR"];
    let cities = vec!["New York", "London", "Tokyo", "Berlin", "Paris"];

    (0..count)
        .map(|i| {
            let now = Utc::now() - chrono::Duration::hours(i as i64);
            let session_base = i / 5;

            json!({
                "user_id": format!("user_{}", (i % 100) + 1),
                "session_id": format!("session_{:06x}", session_base),
                "event_type": event_types[i % event_types.len()],
                "page_url": pages[i % pages.len()],
                "referrer": referrers[i % referrers.len()],
                "device": devices[i % devices.len()],
                "browser": browsers[i % browsers.len()],
                "country": countries[i % countries.len()],
                "city": cities[i % cities.len()],
                "timestamp": now.to_rfc3339(),
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_products() {
        let products = generate_products(10);
        assert_eq!(products.len(), 10);
        assert!(products[0]["product_id"].is_string());
        assert!(products[0]["price"].is_f64());
    }

    #[test]
    fn test_generate_logs() {
        let logs = generate_logs(10);
        assert_eq!(logs.len(), 10);
        assert!(logs[0]["level"].is_string());
        assert!(logs[0]["timestamp"].is_string());
    }

    #[test]
    fn test_generate_analytics() {
        let analytics = generate_analytics(10);
        assert_eq!(analytics.len(), 10);
        assert!(analytics[0]["user_id"].is_string());
        assert!(analytics[0]["event_type"].is_string());
    }
}
