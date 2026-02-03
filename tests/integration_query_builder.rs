//! Integration tests for the Apicalypse query builder.

use igdb_atlas::QueryBuilder;

#[test]
fn test_empty_query_defaults_to_all_fields() {
    let query = QueryBuilder::new().build();
    assert!(
        query.contains("fields *;"),
        "Empty query should select all fields. Got: {}",
        query
    );
}

#[test]
fn test_single_field_selection() {
    let query = QueryBuilder::new().select(&["name"]).build();
    assert!(
        query.contains("fields name;"),
        "Should contain field. Got: {}",
        query
    );
}

#[test]
fn test_multiple_field_selection() {
    let query = QueryBuilder::new()
        .select(&["name", "rating", "summary"])
        .build();

    assert!(
        query.contains("fields name,rating,summary;"),
        "Got: {}",
        query
    );
}

#[test]
fn test_add_field_appends() {
    let query = QueryBuilder::new()
        .select(&["name"])
        .add_field("rating")
        .add_field("summary")
        .build();

    assert!(query.contains("name"), "Got: {}", query);
    assert!(query.contains("rating"), "Got: {}", query);
    assert!(query.contains("summary"), "Got: {}", query);
}

#[test]
fn test_search_clause() {
    let query = QueryBuilder::new().search("The Witcher 3").build();
    assert!(
        query.contains("search \"The Witcher 3\";"),
        "Should contain search term. Got: {}",
        query
    );
}

#[test]
fn test_single_where_clause() {
    let query = QueryBuilder::new().where_clause("rating > 80").build();

    assert!(
        query.contains("where rating > 80;"),
        "Should contain where clause. Got: {}",
        query
    );
}

#[test]
fn test_multiple_where_clauses_joined_with_and() {
    let query = QueryBuilder::new()
        .where_clause("rating > 80")
        .and_where("platforms = 48")
        .build();

    assert!(
        query.contains("where rating > 80 & platforms = 48;"),
        "Multiple where clauses should be AND'd. Got: {}",
        query
    );
}

#[test]
fn test_three_where_clauses() {
    let query = QueryBuilder::new()
        .where_clause("rating > 80")
        .and_where("platforms = 48")
        .and_where("genres = 12")
        .build();

    assert!(
        query.contains("where rating > 80 & platforms = 48 & genres = 12;"),
        "Three clauses should be AND'd. Got: {}",
        query
    );
}

#[test]
fn test_sort_descending() {
    let query = QueryBuilder::new().sort_by("rating", true).build();
    assert!(
        query.contains("sort rating desc;"),
        "Should sort descending. Got: {}",
        query
    );
}

#[test]
fn test_sort_ascending() {
    let query = QueryBuilder::new().sort_by("name", false).build();
    assert!(
        query.contains("sort name asc;"),
        "Should sort ascending. Got: {}",
        query
    );
}

#[test]
fn test_limit() {
    let query = QueryBuilder::new().limit(25).build();
    assert!(
        query.contains("limit 25;"),
        "Should contain limit. Got: {}",
        query
    );
}

#[test]
fn test_offset() {
    let query = QueryBuilder::new().offset(100).build();
    assert!(
        query.contains("offset 100;"),
        "Should contain offset. Got: {}",
        query
    );
}

#[test]
fn test_limit_and_offset_together() {
    let query = QueryBuilder::new().limit(10).offset(20).build();

    assert!(query.contains("limit 10;"), "Got: {}", query);
    assert!(query.contains("offset 20;"), "Got: {}", query);
}

#[test]
fn test_expand_all_subfields() {
    let query = QueryBuilder::new()
        .select(&["name"])
        .expand("platforms", &[])
        .build();

    assert!(
        query.contains("platforms.*"),
        "Empty sub_fields should expand all. Got: {}",
        query
    );
    // Should not have "platforms" as a bare field
    assert!(
        !query.contains("fields name,platforms;"),
        "Should replace bare parent with expansion. Got: {}",
        query
    );
}

#[test]
fn test_expand_specific_subfields_dot_notation() {
    let query = QueryBuilder::new()
        .select(&["name"])
        .expand("platforms", &["name", "abbreviation"])
        .build();

    assert!(
        query.contains("platforms.name"),
        "Should use dot notation. Got: {}",
        query
    );
    assert!(
        query.contains("platforms.abbreviation"),
        "Should use dot notation. Got: {}",
        query
    );
}

#[test]
fn test_expand_removes_parent_from_fields() {
    let query = QueryBuilder::new()
        .select(&["name", "platforms", "genres"])
        .expand("platforms", &["name"])
        .build();

    // "platforms" as a bare field should be replaced
    // Should have "platforms.name" instead
    assert!(query.contains("platforms.name"), "Got: {}", query);
    // The bare "platforms" should not appear adjacent to a comma
    // (it's fine as part of "platforms.name")
    let fields_section = query.split(';').next().unwrap();
    let fields: Vec<&str> = fields_section
        .trim_start_matches("fields ")
        .split(',')
        .collect();
    assert!(
        !fields.contains(&"platforms"),
        "Bare 'platforms' should be removed when expanded. Fields: {:?}",
        fields
    );
}

#[test]
fn test_multiple_expansions() {
    let query = QueryBuilder::new()
        .select(&["name"])
        .expand("platforms", &["name"])
        .expand("genres", &["name", "slug"])
        .build();

    assert!(query.contains("platforms.name"), "Got: {}", query);
    assert!(query.contains("genres.name"), "Got: {}", query);
    assert!(query.contains("genres.slug"), "Got: {}", query);
}

#[test]
fn test_expansion_with_star_fields() {
    let query = QueryBuilder::new()
        .select(&["*"])
        .expand("platforms", &["name"])
        .build();

    // Should have both * and the expansion
    assert!(query.contains("*"), "Got: {}", query);
    assert!(query.contains("platforms.name"), "Got: {}", query);
}

#[test]
fn test_expansion_only_no_explicit_fields() {
    let query = QueryBuilder::new()
        .expand("platforms", &["name", "abbreviation"])
        .build();

    assert!(query.contains("platforms.name"), "Got: {}", query);
    assert!(query.contains("platforms.abbreviation"), "Got: {}", query);
    assert!(
        !query.contains("fields *;"),
        "Should not have wildcard when expansions provide fields. Got: {}",
        query
    );
}

#[test]
fn test_deep_expansion() {
    let query = QueryBuilder::new()
        .select(&["name"])
        .expand("involved_companies", &["company", "developer", "publisher"])
        .build();

    assert!(
        query.contains("involved_companies.company"),
        "Got: {}",
        query
    );
    assert!(
        query.contains("involved_companies.developer"),
        "Got: {}",
        query
    );
}

#[test]
fn test_full_query_all_features() {
    let query = QueryBuilder::new()
        .select(&["name", "rating"])
        .search("Zelda")
        .where_clause("rating > 80")
        .and_where("genres != null")
        .sort_by("rating", true)
        .limit(10)
        .offset(5)
        .expand("platforms", &["name"])
        .build();

    assert!(query.contains("fields "), "Missing fields");
    assert!(query.contains("search \"Zelda\";"), "Missing search");
    assert!(
        query.contains("where rating > 80 & genres != null;"),
        "Missing where. Got: {}",
        query
    );
    assert!(query.contains("sort rating desc;"), "Missing sort");
    assert!(query.contains("limit 10;"), "Missing limit");
    assert!(query.contains("offset 5;"), "Missing offset");
    assert!(query.contains("platforms.name"), "Missing expansion");
}

#[test]
fn test_field_count() {
    let builder = QueryBuilder::new().select(&["name", "rating", "summary"]);
    assert_eq!(builder.field_count(), 3);
}

#[test]
fn test_field_count_empty() {
    let builder = QueryBuilder::new();
    assert_eq!(builder.field_count(), 0);
}

#[test]
fn test_has_search_false_by_default() {
    let builder = QueryBuilder::new();
    assert!(!builder.has_search());
}

#[test]
fn test_has_search_true_when_set() {
    let builder = QueryBuilder::new().search("test");
    assert!(builder.has_search());
}

#[test]
fn test_where_count() {
    let builder = QueryBuilder::new()
        .where_clause("a = 1")
        .and_where("b = 2")
        .and_where("c = 3");

    assert_eq!(builder.where_count(), 3);
}

#[test]
fn test_where_count_zero() {
    let builder = QueryBuilder::new();
    assert_eq!(builder.where_count(), 0);
}

#[test]
fn test_expansion_count() {
    let builder = QueryBuilder::new()
        .expand("platforms", &["name"])
        .expand("genres", &["name"]);

    assert_eq!(builder.expansion_count(), 2);
}

#[test]
fn test_builder_is_cloneable() {
    let original = QueryBuilder::new()
        .select(&["name", "rating"])
        .search("test")
        .limit(5);

    let cloned = original.clone();
    assert_eq!(original.build(), cloned.build());
}

#[test]
fn test_special_characters_in_search() {
    let query = QueryBuilder::new().search("Pokémon Sword & Shield").build();

    assert!(
        query.contains("search \"Pokémon Sword & Shield\";"),
        "Should handle special characters. Got: {}",
        query
    );
}

#[test]
fn test_in_clause_filter() {
    let query = QueryBuilder::new()
        .where_clause("platforms = (48, 49, 6)")
        .build();

    assert!(
        query.contains("where platforms = (48, 49, 6);"),
        "Should handle IN clause. Got: {}",
        query
    );
}

#[test]
fn test_not_filter() {
    let query = QueryBuilder::new().where_clause("genres != null").build();

    assert!(
        query.contains("where genres != null;"),
        "Should handle NOT filter. Got: {}",
        query
    );
}
